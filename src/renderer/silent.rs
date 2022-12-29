use log::{debug, info, warn};
use crate::gif::Animation;
use crate::renderer::{Color, Renderer};

pub struct SilentRendererSettings {}

impl Renderer for SilentRendererSettings {
    fn play(&self, anim: &Animation) {
        warn!("Silent renderer")
    }

    fn play_colored(&self, anim: &Animation, color: &Color) {
        warn!("Silent renderer with color overwrite")
    }
}