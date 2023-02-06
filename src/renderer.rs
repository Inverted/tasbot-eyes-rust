use std::path::PathBuf;
use std::time;

use log::{info, warn};

use crate::color::Color;
use crate::gif::{Animation, Frame, read_animation};

///Renderer, that animates no animation at all (for debugging or testing)
pub mod silent;

///Renderer, that animates in current console
pub mod console;

///Renderer, that animates on [blastermak's LED matrix for TASBot](https://github.com/blastermak/tasbot_eyes_pcb)
pub mod tasbot_eyes;

///Renderer, that animates on an LED matrix
///
/// # ATTENTION!
/// This is more pseudocode then anything else! This is not tested and might not work!
pub mod led_matrix;

///The universial interface for all renderer
pub trait Renderer {
    ///Play the animation as it is
    fn play(&mut self, anim: Animation);

    ///Play the animation, but overwrite the color of the animation
    fn play_colored(&mut self, anim: Animation, color: &Color);

    ///Clear the renderer medium
    fn clear(&mut self);

    ///Print the configuration of the renderer
    fn print_config(&self);
}

/// Play an animation from a given path with a given color option
///
/// # Input
/// * `renderer`: The renderer that is to use to render the animation
/// * `path`: The `PathBuf` to the animation
/// * `color`: The `Option<Color>`, if the color should be overwritten and with which color
pub fn play_animation_from_path<T: Renderer>(renderer: &mut T, path: PathBuf, color: Option<Color>) {
    let anim = read_animation(&path);
    match anim {
        Ok(anim) => {
            match color {
                None => {
                    info!("Attempt to play ({})", path.to_str().unwrap_or("Invalid path"));
                    renderer.play(anim);
                }
                Some(color) => {
                    info!("Attempt to play ({}) with (#{}) as color overwrite", path.to_str().unwrap_or("Invalid path"), color);
                    renderer.play_colored(anim, &color);
                }
            }
        }

        Err(err) => {
            warn!("Can't read ({}): {}", path.to_str().unwrap_or("Invalid path"), err.to_string());
        }
    }
}

/// Sleep between the `Frame`s, based on the delay, thats was encoded with the frame
///
/// # Input
/// The current `Frame` with its delay
pub fn sleep_frame_delay(frame: &Frame) {
    let ms = time::Duration::from_millis((frame.delay * 10) as u64);
    info!("Sleeping for delay for {} ms", ms.as_millis());
    std::thread::sleep(ms);
}