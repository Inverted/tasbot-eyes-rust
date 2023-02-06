use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::path::PathBuf;

use colored::Colorize;
use log::{info, warn};
use once_cell::sync::OnceCell;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rs_ws281x::RawColor;
use thiserror::Error;

use crate::arguments::{ARGUMENTS, fallback_arguments};
use crate::file_operations::read_palette;

pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const PURPLE: Color = Color { r: 255, g: 0, b: 255 };

///The default color, grayscale animations get rendered with, when no overwrite color is set
pub const FALLBACK_COLOR: Color = WHITE;

///The default palette, that random colors can get chosen from
const DEFAULT_PALETTE: [Color; 6] = [RED, YELLOW, GREEN, CYAN, BLUE, PURPLE];

///Once the color palette is initialized, make it available globally
pub static COLOR_PALETTE: OnceCell<Vec<Color>> = OnceCell::new();

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("An JSON error occurred: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("An error occurred: {0}")]
    Other(String),
}

#[derive(Clone, Copy, PartialEq, Debug)]
///The structure that's used to store the color value of an pixel
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Convert the color to a hex integer
    ///
    /// # Output
    /// An `u32` color string
    pub fn to_hex(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Get a color from an hex integer
    ///
    /// # Input
    /// `hex`: The hex integer that represents a color
    ///
    /// # Output
    /// An `Color` structure based on the given `hex` color
    pub fn from_hex(hex: u32) -> Self {
        Color {
            r: (hex >> 16) as u8,
            g: (hex >> 8) as u8,
            b: hex as u8,
        }
    }

    /// Convert a given hex color string to an color
    ///
    /// # Input
    /// `hex_string`: The string that contains a hex color
    ///
    /// # Output
    /// A `Result<Color, String>`, indicating if the conversion was successfully. If so, a `Color` structure is
    /// wrapped in it. If not, a `String` is returned as error (to be printed).
    pub fn from_hex_string(hex_string: &str) -> Result<Color, String> {
        if hex_string.len() != 6 {
            return Err(format!("Hex string is to short! Must be 6 but is {}", hex_string.len()));
        }

        match u32::from_str_radix(hex_string, 16) {
            Ok(c) => {
                Ok(Color::from_hex(c))
            }
            Err(e) => {
                warn!("Can't parse given color: {}", e.to_string());
                return Err("Invalid hex string format".to_owned());
            }
        }
    }

    /// Convert the color to a `RawColor` that is used to set the LED color
    ///
    /// # Output
    /// A `RawColor`, that represent this color
    pub fn to_raw(self) -> RawColor {
        [
            self.b,
            self.g,
            self.r,
            0,
        ]
    }
}

impl Display for Color {
    ///Format the color as hex string
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Initialize the color palette `OnceCell<>` with either
/// * the given color from the argument
/// * or the default color, if no argument was given
pub fn init_color_palette(path: &Option<PathBuf>) {
    let mut pal: Vec<Color> = Vec::new();

    match path {
        None => {
            //Default palette
            pal = DEFAULT_PALETTE.to_vec();
        }
        Some(pal_path) => {
            //Read color palette
            match read_color_palette(&PathBuf::from(pal_path)) {
                Ok(p) => {
                    pal = p;
                }
                Err(e) => { warn!("{}", e.to_string()) }
            };
        }
    }

    COLOR_PALETTE.get_or_init(|| pal);
}

/// Read a given color palette file
///
/// # Input
/// A `PathBuf` to the color palette file
///
/// # Output
/// A `Result<Vec<Color>, ColorError>`
/// * The `Vec<Color>` is the list of colors, that are found in the color palette file
/// * A `ColorError` is thrown, when the file cannot be read or the palette is empty
pub fn read_color_palette(path: &PathBuf) -> Result<Vec<Color>, ColorError> {
    let mut palette: Vec<Color> = Vec::new();

    let pal = read_palette(path)?;
    for hex_color in pal.colors {
        match Color::from_hex_string(hex_color.as_str()) {
            Ok(c) => {
                info!("Added #{} to color palette", format!("{}", c).truecolor(c.r, c.g, c.b));
                palette.push(c);
            }
            Err(e) => warn!("Problem with reading color: {}", e.to_string())
        };
    }

    if palette.is_empty() {
        Err(ColorError::Other(String::from("No colors in file")))
    } else {
        Ok(palette)
    }
}

/// Get a random color from the current color palette
///
/// # Output
/// A randomly selected `Color` from the color palette
///
/// # Todo
/// Could be combined with `get_random_color()` using an `Option<>` for `colors: &Vec<Color>`
pub fn get_random_color_from_palette() -> Color {
    match COLOR_PALETTE.get() {
        None => {
            warn!("Can't get color palette! Reverting back to default palette!");
            get_random_color(&DEFAULT_PALETTE.to_vec())
        }
        Some(pal) => {
            get_random_color(pal)
        }
    }
}

/// Get a random color from a given color palette
///
/// # Input
/// A `Vec<Colo>` that's the color palette
///
/// # Output
/// A random `Color` from the given color palette
fn get_random_color(colors: &Vec<Color>) -> Color {
    let mut rng = thread_rng();
    match colors.choose(&mut rng) {
        None => {
            warn!("Using default color!");
            FALLBACK_COLOR
        }
        Some(color) => { color.clone() }
    }
}

/// Get the right color for the base and blink animations. Based on the given arguments,
/// a few possibilities are valid:
/// * use the color of the animation
/// * use the overwrite color
/// * use a random selected color
///
/// # Input
/// `use_rand_color`: Indicating, if a random color should be selected
///
/// # Output
/// The right `Color` that should be use to render
pub fn get_base_or_blink_color(use_rand_color: bool) -> Option<Color> {
    let default_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&default_args);

    //Convert given color
    let def_color = match u32::from_str_radix(&args.default_color.clone().unwrap_or(FALLBACK_COLOR.to_string()), 16) { //todo: improve this
        Ok(c) => { c }
        Err(_) => {
            warn!("Can't parse given color. Using default color");
            FALLBACK_COLOR.to_hex()
        }
    };

    //If default color set, use it, else keep the animations color
    let color = if def_color != FALLBACK_COLOR.to_hex() { Some(Color::from_hex(def_color)) } else { None };

    //However, also check, if a random color should be chosen. If not, use whatever the last line yielded
    if use_rand_color { Some(get_random_color_from_palette()) } else { color }
}

/// Calculate the gamma correction for a channel value
///
/// # Input
/// * `channel_value`: A `u8` that is the value of a color channel, e.g. R, G or B
/// * `gamma`: The value that's to be used as gamma value. 1.0 is no gamma correction
pub fn get_gamma_correction(channel_value: u8, gamma: f32) -> u8 {
    let mut g = gamma;
    if g < 0f32 { g = 0f32 }
    ((channel_value as f32 / u8::MAX as f32).powf(g) * u8::MAX as f32 + 0.5).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use rand::Rng;

    #[test]
    fn test_to_hex() {
        let color = Color { r: 255, g: 0, b: 0 };
        let hex = color.to_hex();
        assert_eq!(hex, 0xff0000);
    }

    #[test]
    fn test_from_hex() {
        let hex = 0xff0000;
        let color = Color::from_hex(hex);
        assert_eq!(color, Color { r: 255, g: 0, b: 0 });
    }

    #[test]
    fn test_from_hex_string() {
        let hex_string = "ff0000";
        let color = Color::from_hex_string(hex_string).unwrap();
        assert_eq!(color, Color { r: 255, g: 0, b: 0 });

        let hex_string = "fff";
        let result = Color::from_hex_string(hex_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_raw() {
        let color = Color { r: 255, g: 0, b: 0 };
        let raw = color.to_raw();
        assert_eq!(raw, [0, 0, 255, 0]);
    }

    #[test]
    fn test_read_color_palette() {
        let path = PathBuf::from("test_palette.json");
        let palette = read_color_palette(&path).unwrap();
        assert_eq!(palette.len(), 3);
        assert_eq!(palette[0], Color { r: 255, g: 0, b: 0 });
        assert_eq!(palette[1], Color { r: 0, g: 255, b: 0 });
        assert_eq!(palette[2], Color { r: 0, g: 0, b: 255 });
    }

    /*
    #[test]
    fn test_read_color_palette_with_error() {
        let path = PathBuf::from("invalid_palette.json");
        let result = read_color_palette(&path);
        assert!(result.is_err());
    }
     */

    fn mock_palette() -> Vec<Color> {
        vec![Color::from_hex_string("FF0000").unwrap(),
             Color::from_hex_string("00FF00").unwrap(),
             Color::from_hex_string("0000FF").unwrap(),
        ]
    }

    #[test]
    fn test_get_random_color_from_palette_with_palette() {
        let mock_palette = mock_palette();
        let color = get_random_color(&mock_palette);

        assert!(mock_palette.contains(&color));
    }

    #[test]
    fn test_get_random_color_from_palette_same_color_multiple_calls() {
        let mock_palette = mock_palette();

        let color_a = get_random_color(&mock_palette);
        let color_b = get_random_color(&mock_palette);

        assert!(mock_palette.contains(&color_a));
        assert!(mock_palette.contains(&color_b));
    }

    #[test]
    fn test_get_gamma_correction() {
        let channel_value = 128;
        let gamma = 2.2;

        let corrected_value = get_gamma_correction(channel_value, gamma);

        assert!(corrected_value > 0 && corrected_value < u8::MAX);
    }
}