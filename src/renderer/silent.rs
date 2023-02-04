use log::debug;

use crate::gif::Animation;
use crate::renderer::{Color, Renderer};

pub struct SilentRendererSettings {}

impl Renderer for SilentRendererSettings {
    fn play(&mut self, _anim: Animation) {
        debug!("Silent renderer")
    }

    fn play_colored(&mut self, _anim: Animation, _color: &Color) { debug!("Silent renderer with color overwrite") }

    fn clear(&mut self) { debug!("Clear console") }

    fn print_config(&self) { debug!("No config for silent renderer needed") }
}