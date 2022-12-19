use colored::Colorize;
use crate::gif::read_animation;
use crate::renderer::Renderer;
use crate::renderer_console::ConsoleRenderer;

mod filesystem;
mod gif;
mod renderer;
mod renderer_console;

fn main() {
    //let file = "animations/base.gif";
    let file = "animations/colorful.gif";
    let anim = read_animation(file).expect("TODO: panic message");

    let rend: ConsoleRenderer = ConsoleRenderer {};
    ConsoleRenderer::play(&rend, &anim);
}
