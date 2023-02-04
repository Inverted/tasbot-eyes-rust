use std::{io, time};
use std::fmt::{Display, Formatter, write};
use std::io::Write;
use std::os::unix::raw::time_t;

use colored::{ColoredString, Colorize};
use log::info;

use crate::gif::{Animation, Frame, pixel_is_black};
use crate::renderer::{Color, Renderer, sleep_frame_delay};

const FILLED_CHARACTERS: &str = "██";
const EMPTY_CHARACTERS: &str = "  ";

pub struct ConsoleRendererSettings {
    pub clear_console: bool,
}

impl Display for ConsoleRendererSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&*format!("\t-Clear console after every frame: {}", self.clear_console));

        write!(f, "{}", result)
    }
}

impl Renderer for ConsoleRendererSettings {
    fn play(&mut self, anim: Animation) {
        for frame in &anim.frames {
            show_frame(self, frame, None);
        }
    }

    fn play_colored(&mut self, anim: Animation, color: &Color) {
        let color = if anim.grayscale { Some(color) } else { None };

        for frame in &anim.frames {
            show_frame(self, frame, color);
        }
    }

    fn clear(&mut self) {
        clear_console();
    }

    fn print_config(&self) {
        info!("Start console renderer using those arguments:\n{}", self);
    }
}

fn show_frame(settings: &ConsoleRendererSettings, frame: &Frame, color: Option<&Color>) {
    //clear console
    if settings.clear_console {
        clear_console();
    }

    //render frame
    render_frame(frame, color);

    //sleep base on delay from gif
    sleep_frame_delay(frame);
}

fn render_frame(frame: &Frame, color: Option<&Color>) {
    for row in frame.pixels {
        for pixel in row {
            if !pixel_is_black(&pixel) {
                match color {
                    None => {
                        print!("{}", FILLED_CHARACTERS.truecolor(pixel.r, pixel.g, pixel.b));
                    }
                    Some(col) => {
                        print!("{}", FILLED_CHARACTERS.truecolor(col.r, col.g, col.b));
                    }
                }
            } else {
                print!("{EMPTY_CHARACTERS}");
            }
        }
        print!("\n"); //look up doku if we need to flush
        io::stdout().flush().unwrap();
        //io::stdout().write() could be faster then print
    }
    info!("Rendering okay")
}

fn clear_console() {
    print!("{}[2J", 27 as char);
}