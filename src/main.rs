use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use colored::Colorize;
use log::{info, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::filesystem::files_in_directory;
use crate::gif::{Animation, read_animation};
use crate::logging::CONSOLE_LOGGER;
use crate::renderer::{play_animation_from_path, Renderer};
use crate::renderer_console::ConsoleRendererSettings;
use crate::tasbot::get_blink_delay;

mod filesystem;
mod gif;
mod renderer;
mod renderer_console;
mod logging;
mod tasbot;

const BASE_PATH: &str = "./gifs/base.gif";
//todo: so, its a slice, but also the whole string??? whos the owner then?
//borrow the data from the global data section

const STARTUP_PATH: &str = "./gifs/startup.gif";
const OTHER_PATH: &str = "./gifs/others/";
const BLINK_PATH: &str = "./gifs/blinks/";

//itertools
//todo: ask: are memory leaks possible at all in rust? --> yes

fn main() {
    setup_logger();

    let path = Path::new("./gifs/");

    let mut files = files_in_directory(path).expect("Can't open directory");

    let rend: ConsoleRendererSettings = ConsoleRendererSettings {
        clear_console: false,
    };

    for file in files {
        info!("Play animation \"{}\"", file.to_str().expect("TODO: panic message"));
        let anim = read_animation(&file).expect("TODO: panic message");
        ConsoleRendererSettings::play(&rend, &anim);
    }
}

fn run_tasbot_eyes<T: Renderer>(renderer: T) {
    let skip_startup_animation = false; //todo no ask: argument

    //Startup sequence
    if !skip_startup_animation {
        startup(&renderer);
    }
    show_base(&renderer);
    do_blink_cycle(&renderer);
}

fn startup<T: Renderer>(renderer: &T) {
    let startup_anim_path = Path::new(STARTUP_PATH);
    play_animation_from_path(renderer, startup_anim_path.to_path_buf());
}

fn show_base<T: Renderer>(renderer: &T) {
    let max_blinks = 4; //todo: no ask: arguments
    let min_time_between_blinks = 4;

    //skip base, when no blinks at all
    if max_blinks != 0 && min_time_between_blinks != 0 {
        let base_path = Path::new(BASE_PATH);
        play_animation_from_path(renderer, base_path.to_path_buf());
    }
}

fn do_blink_cycle<T: Renderer>(renderer: &T) {
    //todo: all arguments
    let playback_speed: f32 = 1.0;
    let min_delay: u16 = 4000;
    let max_delay: u16 = 6000;
    let max_blinks: u8 = 4; //make sure to check >0

    let delay: u16 = get_blink_delay(min_delay, max_delay, playback_speed);
    thread::sleep(Duration::from_millis(delay as u64));

    for _ in 0..max_blinks {
        let blink_anims_path = Path::new(BLINK_PATH);
        let mut files = files_in_directory(&blink_anims_path);

        match files {
            None => { warn!("Can't read files in directory ({})", blink_anims_path.to_str().expect("Issue with converting path to string")); }
            Some(files) => {
                let mut rng = thread_rng();
                let random_blink = files.choose(&mut rng);
                match random_blink {
                    None => { warn!("Can't choose a random animation"); }
                    Some(path) => {
                        play_animation_from_path(renderer, path.to_path_buf()); //todo: lol, this is weird. &pathbuf to ... pathbuf?
                    }
                }
            }
        }
    }
}


fn setup_logger() {
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");
    log::set_max_level(log::LevelFilter::Info);
}