use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::Parser;
use log::{info, warn};
use once_cell::sync::OnceCell;

use crate::color::FALLBACK_COLOR;

///Globally accessible argument results. Never changes after initialized
pub static ARGUMENTS: OnceCell<Arguments> = OnceCell::new();

#[derive(Parser, PartialEq, Debug)]
#[clap(author = "R3tr0BoiDX aka Mirco Janisch", about = "Software, to control new TASBot's eyes")]
pub struct Arguments {
    //todo: besides not needing it any more, how could I do something like this? both arent working
    //#[clap(short='l', long, required=false, possible_values=&log::LevelFilter::variants())] //aint working
    //#[clap(short='l', long, required=false, possible_values=&["error", "warn", "info", "debug"])] //neither
    //Set the log level
    //pub log_level: String,

    #[clap(short = 'u', long)]
    ///Skip the startup animation
    pub skip_startup_animation: bool,

    #[clap(short = 's', long, required = false, default_value = "1")]
    ///Set playback speed. Needs to be bigger then 0
    pub playback_speed: f32, //factor

    #[clap(short = 'b', long, required = false, default_value = "4")]
    ///Set maximum count of blinks between animations
    pub max_blinks: u8,

    #[clap(short = 'm', long, required = false, default_value = "4000")]
    ///Set minimum milliseconds between blinks
    pub min_delay: u16, //ms

    #[clap(short = 'n', long, required = false, default_value = "6000")]
    ///Set maximum milliseconds between blinks
    pub max_delay: u16, //ms

    #[clap(short = 'c', long)]
    ///Use random color from palette for grayscale animations
    pub color_overwrite: bool,

    #[clap(short = 'a', long)]
    ///Use random color from palette for grayscale animations as well as the blinks and base
    pub color_overwrite_all: bool,

    //todo: how can I provide an example? both are not working
    //#[clap(short='o', long, required=false, long_example="FF0080")]
    //#[clap(short='o', long, required=false, example="FF0080")]
    #[clap(short = 'o', long, required = false)]
    ///Color in hex format that should be used for not colored animations, e.g. -o FF0080 for magenta
    pub default_color: Option<String>,

    #[clap(short = 'p', long, required = false)]
    ///The path to a color palette
    pub palette: Option<PathBuf>,

    #[clap(short = 'P', long, required = false)]
    ///The path to a playlist
    pub playlist: Option<PathBuf>,

    #[clap(short = 'C', long)]
    ///Continue with normal program flow after playlist
    pub continue_after_playlist: bool,

    #[clap(short = 'i', long, required = false, default_value = "8082")]
    ///Set the TCP port that is to use for receiving animations
    pub inject_port: u16,

    #[command(subcommand)]
    ///Which renderer to use
    pub renderer: RendererType,
}

#[derive(clap::Subcommand, PartialEq, Debug)]
pub enum RendererType {
    ///Render animations in current console
    Console {
        #[clap(short = 'c', long)]
        ///Clear the console after every frame
        clear: bool,
    },

    ///Render animations on an LED matrix
    Matrix {
        //strip_type: String, //todo: also provide a list

        ///Change GPIO data pin. Possible values are between 2 to 27
        pin: u8,

        ///Height of the matrix, must be at least 8
        width: usize,

        ///Width of the matrix, must be at least 28
        height: usize,

        #[clap(short = 'b', long, required = false)]
        ///Set maximum possible brightness
        brightness: Option<u8>,

        #[clap(short = 'f', long, required = false)] //todo: example value
        ///Set the frequency of the signal to the LEDs, usually like 800kHz
        target_freq: Option<u32>,

        #[clap(short = 'd', long, required = false)] //todo: set value name <DNA> to <channel> e.g.
        ///Set the DMA channel. Possible values are between 0 to 13. Default is 10
        dma: Option<u8>,

        #[clap(short = 'i', long)]
        ///Set the invert flag
        inverted: bool,

        #[clap(short = 'g', long)]
        ///Use gamma correction
        gamma_correction: bool,

        #[clap(short = 'G', long, required = false)]
        ///Gamma value for gamma correction
        gamma: f32,
        // Higher values will result in dimmer colors, lower values will be brighter. 1.0 is no correction.
    },

    ///Render animations on blastermak's LED matrix for TASBot
    TASBot {
        #[clap(short = 'd', long, required = false)]
        ///Change GPIO data pin. Possible values are between 2 to 27. Default is 10
        pin: Option<u8>,

        #[clap(short = 'b', long, required = false)]
        ///Set maximum possible brightness. Default is 4
        brightness: Option<u8>,

        #[clap(short = 'g', long)]
        ///Use gamma correction
        gamma_correction: bool,

        #[clap(short = 'G', long, required = false, default_value = "2.8")]
        ///Value for gamma correction
        gamma: f32,
        // Higher values will result in dimmer colors, lower values will be brighter. 1.0 is no correction.
    },

    ///Render no animation at all (for debugging or testing)
    Silent,
}

impl Display for Arguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&*format!("\t-Skip startup animation: {}\n", self.skip_startup_animation.to_string()));
        result.push_str(&*format!("\t-Playback speed: {}\n", self.playback_speed));
        result.push_str(&*format!("\t-Maximum count of blinks: {}\n", self.max_blinks));
        result.push_str(&*format!("\t-Minimum delay between blinks: {} ms\n", self.min_delay));
        result.push_str(&*format!("\t-Maximum delay between blinks: {} ms\n", self.max_delay));
        result.push_str(&*format!("\t-Overwrite colors of grayscale animations: {}\n", self.color_overwrite.to_string()));
        result.push_str(&*format!("\t-Overwrite colors of grayscale animations, base and blinks: {}\n", self.color_overwrite_all.to_string()));
        result.push_str(&*format!("\t-Color for base, blinks and grayscale animations: #{}\n", self.default_color.clone().unwrap_or(FALLBACK_COLOR.to_string())));
        result.push_str(&*format!("\t-Color palette for random colors: {}\n", self.palette.clone().unwrap_or(PathBuf::from("None")).display()));
        result.push_str(&*format!("\t-Playlist to play: {}\n", self.playlist.clone().unwrap_or(PathBuf::from("None")).display()));
        result.push_str(&*format!("\t-Continue with normal flow after playlist: {}\n", self.continue_after_playlist.to_string()));
        result.push_str(&*format!("\t-TCP port for receiving animations: {}", self.inject_port));

        write!(f, "{}", result)
    }
}

pub fn init_arguments() {
    let args = check_arguments(Arguments::parse());
    info!("Using those arguments:\n{}", args);

    ARGUMENTS.get_or_init(|| args);
}

//todo: move check of other arguments here as well

fn check_arguments(mut raw_args: Arguments) -> Arguments {
    if raw_args.playback_speed <= 0.0 {
        warn!("Playback speed can't be smaller then 0! Using 1.0");
        raw_args.playback_speed = 1.0;
    }

    if raw_args.max_delay < raw_args.min_delay {
        warn!("Maximum delay between blinks can't be smaller then minimum delay. Swapping them.");
        let temp = raw_args.min_delay;
        raw_args.min_delay = raw_args.max_delay;
        raw_args.max_delay = temp;
    }

    //Attempt to convert it. If possible, everything is good
    raw_args.default_color = match u32::from_str_radix(&raw_args.default_color.clone().unwrap_or(FALLBACK_COLOR.to_string()), 16) {
        Ok(_) => { raw_args.default_color } //nothing changes
        Err(e) => {
            warn!("Given color is not in a valid format. Using default color: {}", e.to_string());
            None
        }
    };

    raw_args
}

pub fn fallback_arguments() -> Arguments {
    Arguments {
        skip_startup_animation: true,
        playback_speed: (1.0),

        max_blinks: 4,
        min_delay: 4000,
        max_delay: 6000,

        color_overwrite: false,
        color_overwrite_all: false,
        default_color: None,
        palette: None,
        playlist: None,
        continue_after_playlist: false,
        inject_port: 8082,
        renderer: RendererType::Silent,
    }
}

#[cfg(test)]
mod tests{
    use crate::arguments::{Arguments, check_arguments, RendererType};

    #[test]
    fn test_check_arguments() {
        let mut raw_args = Arguments {
            skip_startup_animation: false,
            playback_speed: 0.0,
            max_blinks: 0,
            min_delay: 10,
            max_delay: 5,
            color_overwrite: false,
            color_overwrite_all: false,
            default_color: Some("ffffff".to_owned()),
            palette: None,
            playlist: None,
            continue_after_playlist: false,
            inject_port: 0,
            renderer: RendererType::Silent,
        };

        let expected_args = Arguments {
            skip_startup_animation: false,
            playback_speed: 1.0,
            max_blinks: 0,
            min_delay: 5,
            max_delay: 10,
            color_overwrite: false,
            color_overwrite_all: false,
            default_color: None,
            palette: None,
            playlist: None,
            continue_after_playlist: false,
            inject_port: 0,
            renderer: RendererType::Silent,
        };

        let result = check_arguments(raw_args);
        assert_ne!(expected_args, result);
    }

}