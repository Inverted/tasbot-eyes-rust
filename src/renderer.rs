use std::path::{Path, PathBuf};
use log::warn;
use crate::gif::{Animation, read_animation};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

//todo: make renderer type subcommand like structure like git --> can you do such things with clap?

pub trait Renderer{
    fn play(&self, anim: &Animation); //todo: no ask: animation should be consumed, ig
    fn play_colored(&self, anim: &Animation, color: Color);
}

pub fn play_animation_from_path<T: Renderer>(renderer: &T, path: PathBuf){
    let startup_anim = read_animation(&path);
    match startup_anim {
        None => {
            warn!("Can't play ({})", path.to_str().expect("none yet, lol")) //todo no ask
        }
        Some(anim) => {
            renderer.play(&anim);
        }
    }
}