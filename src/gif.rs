use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::PathBuf;

use gif::DecodingError;

const HEIGHT: usize = 8;
const WIDTH: usize = 28;

pub struct Animation {
    pub frames: Vec<Frame>,
    pub grayscale: bool,
}

pub struct Frame {
    pub pixels: [[Pixel; WIDTH]; HEIGHT],
    pub delay: u16,
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pixel: R:{}, G:{}, B:{}, A:{}", self.r, self.g, self.b, self.a)
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in self.pixels {
            for pixel in row {
                result.push(if pixel_is_black(&pixel) { ' ' } else { 'x' });
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

pub fn read_animation(path: PathBuf) -> Option<Animation> {

    //Setup decoder
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    //Open and read file
    let file = File::open(path).ok()?; //todo: ask, is .ok? nice?
    let mut decoder = decoder.read_info(file).ok()?; //todo: and rather, whats a nice way to print out the issue

    //todo: ist this nice? .ok_or(Err("Error here?")) ?

    /*
    ideal would be if I could return Some() when everything is alright or print out a custom defined
    message with something like error()! and return None
     */

    //Interpret data
    let mut anim: Animation = Animation {
        frames: vec![],
        grayscale: true,
    };

    while let Some(raw_frame) = decoder.read_next_frame().ok()? {
        let frame = read_frame(raw_frame);

        //A single frame with color is worth enough to mark the entire animation as colorful
        if anim.grayscale {
            anim.grayscale = frame_is_grayscaled(&frame);
        }

        anim.frames.push(frame);
    }

    Some(anim)
}

fn read_frame(raw_frame: &gif::Frame) -> Frame {
    //Init empty struct
    let mut frame: Frame = Frame {
        pixels: [[Pixel { r: 0, g: 0, b: 0, a: 0 }; WIDTH]; HEIGHT],
        delay: raw_frame.delay,
    };

    //Read all pixels into 2D array
    //Range based loop is necessary, as the buffer provides each RGBA value in a sequenz
    //Meaning, given a frame has a total of 64 pixels (8x8), the buffer will have 256 entries
    //Thus were stepping through the buffer with the step size 4
    for i in (0..raw_frame.buffer.len()).step_by(4) {
        let pixel = Pixel {
            r: raw_frame.buffer[i],
            g: raw_frame.buffer[i + 1],
            b: raw_frame.buffer[i + 2],
            a: raw_frame.buffer[i + 3],
        };

        //Convert 1D index into 2D index
        let index = i / 4;
        let x = index / WIDTH;
        let y = index % WIDTH;

        //Set pixel in frame based on calculated index
        frame.pixels[x][y] = pixel;
    }

    frame
}


//todo: can this be done nicer? somehow with iterators?

fn frame_is_grayscaled(frame: &Frame) -> bool {
    let mut result = true;

    for row in frame.pixels {
        for pixel in row {
            if !pixel_is_greyscale(&pixel) {
                result = false;
                break;
            }
        }
        if !result { break; }
    }

    result
}

pub fn pixel_is_black(pixel: &Pixel) -> bool {
    pixel.r == 0 && pixel.g == 0 && pixel.b == 0
}

fn pixel_is_greyscale(pixel: &Pixel) -> bool {
    pixel.r == pixel.g && pixel.g == pixel.b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_animation() {
        // Test reading a grayscale animation
        let anim = read_animation(PathBuf::from("animations/base.gif")).unwrap();
        assert_eq!(anim.grayscale, true);
        assert_eq!(anim.frames.len(), 1);
        assert_eq!(frame_is_grayscaled(&anim.frames[0]), true);

        // Test reading a colorful animation
        let anim = read_animation(PathBuf::from("animations/testbot.gif")).unwrap();
        assert_eq!(anim.grayscale, false);
        assert_eq!(anim.frames.len(), 1);
        assert_eq!(frame_is_grayscaled(&anim.frames[0]), false);
    }

    #[test]
    fn test_read_frame() {
        // Read a frame from a GIF file
        let file = File::open("animations/gray.gif").unwrap();
        let mut decoder = gif::DecodeOptions::new().read_info(file).unwrap();
        let raw_frame = decoder.read_next_frame().unwrap().unwrap();

        // Test that the returned frame has the correct values
        let frame = read_frame(&raw_frame);
        assert_eq!(frame.delay, 0);
        //assert_eq!(frame.pixels, [[Pixel { r: 128, g: 128, b: 128, a: 255 }; WIDTH]; HEIGHT]); //not allowed, binary operation `==` cannot be applied
        //todo: ask, what to do here
    }


    #[test]
    fn test_frame_is_grayscaled() {
        // Test a grayscale frame
        let frame = Frame {
            pixels: [[Pixel { r: 128, g: 128, b: 128, a: 255 }; WIDTH]; HEIGHT],
            delay: 100,
        };
        assert_eq!(frame_is_grayscaled(&frame), true);

        // Test a colorful frame
        let frame = Frame {
            pixels: [[Pixel { r: 128, g: 128, b: 64, a: 255 }; WIDTH]; HEIGHT],
            delay: 100,
        };
        assert_eq!(frame_is_grayscaled(&frame), false);
    }

    #[test]
    fn test_pixel_is_greyscale() {
        // Test a grayscale pixel
        let pixel = Pixel { r: 128, g: 128, b: 128, a: 255 };
        assert_eq!(pixel_is_greyscale(&pixel), true);

        // Test a colorful pixel
        let pixel = Pixel { r: 128, g: 128, b: 64, a: 255 };
        assert_eq!(pixel_is_greyscale(&pixel), false);
    }

    #[test]
    fn test_pixel_is_black() {
        // Test a black pixel
        let pixel = Pixel { r: 0, g: 0, b: 0, a: 255 };
        assert_eq!(pixel_is_black(&pixel), true);

        // Test a non-black pixel
        let pixel = Pixel { r: 128, g: 128, b: 128, a: 255 };
        assert_eq!(pixel_is_black(&pixel), false);
    }

    #[test]
    fn test_display_pixel() {
        // Test formatting a pixel
        let pixel = Pixel { r: 128, g: 128, b: 128, a: 255 };
        let expected = "Pixel: R:128, G:128, B:128, A:255";
        assert_eq!(format!("{}", pixel), expected);
    }
}