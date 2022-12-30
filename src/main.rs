use std::cell::RefCell;
use std::mem::transmute;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rs_ws281x::{Controller, WS2811Error};
use crate::arguments::{ARGUMENTS, default_arguments, read_arguments};
use crate::color::{DEFAULT_PALETTE, get_base_or_blink_color, get_random_color, GREEN};

use crate::file_operations::files_in_directory;
use crate::led::build_controller;
use crate::logging::CONSOLE_LOGGER;
use crate::renderer::{play_animation_from_path, Renderer};
use crate::renderer::console::ConsoleRendererSettings;
use crate::renderer::silent::SilentRendererSettings;
use crate::renderer::tasbot_eyes::{get_tasbot_eye_config, TASBotRendererSettings};
use crate::tasbot::{run_tasbot_eyes};

mod file_operations;
mod gif;
mod renderer;
mod logging;
mod tasbot;
mod color;
mod arguments;
mod led;

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

fn main() {
    let args = ARGUMENTS.get_or_init(||read_arguments());

    //Setup things
    setup_logger(args.level);

    //todo: create right renderer
    let cli: ConsoleRendererSettings = ConsoleRendererSettings {
        clear_console: false,
    };

    let silent: SilentRendererSettings = SilentRendererSettings{};
    
    match build_controller(get_tasbot_eye_config(18, 4)){
        Ok(controller) => {
            let tasbot_eyes: TASBotRendererSettings = TASBotRendererSettings{
                controller,
            };

            /*
            let celled_rend = Rc::new(tasbot_eyes);
            setup_ctrlc(celled_rend.clone());
             */

            //Run the eyes
            run_tasbot_eyes(tasbot_eyes);
        }
        Err(err) => {
            error!("Can't build controller, Error: {}", err.to_string());
        }
    }
}

fn setup_logger(level: log::LevelFilter) {
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");
    log::set_max_level(level);
    println!("[APP] Set log level to {}", level);
}

/*
fn setup_ctrlc<T: Renderer>(renderer: Rc<T>){
    ctrlc::set_handler(move || {
        renderer.deref().clear();
    }).expect("Error setting Ctrl-C handler");
}
 */