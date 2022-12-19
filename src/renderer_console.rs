use std::{io, time};
use std::io::Write;
use std::os::unix::raw::time_t;
use colored::{ColoredString, Colorize};
use crate::gif::{Animation, Frame, pixel_is_black};
use crate::renderer::Renderer;

pub struct ConsoleRenderer {
    pub grayscale: bool,
}

impl Renderer for ConsoleRenderer{
    fn play(&self, anim: &Animation) {
        for frame in &anim.frames {
            render_frame(frame);
        }
    }
}

fn render_frame(frame: &Frame){
    //clear console
    print!("{}[2J", 27 as char);

    //render frame
    for row in frame.pixels {
        for pixel in row {
            if !pixel_is_black(&pixel) {
                print!("{}", "██".truecolor(pixel.r, pixel.g, pixel.b));
            } else {
                print!("  ");
            }
        }
        print!("\n");
        io::stdout().flush().unwrap();
        //todo: to flush or not flush?
    }

    //sleep base on delay from gif
    let ms = time::Duration::from_millis((frame.delay * 10) as u64);
    std::thread::sleep(ms);
}