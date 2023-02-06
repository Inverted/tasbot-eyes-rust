use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::PathBuf;

use log::info;
use thiserror::Error;

/// The required height of an animation
const HEIGHT: usize = 8;

/// The required width of an animation
const WIDTH: usize = 28;

#[derive(Error, Debug)]
pub enum GifError {
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("A decoder error occurred: {0}")]
    Decode(#[from] gif::DecodingError),
}

/// Structure that represents all the needed data from a GIF file
pub struct Animation {
    /// All frames the GIF animation contains
    pub frames: Vec<Frame>,

    /// Indicates of **all** pixel of the animation are grayscale.
    /// If theres just a single pixel at any frame, that is not grayscale,
    /// the entire `Animation` counts as not grayscale anymore
    pub grayscale: bool,
}

#[derive(Eq, PartialEq, Debug)]
///A single frame of an animation with its delay
pub struct Frame {
    /// 2D array of all pixel of that frame
    pub pixels: [[Pixel; WIDTH]; HEIGHT],

    /// The delay of the frame, meaning, the duration *this* frame is shown.
    /// GIF frame delay are a multiple of 10 ms
    pub delay: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
///A single pixel of an frame
pub struct Pixel {
    ///Red channel
    pub r: u8,

    ///Green channel
    pub g: u8,

    ///Blue channel
    pub b: u8,

    /// (Unused) alpha channel
    pub a: u8,
}

impl Display for Pixel {
    ///Format a pixel to be shown as it individual channel values
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pixel: R:{}, G:{}, B:{}, A:{}", self.r, self.g, self.b, self.a)
    }
}

impl Display for Frame {
    ///Format a frame to be rendered in black and white
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

/// Read an animation from the given `PathBuf`
///
/// # Input
/// The `PathBuf` where the animation is stored
///
/// # Output
/// A `Result<Animation, GifError>` with
/// * `Animation` being the successfully read animation
/// * `GifError` being thrown when:
///     - The file cannot be opened
///     - The file info cannot be read
///     - Any frame cannot be read
///
/// # Todo
/// Some of this code should be moved to the `file_operations` module
pub fn read_animation(path: &PathBuf) -> Result<Animation, GifError> {
    info!("Attempt to read ({})", path.to_str().unwrap_or("Invalid path"));

    //Setup decoder
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    //Open and read file
    let file = File::open(path)?;
    let mut decoder = decoder.read_info(file)?;

    //Setup structure
    let mut anim: Animation = Animation {
        frames: vec![],
        grayscale: true,
    };

    //Interpret data
    while let Some(raw_frame) = decoder.read_next_frame()? {
        let frame = read_frame(raw_frame);

        //A single frame with color is worth enough to mark the entire animation as colorful
        if anim.grayscale {
            anim.grayscale = frame_is_grayscale(&frame);
        }

        anim.frames.push(frame);
    }

    info!("Read animation - Frame count: {}, Grayscale: {}", anim.frames.len(), anim.grayscale);
    Ok(anim)
}

/// Read the (next) frame of an `Animation`
///
/// # Input
/// A raw `gif::Frame` of the `Decoder` that to be parsed
///
/// # Output
/// A `Frame` of our own kind
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

    info!("Read frame - Pixel count: {}, Delay: {} ms", frame.pixels.len() * frame.pixels[0].len(), frame.delay);
    frame
}

/// Check if a given `Frame` is grayscale by checking all it's pixels
///
/// # Input
/// The `Frame` that is to check
///
/// # Output
/// If the `Frame` is grayscale
fn frame_is_grayscale(frame: &Frame) -> bool {
    let mut result = true;

    for row in frame.pixels {
        for pixel in row {
            if !pixel_is_grayscale(&pixel) {
                // A not grayscale image is enough to mark frame as not grayscale
                result = false;
                break;
            }
        }
        if !result { break; }
    }

    result
}

/// Check if a given `Pixel` is black
///
/// # Input
/// The `Pixel` to check
///
/// # Output
/// If the `Pixel` is black
pub fn pixel_is_black(pixel: &Pixel) -> bool {
    pixel.r == 0 && pixel.g == 0 && pixel.b == 0
}

/// Check if a given `Pixel` is grayscale
///
/// # Input
/// The `Pixel` to check
///
/// # Output
/// If the `Pixel` is grayscale
fn pixel_is_grayscale(pixel: &Pixel) -> bool {
    pixel.r == pixel.g && pixel.g == pixel.b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_animation() {
        // Test reading a grayscale animation
        let anim = read_animation(&PathBuf::from("gifs/base.gif")).unwrap();
        assert_eq!(anim.grayscale, true);
        assert_eq!(anim.frames.len(), 1);
        assert_eq!(frame_is_grayscale(&anim.frames[0]), true);

        // Test reading a colorful animation
        let anim = read_animation(&PathBuf::from("gifs/testbot.gif")).unwrap();
        assert_eq!(anim.grayscale, false);
        assert_eq!(anim.frames.len(), 1);
        assert_eq!(frame_is_grayscale(&anim.frames[0]), false);
    }

    #[test]
    fn test_read_frame() {
        // Read a frame from a GIF file
        let file = File::open("gifs/gray.gif").unwrap();
        let mut decoder = gif::DecodeOptions::new().read_info(file).unwrap();
        let raw_frame = decoder.read_next_frame().unwrap().unwrap();

        // Test that the returned frame has the correct values
        let frame = read_frame(&raw_frame);
        assert_eq!(frame.delay, 0);
        assert_eq!(frame.pixels, [[Pixel { r: 0, g: 0, b: 0, a: 0 }; WIDTH]; HEIGHT]);
    }


    #[test]
    fn test_frame_is_grayscale() {
        // Test a grayscale frame
        let frame = Frame {
            pixels: [[Pixel { r: 128, g: 128, b: 128, a: 255 }; WIDTH]; HEIGHT],
            delay: 100,
        };
        assert_eq!(frame_is_grayscale(&frame), true);

        // Test a colorful frame
        let frame = Frame {
            pixels: [[Pixel { r: 128, g: 128, b: 64, a: 255 }; WIDTH]; HEIGHT],
            delay: 100,
        };
        assert_eq!(frame_is_grayscale(&frame), false);
    }

    #[test]
    fn test_pixel_is_greyscale() {
        // Test a grayscale pixel
        let pixel = Pixel { r: 128, g: 128, b: 128, a: 255 };
        assert_eq!(pixel_is_grayscale(&pixel), true);

        // Test a colorful pixel
        let pixel = Pixel { r: 128, g: 128, b: 64, a: 255 };
        assert_eq!(pixel_is_grayscale(&pixel), false);
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