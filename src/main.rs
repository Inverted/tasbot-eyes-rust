use std::{env, thread, time};
use std::cell::{Cell, RefCell};
use std::mem::transmute;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use colored::Colorize;

use log::{error, info, LevelFilter, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rs_ws281x::{Controller, StripType, WS2811Error};
use rs_ws281x::StripType::Ws2812;

use crate::arguments::{ARGUMENTS, fallback_arguments, init_arguments, RendererType};
use crate::color::{DEFAULT_PALETTE, get_base_or_blink_color, get_random_color, GREEN};
use crate::file_operations::files_in_directory;
use crate::led::{build_controller, LEDHardwareConfig};
use crate::logging::CONSOLE_LOGGER;
use crate::network::start_recv_file_server;
use crate::renderer::{play_animation_from_path, Renderer};
use crate::renderer::console::ConsoleRendererSettings;
use crate::renderer::led_matrix::{get_led_matrix_config, LEDMatrixError, LEDMatrixRenderer};
use crate::renderer::silent::SilentRendererSettings;
use crate::renderer::tasbot_eyes::{get_tasbot_eye_config, SCREEN_HEIGHT, SCREEN_WIDTH, TASBotRendererSettings};
use crate::tasbot::run_eyes;

mod file_operations;
mod gif;
mod renderer;
mod logging;
mod tasbot;
mod color;
mod arguments;
mod led;
mod network;

//itertools
//cargo docs
//always auto derive debug, when implementing Display
//todo: subcommands with clap
//clear on exit
//cfg for arm not working

/*
? is propagating
.expect is panicking with error message
 */

pub const ENV_LOG_LEVEL: &str = "TASBOT_EYES_LOG_LEVEL";
pub const LOG_LEVEL_FALLBACK: &str = "trace";

fn main() {
    //Setup logger
    let log_level = env::var(ENV_LOG_LEVEL).unwrap_or(get_fallback_log_level());
    setup_logger(log_level);

    //Process arguments
    init_arguments();
    let running = Arc::new(AtomicBool::new(true));
    let fallback_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&fallback_args);

    //Setup things
    //setup_ctrlc(running.clone());

    //Setup queue and network thread
    let queue: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let queue_network = queue.clone();
    thread::spawn(move || {
        start_recv_file_server(queue_network);
    });

    //Check arguments and start with right renderer
    match &args.renderer {
        RendererType::Console {
            clear
        } => {
            let cli = ConsoleRendererSettings {
                clear_console: clear.clone(),
            };

            run_eyes(cli, queue.clone(), running);
        }

        RendererType::Matrix {
            //strip_type,
            pin,
            width,
            height,
            brightness,
            target_freq,
            dma,
            inverted
        } => {
            /*
            let strip_type = match strip_type.parse::<StripType>() {
                Ok(val) => { val }
                Err(_) => {
                    println!("[EXCEPT] Unrecognized strip type, reverting to WS2812");
                    "Ws2812".to_string()
                }
            };
            todo: pull library and implement `StripType: FromStr`
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
                let mut b = brightness.unwrap();

                if b > 255 {
                    warn!("Brightness given ({}) higher than 255. Gonna use 255", b);
                    b = 255;
                } else if b < 0 {
                    warn!("Brightness given ({}) smaller than 0. Gonna use 0", b);
                    b = 0;
                }

                bright = Some(b);
            }

            if target_freq.is_some() {
                if target_freq.unwrap() < 0 {
                    error!("Frequenz given ({}) can't be smaller then 0", target_freq.unwrap());
                    panic!()
                }
            }

            if dma.is_some() {
                let mut dma_channel = dma.unwrap();
                if dma_channel < 0 {
                    error!("DMA channel given ({}) smaller than 0", dma_channel);
                    panic!()
                }

                if dma_channel > 13 {
                    error!("DMA channel given ({}) bigger than 13", dma_channel);
                    panic!()
                }
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
                            let matrix = LEDMatrixRenderer{
                                controller,
                            };

                            run_eyes(matrix, queue.clone(),running);
                        }
                        Err(e) => {
                            error!("Can't build hardware controller: {}", e.to_string());
                            panic!();
                        }
                    }
                }
                Err(e) => {
                    error!("Can't create LED hardware config: {}", e.to_string());
                    panic!();
                }
            }
        }

        RendererType::TASBot {
            pin,
            brightness
        } => {
            let mut bright: Option<u8> = None;
            if brightness.is_some() {
                let mut b = brightness.unwrap();

                if b > 255 {
                    warn!("Brightness given ({}) higher than 255. Gonna use 255", b);
                    b = 255;
                } else if b < 0 {
                    warn!("Brightness given ({}) smaller than 0. Gonna use 0", b);
                    b = 0;
                }

                bright = Some(b);
            }


            if pin.is_some() {
                let p = pin.unwrap();
                if p > 27 || p < 2 {
                    error!("GPIO pin #{} doesnt exist or is not valid to use. Pin ID must be between 2 and 27", p);
                    panic!();
                }
            }

            match build_controller(get_tasbot_eye_config(*pin, bright)) {
                Ok(controller) => {
                    let tasbot_eyes = TASBotRendererSettings {
                        controller,
                    };

                    run_eyes(tasbot_eyes,queue.clone(), running);
                }
                Err(e) => {
                    error!("Can't build hardware controller: {}", e.to_string());
                }
            }

        }

        RendererType::Silent => {
            let silent = SilentRendererSettings{};
            run_eyes(silent, queue.clone(),running);
        }
    }
}

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

fn get_fallback_log_level() -> String{
    println!("{}", "Using the fallback log level!".red());
    LOG_LEVEL_FALLBACK.to_string()
}

//todo: this works badly
fn setup_ctrlc(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    }).expect("[EXCEPT] Error setting Ctrl-C handler");
}