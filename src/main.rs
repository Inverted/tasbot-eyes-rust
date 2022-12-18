use crate::gif::read_animation;

mod filesystem;
mod gif;

fn main() {
    let file = "animations/base.gif";
    //let file = "animations/testbot.gif";
    read_animation(file).expect("TODO: panic message");
}
