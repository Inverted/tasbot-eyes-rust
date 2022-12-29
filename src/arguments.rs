use log::{LevelFilter, warn};
use once_cell::sync::OnceCell;
use crate::color::Color;

pub static ARGUMENTS: OnceCell<Arguments> = OnceCell::new();

pub struct Arguments{
    pub level : LevelFilter,
    pub skip_startup_animation: bool,
    pub playback_speed: f32, //factor

    pub max_blinks: u8,
    pub min_delay: u16, //ms
    pub max_delay: u16, //ms

    pub color_overwrite: bool,
    pub color_overwrite_all: bool,
    pub overwrite_color_default: Option<&'static Color>,
}

pub fn default_arguments() -> Arguments{
    warn!("Using default arguments!");

    Arguments{
        level: LevelFilter::Debug,
        skip_startup_animation: true,
        playback_speed: 1.0,

        max_blinks: 4,
        min_delay: 4000,
        max_delay: 6000,

        color_overwrite: false,
        color_overwrite_all: false,
        overwrite_color_default: None,
    }
}

pub fn read_arguments() -> Arguments{
    default_arguments()
}