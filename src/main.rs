use std::mem::transmute;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::arguments::{ARGUMENTS, default_arguments, read_arguments};
use crate::color::{DEFAULT_PALETTE, get_base_or_blink_color, get_random_color, GREEN};

use crate::file_operations::files_in_directory;
use crate::logging::CONSOLE_LOGGER;
use crate::renderer::{play_animation_from_path, Renderer};
use crate::renderer::console::ConsoleRendererSettings;
use crate::renderer::silent::SilentRendererSettings;
use crate::renderer::tasbot_eyes::run_test;
use crate::tasbot::{run_tasbot_eyes};

mod file_operations;
mod gif;
mod renderer;
mod logging;
mod tasbot;
mod color;
mod arguments;

//itertools
//cargo docs
//always auto derive debug, when implementing Display
//todo: subcommands with clap

/*
? is propagating
.expect is panicking with error message
 */

fn main() {

    run_test();


    /*
    let args = ARGUMENTS.get_or_init(||read_arguments());

    //Setup things
    setup_logger(args.level);

    //todo: create right renderer
    let cli: ConsoleRendererSettings = ConsoleRendererSettings {
        clear_console: false,
    };

    let silent: SilentRendererSettings = SilentRendererSettings{};

    //Run the eyes
    run_tasbot_eyes(cli);

     */
}

fn setup_logger(level: log::LevelFilter) {
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");
    log::set_max_level(level);
    println!("[APP] Set log level to {}", level);
}