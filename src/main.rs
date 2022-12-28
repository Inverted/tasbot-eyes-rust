use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::file_operations::files_in_directory;
use crate::logging::CONSOLE_LOGGER;
use crate::renderer::{Color, play_animation_from_path, Renderer};
use crate::renderer::console::ConsoleRendererSettings;
use crate::renderer::silent::SilentRendererSettings;
use crate::tasbot::{get_blink_amount, get_blink_delay};

mod file_operations;
mod gif;
mod renderer;
mod logging;
mod tasbot;

const BASE_PATH: &str = "./gifs/base.gif";
const STARTUP_PATH: &str = "./gifs/startup.gif";
const OTHER_PATH: &str = "./gifs/others/";
const BLINK_PATH: &str = "./gifs/blinks/";

//itertools
//cargo docs
//always auto derive debug, when implementing Display
//todo: subcommands with clap

/*
? is propagating
.expect is panicking with error message
 */

fn main() {
    //todo: argument
    let level = log::LevelFilter::Debug;

    //Setup things
    setup_logger(level);

    //todo: create right renderer
    let cli: ConsoleRendererSettings = ConsoleRendererSettings {
        clear_console: false,
    };

    let silent: SilentRendererSettings = SilentRendererSettings{};

    //Run the eyes
    run_tasbot_eyes(silent);
}

fn run_tasbot_eyes<T: Renderer>(renderer: T) {
    let skip_startup_animation = false; //todo no ask: argument
    let color_overwrite = true;
    let color_overwrite_all = true;
    let overwrite_color_chosen = Color {
        r: 0,
        g: 255,
        b: 0,
    };

    //Startup sequence
    if !skip_startup_animation {
        //startup(&renderer);
    }

    let mut queue: Vec<PathBuf> = Vec::new();
    loop {
        let overwrite_color = if color_overwrite { Some(overwrite_color_chosen) } else { None };
        let overwrite_color_all = if color_overwrite && color_overwrite_all { Some(overwrite_color_chosen) } else { None };

        show_base(&renderer, overwrite_color_all);
        do_blink_cycle(&renderer, overwrite_color_all);
        show_next_animation(&renderer, &mut queue, overwrite_color);
    }
}

fn startup<T: Renderer>(renderer: &T) {
    info!("Play startup animation");
    let startup_anim_path = Path::new(STARTUP_PATH);
    play_animation_from_path(renderer, startup_anim_path.to_path_buf(), None);
    info!("Done playing startup animation");
}

fn show_base<T: Renderer>(renderer: &T, color: Option<Color>) {
    info!("Play base animation");
    let max_blinks = 4; //todo: no ask: arguments
    let min_time_between_blinks = 4;

    //skip base, when no blinks at all
    if max_blinks != 0 && min_time_between_blinks != 0 {
        let base_path = Path::new(BASE_PATH);
        play_animation_from_path(renderer, base_path.to_path_buf(), color);
    }
    info!("Done playing base animation");
}

fn do_blink_cycle<T: Renderer>(renderer: &T, color: Option<Color>) {
    info!("Enter blink cycle");
    //todo: all arguments
    let playback_speed: f32 = 1.0;
    let min_delay: u16 = 4000;
    let max_delay: u16 = 6000;
    let max_blinks: u8 = 4; //make sure to check >0

    let blink_amount = get_blink_amount(max_blinks);
    info!("Blinking {} time", blink_amount);

    let blink_anims_path = Path::new(BLINK_PATH);
    let files = files_in_directory(&blink_anims_path);

    blink_sleep(get_blink_delay(min_delay, max_delay, playback_speed) as u64);
    for _ in 0..blink_amount {
        match &files {
            Ok(files) => {
                let mut rng = thread_rng();
                let random_blink = files.choose(&mut rng);
                match random_blink {
                    None => { warn!("Can't choose a random animation"); }
                    Some(path) => {
                        play_animation_from_path(renderer, path.to_path_buf(), color);
                    }
                }
            }
            Err(err) => {
                warn!("Can't read files in directory ({}), Error: {}", blink_anims_path.to_str().unwrap_or("Invalid path"), err.to_string());
            }
        }
        blink_sleep(get_blink_delay(min_delay, max_delay, playback_speed) as u64);
    }
    info!("Exit blink cycle");
}

fn blink_sleep(delay: u64) {
    info!("Sleeping for blink for {} ms", delay);
    thread::sleep(Duration::from_millis(delay));
}

fn show_next_animation<T: Renderer>(renderer: &T, anim_queue: &mut Vec<PathBuf>, color: Option<Color>) {
    let path = anim_queue.pop();

    match path {
        None => {
            //Queue is empty, create a new one
            let other_path = Path::new(OTHER_PATH);
            let files = files_in_directory(other_path);
            match files {
                Ok(mut files) => {
                    let mut rng = thread_rng();
                    files.shuffle(&mut rng);

                    anim_queue.clear();
                    anim_queue.extend(files);

                    if anim_queue.len() > 0 {
                        info!("Created new queue");

                        //Recursive call itself, to actually show animation
                        show_next_animation(renderer, anim_queue, color);
                    } else {
                        let message = "Directory seems empty, please check!";
                        error!("{}", message);
                        panic!("{}", message)
                    }
                }
                Err(err) => {
                    error!("Can't read directory ({}): {}", other_path.to_str().unwrap_or("Invalid path"), err.to_string());
                    panic!("{}", err.to_string());
                }
            }
        }
        Some(path) => {
            //Queue is not empty, play animation
            play_animation_from_path(renderer, path, color);
        }
    }
}

fn setup_logger(level: log::LevelFilter) {
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");
    log::set_max_level(level);
    println!("[APP] Set log level to {}", level);
}