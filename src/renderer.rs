pub mod silent;
pub mod console;

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use log::{info, warn};

use crate::gif::{Animation, GifError, read_animation};

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

pub trait Renderer {
    fn play(&self, anim: &Animation);
    //todo: no ask: animation should be consumed, ig
    fn play_colored(&self, anim: &Animation, color: Color);
}

pub fn play_animation_from_path<T: Renderer>(renderer: &T, path: PathBuf, color: Option<Color>) {
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
                    renderer.play_colored(&anim, color);
                }
            }


        }
        Err(err) => {
            warn!("Can't read ({}): {}", path.to_str().unwrap_or("Invalid path"), err.to_string());
        }
    }
}
