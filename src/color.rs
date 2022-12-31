use std::fmt::{Display, Formatter};

use log::warn;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::arguments;
use crate::arguments::{ARGUMENTS, default_arguments};

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const PURPLE: Color = Color { r: 255, g: 0, b: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

pub const DEFAULT_PALETTE: [Color; 6] = [RED, YELLOW, GREEN, CYAN, BLUE, PURPLE];

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

pub fn get_random_color(colors: &[Color]) -> Color {
    let mut rng = thread_rng();
    match colors.choose(&mut rng) {
        None => {
            warn!("Using default color!");
            Color {
                r: 255,
                g: 255,
                b: 255,
            }
        }
        Some(color) => { color.clone() }
    }
}

pub fn get_base_or_blink_color(use_ran_color: bool) -> Option<Color> {
    let default_args = default_arguments();
    let args = ARGUMENTS.get().unwrap_or(&default_args);

    //If default color set, use it, else keep the animations color
    let color = if args.default_color != 0x000000 { Some(hex_to_rgb(args.default_color)) } else { None };

    //However, also check, if a random color should be chosen. If not, use whatever the last line yielded
    if use_ran_color { Some(get_random_color(&DEFAULT_PALETTE)) } else { color }
}

fn hex_to_rgb(hex: u32) -> Color {
    Color {
        r: (hex >> 16) as u8,
        g: (hex >> 8) as u8,
        b: hex as u8,
    }
}