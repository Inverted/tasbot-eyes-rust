use clap::builder::Str;
use log::{LevelFilter, warn};
use once_cell::sync::OnceCell;
use clap::{Arg, Parser};
use clap::ValueHint;

use crate::color::Color;

pub static ARGUMENTS: OnceCell<Arguments> = OnceCell::new();

#[derive(Parser,Debug)]
#[clap(author="Mirco Janisch", about="Software, to control new TASBot's eyes, build by blastermak")]
pub struct Arguments{
    //See pub enum ValueHint
    //value_hint=ValueHint::CommandString


    #[clap(short='l', long, required=false)]
    ///Set the log level
    pub log_level: String,

    #[clap(short='u', long)]
    ///Skip the startup animation
    pub skip_startup_animation: bool,

    #[clap(short='s', long, required=false, val_names=["factor"])]
    ///Set playback speed. Needs to be bigger than 0
    pub playback_speed: f32, //factor

    #[clap(short='B', long, required=false)]
    ///Controls the blinks. Highest number that can be used within the pattern is 9.
    /// 1st: Maximum number of blinks between animations,
    /// 2nd: Minimum seconds between blinks,
    /// 3rd: Maximum seconds between blinks.
    /// Example: 4-4-6 (default):
    /// Maximum of 4 blinks between animations,
    /// 4 to 6 seconds between each blink
    pub blink_pattern: String,



    //todo: Outdated: replace with blink pattern
    #[clap(short='j', long, required=false)]
    pub max_blinks: u8,

    #[clap(short='q', long, required=false)]
    pub min_delay: u16, //ms

    #[clap(short='t', long, required=false)]
    pub max_delay: u16, //ms



    #[clap(short='c', long)]
    ///Use random color from palette for grayscale animations
    pub color_overwrite: bool,

    #[clap(short='a', long)]
    ///Use random color from palette for grayscale animations as well as the blinks and base
    pub color_overwrite_all: bool,

    #[clap(short='C', long, required=false)]
    ///Default color that should be used for not colored animations
    pub default_color: u32,
}

pub fn build_arguments(){
    let args = Arguments::parse();
    println!("{:?}", args);
}

pub fn default_arguments() -> Arguments{
    warn!("Using default arguments!");

    Arguments{
        log_level: LevelFilter::Debug.to_string(), //todo: to uppercase
        skip_startup_animation: true,
        playback_speed: 1.0,

        blink_pattern: "4-4-6".to_string(),
        max_blinks: 4,
        min_delay: 4000,
        max_delay: 6000,

        //todo: clean again
        color_overwrite: false,
        color_overwrite_all: false,
        default_color: 0x000000,
    }
}

pub fn read_arguments() -> Arguments{
    default_arguments()
}