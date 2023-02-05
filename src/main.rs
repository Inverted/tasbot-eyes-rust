use std::{env, thread};
use std::fmt::format;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;
use log::{error, info, LevelFilter, warn};
use rs_ws281x::StripType::Ws2812;

use crate::arguments::{ARGUMENTS, fallback_arguments, init_arguments, RendererType};
use crate::color::init_color_palette;
use crate::led::build_controller;
use crate::logging::CONSOLE_LOGGER;
use crate::network::start_recv_file_server;
use crate::renderer::console::ConsoleRendererSettings;
use crate::renderer::led_matrix::{get_led_matrix_config, LEDMatrixRenderer};
use crate::renderer::silent::SilentRendererSettings;
use crate::renderer::tasbot_eyes::{get_tasbot_eye_config, SCREEN_HEIGHT, SCREEN_WIDTH, TASBotRendererSettings};
use crate::tasbot::start_eyes;

/// Contain operations, that read or write to a permanent storage
mod file_operations;

/// Read and parse GIF data
mod gif;

/// Has the trait for all renderer and some common logic among them
mod renderer;

/// Contains a basic console logger
mod logging;

/// TASBot specific logic
mod tasbot;

/// The color structure and other conversion code
mod color;

/// Define and parse command line arguments
mod arguments;

/// Contains the hardware configuration structure and a builder for it
mod led;

/// Network code such as the animation injection thread are here
mod network;

//todo: cargo docs
//todo: clear on exit as argument or clear on exit fast
//todo: cfg for arm not working
//todo: reorder files, lol

/* Notes
? is propagating
.expect is panicking with error message

always auto derive debug, when implementing Display
 */

/// The environmental variable, that is checked for the log level to use. See [log::Level](https://docs.rs/log/latest/log/enum.Level.html)
pub const ENV_LOG_LEVEL: &str = "TASBOT_EYES_LOG_LEVEL";

/// The fallback log level, when the environmental variable cannot be found
pub const LOG_LEVEL_FALLBACK: &str = "warn";

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    //Setup logger
    let log_level = env::var(ENV_LOG_LEVEL).unwrap_or(get_fallback_log_level());
    setup_logger(log_level);

    //Process arguments
    init_arguments();
    let fallback_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&fallback_args);

    //Setup queue and network thread
    let queue: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let queue_network = queue.clone();
    thread::spawn(move || {
        start_recv_file_server(queue_network);
    });

    //Setup other stuff
    setup_sigint_handler(&running);
    init_color_palette(&args.palette);

    //Check arguments and start with right renderer
    match &args.renderer {

        RendererType::Console {
            clear
        } => {
            let cli = ConsoleRendererSettings {
                clear_console: clear.clone(),
            };

            start_eyes(cli, queue.clone(), running);
        }

        RendererType::Matrix {
            //strip_type,
            pin,
            width,
            height,
            brightness,
            target_freq,
            dma,
            inverted,
            gamma_correction,
            gamma,
        } => {
            /*
            let strip_type = match strip_type.parse::<StripType>() {
                Ok(val) => { val }
                Err(_) => {
                    println!("[EXCEPT] Unrecognized strip type, reverting to WS2812");
                    "Ws2812".to_string()
                }
            };
            todo: pull ws281x library and implement `StripType: FromStr`
             */

            if *pin > 27 || *pin < 2 {
                error!("GPIO pin {} doesnt exist or is not valid to use. Pin ID must be between 2 and 27", *pin);
                panic!();
            }

            if *width < SCREEN_WIDTH {
                error!("Width given ({}) is to small, must be at least {}", *width, SCREEN_WIDTH);
                panic!()
            }

            if *width < SCREEN_HEIGHT {
                error!("Height given ({}) is to small, must be at least {}", *height , SCREEN_HEIGHT);
                panic!()
            }

            let mut bright: Option<u8> = None;
            if brightness.is_some() {
                let b = brightness.unwrap();
                bright = Some(b);
            }

            if dma.is_some() {
                let dma_channel = dma.unwrap();
                if dma_channel > 13 {
                    error!("DMA channel given ({}) bigger than 13", dma_channel);
                    panic!()
                }
            }

            let mut g: f32 = *gamma;
            if g < 0f32 {
                warn!("Gamma value can't be smaller then 0! Setting it to 0");
                g = 0f32;
            }

            let config = get_led_matrix_config(
                Ws2812,
                *pin,
                *width,
                *height,
                bright,
                *target_freq,
                *dma,
                Some(*inverted),
            );

            match config {
                Ok(config) => {
                    match build_controller(config) {
                        Ok(controller) => {
                            let matrix = LEDMatrixRenderer {
                                controller,
                                gamma_correction: *gamma_correction,
                                gamma: g,
                            };

                            start_eyes(matrix, queue.clone(), running);
                        }
                        Err(e) => {
                            error!("Can't build hardware controller: {}", e.to_string());
                            panic!();
                        }
                    }
                }
                Err(e) => {
                    let message = format!("Can't create LED hardware config: {}", e.to_string());
                    error!("{}", message);
                    panic!("{}", message);
                }
            }
        }

        RendererType::TASBot {
            pin,
            brightness,
            gamma_correction,
            gamma,
        } => {
            let mut bright: Option<u8> = None;
            if brightness.is_some() {
                let b = brightness.unwrap();
                bright = Some(b);
            }

            if pin.is_some() {
                let p = pin.unwrap();
                if p > 27 || p < 2 {
                    error!("GPIO pin #{} doesnt exist or is not valid to use. Pin ID must be between 2 and 27", p);
                    panic!();
                }
            }

            let mut g: f32 = *gamma;
            if g < 0f32 {
                warn!("Gamma value can't be smaller then 0! Setting it to 0");
                g = 0f32;
            }

            match build_controller(get_tasbot_eye_config(*pin, bright)) {
                Ok(controller) => {
                    let tasbot_eyes = TASBotRendererSettings {
                        controller,
                        gamma_correction: *gamma_correction,
                        gamma: g,
                    };

                    start_eyes(tasbot_eyes, queue.clone(), running);
                }
                Err(e) => {
                    error!("Can't build hardware controller: {}", e.to_string());
                }
            }
        }

        RendererType::Silent => {
            let silent = SilentRendererSettings {};
            start_eyes(silent, queue.clone(), running);
        }
    }
}

///Setup the handler that gets called, when the process receives an SIGINT
///
/// # Input
/// * `running`: The global-like bool, that indicates if the program is running or not
///
/// # Todo
/// * Its planned, to pack the renderer in its own thread. Potentially also wrap the render in an
/// Arc<Mutex<>> as well and pass it besides the handle to the thread to this function as well.
/// When a SIGINT is received then, kill the thread, call renderer.clear() and then force exit.
fn setup_sigint_handler(running: &Arc<AtomicBool>) {
    let r = running.clone();
    match ctrlc::set_handler(move || {
        info!("Exit program");
        r.store(false, Ordering::SeqCst);
        //todo: renderer.clear();
        std::process::exit(0);
    }) {
        Ok(_) => {}
        Err(e) => {
            let message = format!("Failed to set the SIGINT handler: {}", e.to_string());
            error!("{}", message);
            panic!("{}", message);
        }
    };
}

/// Setup the console logger
///
/// # Input
/// * `level`: The log level as string. Have a look at [log::Level](https://docs.rs/log/latest/log/enum.Level.html)
/// for further information
fn setup_logger(level: String) {
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");

    let log_level = match level.parse::<LevelFilter>() {
        Ok(val) => { val }
        Err(_) => {
            println!("[EXCEPT] Unrecognized log level ({}), reverting to {}. Consider updating the environmental variable \"{}\" to a valid value",
                     level.to_uppercase(), LOG_LEVEL_FALLBACK.to_uppercase(), ENV_LOG_LEVEL);
            LevelFilter::Warn
        }
    };

    log::set_max_level(log_level);
    info!("Set log level to {}", log_level.to_string());
}

/// Get the fallback log level. This is a function, so it can be easier printed to the console,
/// that the fallback log level is used.
fn get_fallback_log_level() -> String {
    println!("{}", format!("Using the fallback log level! Set the \"{}\" environment variable to a valid (rust) log level!", ENV_LOG_LEVEL).red());
    LOG_LEVEL_FALLBACK.to_string()
}