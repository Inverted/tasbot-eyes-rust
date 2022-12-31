use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::mem::transmute;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rs_ws281x::{Controller, WS2811Error};

use crate::arguments::{ARGUMENTS, build_arguments, default_arguments, read_arguments};
use crate::color::{DEFAULT_PALETTE, get_base_or_blink_color, get_random_color, GREEN};
use crate::file_operations::files_in_directory;
use crate::led::build_controller;
use crate::logging::CONSOLE_LOGGER;
use crate::renderer::{play_animation_from_path, Renderer};
use crate::renderer::console::ConsoleRendererSettings;
use crate::renderer::silent::SilentRendererSettings;
use crate::renderer::tasbot_eyes::{get_tasbot_eye_config, TASBotRendererSettings};
use crate::tasbot::run_eyes;

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
    build_arguments();

    /*
    let args = ARGUMENTS.get_or_init(|| read_arguments());
    let running = Arc::new(AtomicBool::new(true));

    //Setup things
    //setup_logger(args.log_level);
    setup_ctrlc(running.clone());


    //todo: create right renderer

    match build_controller(get_tasbot_eye_config(Some(18), Some(4))) {
        Ok(controller) => {
            let tasbot_eyes: TASBotRendererSettings = TASBotRendererSettings {
                controller,
            };

            run_eyes(tasbot_eyes, running);
        }
        Err(err) => {
            error!("Can't build controller, Error: {}", err.to_string());
        }
    }

     */
}

fn setup_logger(level: log::LevelFilter) {
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");
    log::set_max_level(level);
    println!("[APP] Set log level to {}", level);
}

fn setup_ctrlc(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    }).expect("[EXCEPT] Error setting Ctrl-C handler");
}
