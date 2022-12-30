pub mod silent;
pub mod console;
pub mod tasbot_eyes;

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time;

use log::{info, warn};
use crate::color::Color;

use crate::gif::{Animation, Frame, GifError, read_animation};

pub trait Renderer {
    fn play(&mut self, anim: &Animation);
    //todo: no ask: animation should be consumed, ig
    fn play_colored(&self, anim: &Animation, color: &Color);
    fn clear(&mut self);
}

pub fn play_animation_from_path<T: Renderer>(renderer: &mut T, path: PathBuf, color: Option<&Color>) {
    let anim = read_animation(&path);
    match anim {
        Ok(anim) => {
            match color {
                None => {
                    info!("Attempt to play ({})", path.to_str().unwrap_or("Invalid path"));
                    renderer.play(&anim);
                }
                Some(color) => {
                    info!("Attempt to play ({}) with ({}) as color overwrite", path.to_str().unwrap_or("Invalid path"), color);
                    renderer.play_colored(&anim, &color);
                }
            }
        }

        Err(err) => {
            warn!("Can't read ({}): {}", path.to_str().unwrap_or("Invalid path"), err.to_string());
        }
    }
}

pub fn sleep_frame_delay(frame: &Frame) {
    let ms = time::Duration::from_millis((frame.delay * 10) as u64);
    info!("Sleeping for delay for {} ms", ms.as_millis());
    std::thread::sleep(ms);
}