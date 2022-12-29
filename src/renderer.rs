pub mod silent;
pub mod console;

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use log::{info, warn};
use crate::color::Color;

use crate::gif::{Animation, GifError, read_animation};

pub trait Renderer {
    fn play(&self, anim: &Animation);
    //todo: no ask: animation should be consumed, ig
    fn play_colored(&self, anim: &Animation, color: &Color);
}

pub fn play_animation_from_path<T: Renderer>(renderer: &T, path: PathBuf, color: Option<&Color>) {
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
