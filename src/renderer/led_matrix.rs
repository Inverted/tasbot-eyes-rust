/*
 * ATTENTION:   This entire module is not yet tested nor will I!
 *              It just exists for completeness reasons, and serves
 *              as a proof of concept and might be debugged at a later point!
 */

use std::fmt::{Display, Formatter};
use std::ops::Add;
use log::{info, warn};
use rs_ws281x::{Controller, RawColor, StripType};
use thiserror::Error;

use crate::color::Color;
use crate::gif::{Animation, Frame, pixel_is_black};
use crate::led::LEDHardwareConfig;
use crate::renderer::{Renderer, sleep_frame_delay};
use crate::renderer::tasbot_eyes::{SCREEN_HEIGHT, SCREEN_WIDTH};

//default values but not fixed
const BRIGHTNESS: u8 = 4;
const TARGET_FREQ: u32 = 800_000;
const DMA: u8 = 10;
const INVERTED: bool = false;

#[derive(Error, Debug)]
pub enum LEDMatrixError {
    #[error("An error occurred: {0}")]
    Other(String),
}

pub struct LEDMatrixRenderer {
    pub controller: Controller,
    pub gamma_correction: bool,
    pub gamma: f32,
}

impl Display for LEDMatrixRenderer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&*format!("\t-Use gamma correction: {}\n", self.gamma_correction));
        result.push_str(&*format!("\t-Gamma correction value: {}", self.gamma));

        write!(f, "{}", result)
    }
}

impl Renderer for LEDMatrixRenderer {
    fn play(&mut self, anim: &Animation) {
        todo!();
        for frame in &anim.frames {
            show_frame(self, frame, None);
        }
    }

    fn play_colored(&mut self, anim: &Animation, color: &Color) {
        todo!();
        let color = if anim.grayscale { Some(color) } else { None };

        for frame in &anim.frames {
            //show_frame(self, frame, color);
        }
    }

    fn clear(&mut self) {
        todo!();
        clear(self);
    }

    fn print_config(&self) {
        todo!();
        info!("Start matrix renderer using those arguments:\n{}", self);
    }
}

fn show_frame(settings: &mut LEDMatrixRenderer, frame: &Frame, color: Option<&Color>) {
    let leds = settings.controller.leds_mut(0);

    //Index based for loops, as we need the index for the translation
    //todo: Indices might be horrible wrong
    for x in 0..frame.pixels.len() {
        for y in 0..frame.pixels[x].len() {

            let mut rend_color: RawColor;
            match color {
                //Use color of frame
                None => {
                    rend_color = [
                        frame.pixels[y][x].b,
                        frame.pixels[y][x].g,
                        frame.pixels[y][x].r,
                        0,
                    ];
                }

                //Use color given
                Some(color) => {
                    if pixel_is_black(&frame.pixels[y][x]) {
                        rend_color = [0, 0, 0, 0];
                    } else {
                        rend_color = [
                            color.b,
                            color.g,
                            color.r,
                            0,
                        ];
                    }
                }
            }
            let index = y * SCREEN_WIDTH + x; //todo, not tested
            leds[index] = rend_color;
        }
    }

    //Render
    render(settings);

    //Sleep
    sleep_frame_delay(frame);
}

fn clear(settings: &mut LEDMatrixRenderer) {
    let leds = settings.controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }

    render(settings);
}

fn render(settings: &mut LEDMatrixRenderer) {
    //#[cfg(target_arch = "arm")]
    match settings.controller.render() {
        Ok(_) => info!("Rendering okay"),
        Err(_) => warn!("Rendering failed")
    }
}

pub fn get_led_matrix_config(
    strip_type: StripType,
    pin: u8,
    width: usize,
    height: usize,
    brightness: Option<u8>,
    target_freq: Option<u32>,
    dma: Option<u8>,
    inverted: Option<bool>,
) -> Result<LEDHardwareConfig, LEDMatrixError> {
    todo!();

    //Renderer must fit minimum size
    if width < SCREEN_WIDTH || height < SCREEN_HEIGHT {
        return Err(LEDMatrixError::Other(String::new().add("Display size to small!")));
    }

    let count = width * height;
    Ok(LEDHardwareConfig {
        strip_type,
        pin: pin as i32,
        count: count as i32,

        brightness: brightness.unwrap_or(BRIGHTNESS),
        frequenz: target_freq.unwrap_or(TARGET_FREQ),
        dma: dma.unwrap_or(DMA) as i32,
        inverted: inverted.unwrap_or(INVERTED),
    })
}