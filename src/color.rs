use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

use log::{info, warn};
use once_cell::sync::OnceCell;
use rand::seq::SliceRandom;
use rand::thread_rng;
use thiserror::Error;

use crate::arguments;
use crate::arguments::{ARGUMENTS, fallback_arguments};

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const PURPLE: Color = Color { r: 255, g: 0, b: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

pub const DEFAULT_COLOR: Color = WHITE;
const DEFAULT_PALETTE: [Color; 6] = [RED, YELLOW, GREEN, CYAN, BLUE, PURPLE];

pub static COLOR_PALETTE: OnceCell<Vec<Color>> = OnceCell::new();

pub fn init_color_palette(path: Option<PathBuf>) {
    let mut pal: Vec<Color> = Vec::new();

    match path {
        None => {
            //Default palette
            pal = DEFAULT_PALETTE.to_vec();
        }
        Some(pal_path) => {
            //Read color palette
            match read_color_palette(PathBuf::from(pal_path)) {
                Ok(p) => {
                    pal = p;
                }
                Err(e) => { warn!("{}", e.to_string()) }
            };
        }
    }

    COLOR_PALETTE.get_or_init(|| pal);
}

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),
}


#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_hex(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    fn from_hex(hex: u32) -> Self {
        Color {
            r: (hex >> 16) as u8,
            g: (hex >> 8) as u8,
            b: hex as u8,
        }
    }

    pub fn from_hex_string(hex_string: &str) -> Result<Color, String> {
        if hex_string.len() != 6 {
            return Err(format!("Hex string is to short! Must be 6 but is {}", hex_string.len()));
        }

        match string_to_int(hex_string) {
            Ok(c) => {
                Ok(Color::from_hex(c))
            }
            Err(e) => {
                warn!("Can't parse given color: {}", e.to_string());
                return Err("Invalid hex string format".to_owned());
            }
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

fn get_random_color(colors: &Vec<Color>) -> Color {
    let mut rng = thread_rng();
    match colors.choose(&mut rng) {
        None => {
            warn!("Using default color!");
            DEFAULT_COLOR
        }
        Some(color) => { color.clone() }
    }
}

pub fn get_random_color_from_palette() -> Color{
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

pub fn get_base_or_blink_color(use_ran_color: bool) -> Option<Color> {
    let default_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&default_args);

    //Convert given color
    let def_color = match string_to_int(&args.default_color.clone().unwrap_or(DEFAULT_COLOR.to_string())) {
        Ok(c) => { c }
        Err(_) => {
            warn!("Can't parse given color. Using default color");
            DEFAULT_COLOR.to_hex()
        }
    };

    //If default color set, use it, else keep the animations color
    let color = if def_color != DEFAULT_COLOR.to_hex() { Some(Color::from_hex(def_color)) } else { None };

    //However, also check, if a random color should be chosen. If not, use whatever the last line yielded
    if use_ran_color { Some(get_random_color_from_palette()) } else { color }
}

pub fn read_color_palette(path: PathBuf) -> Result<Vec<Color>, ColorError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut pal: Vec<Color> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if &l[0..1] != ";" { //support for comments
                    match Color::from_hex_string(l.as_str()) {
                        Ok(c) => {
                            info!("Added {} to color palette", c);
                            pal.push(c);
                        }
                        Err(e) => warn!("Problem with reading color: {}", e.to_string())
                    };
                }
            }
            Err(e) => warn!("Problem with reading line: {}", e.to_string())
        }
    }
    Ok(pal)
}

fn string_to_int(hex_string: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(hex_string, 16)
}