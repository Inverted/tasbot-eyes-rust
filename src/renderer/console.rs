use std::io;
use std::fmt::{Display, Formatter};
use std::io::Write;

use colored::Colorize;
use log::info;

use crate::gif::{Animation, Frame, pixel_is_black};
use crate::renderer::{Color, Renderer, sleep_frame_delay};

///The character, which is printed for pixel that arent black
const FILLED_CHARACTERS: &str = "██";

///The character, which is printed for pixel that are black
const EMPTY_CHARACTERS: &str = "  ";

/// Configuration for the console renderer
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

/// Handle a frame that's to be rendered in the console
///
/// # Input
/// `settings`: The configuration that should be used for rendering
/// `frame`: The `Frame` that should be rendered
/// `color`: An optional color to overwrite the frames own color
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

/// Render the frame in the console
///
/// # Input
/// `frame`: The `Frame` that's gonna be rendered
/// `color`: An optional color to overwrite the frames own color
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
        print!("\n");
        io::stdout().flush().unwrap();
    }
    info!("Rendering okay")
}

///Clear the console by printing several empty lines.
fn clear_console() {
    print!("{}[2J", 27 as char);
}