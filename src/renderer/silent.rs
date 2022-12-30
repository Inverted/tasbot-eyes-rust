use log::{debug, info, warn};

use crate::gif::Animation;
use crate::renderer::{Color, Renderer};

pub struct SilentRendererSettings {}

impl Renderer for SilentRendererSettings {
    fn play(&mut self, anim: &Animation) {
        debug!("Silent renderer")
    }

    fn play_colored(&mut self, anim: &Animation, color: &Color) { debug!("Silent renderer with color overwrite") }

    fn clear(&mut self) { debug!("Clear console") }
}