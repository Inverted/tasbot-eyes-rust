use std::path::Path;

use colored::{Color, Colorize};
use log::{error, info, Metadata, warn};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::filesystem::files_in_directory;
use crate::gif::read_animation;
use crate::logging::CONSOLE_LOGGER;
use crate::renderer::Renderer;
use crate::renderer_console::ConsoleRenderer;

mod filesystem;
mod gif;
mod renderer;
mod renderer_console;
mod logging;

const BASE_PATH: &str = "./gifs/base.gif"; //todo: so, its a slice, but also the whole string??? whos the owner?
const STARTUP_PATH: &str = "./gifs/startup.gif";
const OTHER_PATH: &str = "./gifs/others/";
const BLINK_PATH: &str = "./gifs/blinks/";

fn main() {
    setup_logger();

    let path = Path::new("./gifs/");

    let mut files = files_in_directory(path).expect("Can't open directory");

    let rend: ConsoleRenderer = ConsoleRenderer {
        grayscale: false,
    };

    for file in files {
        info!("Play animation \"{}\"", file.to_str().expect(""));
        let anim = read_animation(file).expect("TODO: panic message");
        ConsoleRenderer::play(&rend, &anim);
    }
}

fn setup_logger(){
    log::set_logger(&CONSOLE_LOGGER).expect("[EXCEPT] Can't set logger");
    log::set_max_level(log::LevelFilter::Info);
}