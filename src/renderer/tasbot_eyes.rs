use std::fmt::{Display, Formatter};

use log::{info, warn};
use rs_ws281x::{Controller, RawColor, StripType};

use crate::color::{BLACK, Color, get_gamma_correction};
use crate::gif::{Animation, Frame, pixel_is_black};
use crate::led::LEDHardwareConfig;
use crate::renderer::{Renderer, sleep_frame_delay};

//fixed values
/// The frequency the for the pulse (i.e., rectangular) wave signal for TASBot
const TARGET_FREQ: u32 = 800_000;

/// The DMA channel of the Raspberry Pi
const DMA: u8 = 10;

///The strip type TASBots eyes are based on
const STRIP_TYPE: StripType = StripType::Sk6812;

///If the LEDs are in the inverted order
const INVERTED: bool = false;

///The height of the TASBot display
pub const SCREEN_WIDTH: usize = 28;

///The height of the TASBot display
pub const SCREEN_HEIGHT: usize = 8;

///The actual count of LEDs TASBots displays
pub const NUM_PIXELS: u32 = 154;



//default values but not fixed
///The default GPIO pin the display is using on the Raspberry Pi
pub const DEFAULT_GPIO_PIN: u8 = 10;

///The default brightness of the LEDs
pub const DEFAULT_BRIGHTNESS: u8 = 4;


#[rustfmt::skip]
/// The translation matrix, that converts any given 2D index to the actual index of the display on TASBots display
///
/// From [jakobrs original tasbot-display software](https://github.com/jakobrs/tasbot-display/blob/b8854b3f0dc096d4609124a28d8e400acd774b29/src/tasbot.rs)
pub const PIXEL_POSITIONS: [[Option<usize>; SCREEN_WIDTH]; SCREEN_HEIGHT] = [
    [None, None, Some(0), Some(1), Some(2), Some(3), None, None, None, None, Some(101), Some(100), Some(99), Some(98), Some(97), Some(96), Some(95), Some(94), None, None, None, None, Some(105), Some(104), Some(103), Some(102), None, None],
    [None, Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, None, Some(84), Some(85), Some(86), Some(87), Some(88), Some(89), Some(90), Some(91), Some(92), Some(93), None, None, Some(111), Some(110), Some(109), Some(108), Some(107), Some(106), None],
    [Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), None, None, None, None, None, None, None, None, None, None, None, None, Some(119), Some(118), Some(117), Some(116), Some(115), Some(114), Some(113), Some(112)],
    [Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), None, None, None, Some(83), Some(82), Some(81), Some(80), Some(79), Some(78), None, None, None, Some(127), Some(126), Some(125), Some(124), Some(123), Some(122), Some(121), Some(120)],
    [Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), None, None, Some(70), Some(71), Some(72), Some(73), Some(74), Some(75), Some(76), Some(77), None, None, Some(135), Some(134), Some(133), Some(132), Some(131), Some(130), Some(129), Some(128)],
    [Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41), None, None, None, None, None, None, None, None, None, None, None, None, Some(143), Some(142), Some(141), Some(140), Some(139), Some(138), Some(137), Some(136)],
    [None, Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), None, None, None, Some(68), Some(67), Some(66), Some(65), Some(64), Some(63), Some(62), Some(61), None, None, None, Some(149), Some(148), Some(147), Some(146), Some(145), Some(144), None],
    [None, None, Some(48), Some(49), Some(50), Some(51), None, None, None, Some(69), Some(52), Some(53), Some(54), Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), None, None, None, Some(153), Some(152), Some(151), Some(150), None, None]
];

/// Configuration for the TASBot renderer
pub struct TASBotRendererSettings {
    pub controller: Controller,
    pub gamma_correction: bool,
    pub gamma: f32,
}

impl Display for TASBotRendererSettings {

    ///Print out the configuration, that is used
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&*format!("\t-Use gamma correction: {}\n", self.gamma_correction));
        result.push_str(&*format!("\t-Gamma correction value: {}", self.gamma));

        write!(f, "{}", result)
    }
}

impl Renderer for TASBotRendererSettings {
    fn play(&mut self, anim: Animation) {
        for frame in &anim.frames {
            show_frame(self, frame, None);
        }
    }

    fn play_colored(&mut self, anim: Animation, color: &Color) {
        let color = if anim.grayscale { Some(color) } else { None };

        for frame in &anim.frames {
            show_frame(self, frame, color);
        }
    }

    fn clear(&mut self) {
        clear(self);
    }

    fn print_config(&self) { info!("Start TASBot renderer using those arguments:\n{}", self); }
}

/// Handle a frame that's to be rendered in the console
///
/// # Input
/// `settings`: The configuration that should be used for rendering
/// `frame`: The `Frame` that should be rendered
/// `color`: An optional color to overwrite the frames own color
fn show_frame(settings: &mut TASBotRendererSettings, frame: &Frame, color: Option<&Color>) {
    let leds = settings.controller.leds_mut(0);

    //Index based for loops, as we need the index for the translation
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let index = PIXEL_POSITIONS[y][x];
            match index {
                None => {}
                Some(index) => {
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
                                rend_color = BLACK.to_raw();
                            } else {
                                rend_color = color.to_raw();
                            }
                        }
                    }

                    if settings.gamma_correction {
                        let corrected_color: Vec<u8> = rend_color.iter().map(|v| get_gamma_correction(*v, settings.gamma)).collect();

                        rend_color = [
                            corrected_color[0],
                            corrected_color[1],
                            corrected_color[2],
                            0,
                        ];
                    }

                    leds[index] = rend_color;
                }
            }
        }
    }

    //Render
    render(settings);

    //Sleep
    sleep_frame_delay(frame);
}

fn clear(settings: &mut TASBotRendererSettings) {
    let leds = settings.controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }

    render(settings);
}

/// Render the current controller buffer to the LEDs
///
/// # Input
/// `settings`: The configuration that should be used for rendering, which wraps the controller as well
fn render(settings: &mut TASBotRendererSettings) {
    //#[cfg(target_arch = "arm")]
    match settings.controller.render() {
        Ok(_) => { info!("Rendering okay") }
        Err(_) => { warn!("Rendering failed") }
    }
}

///Create a new `LEDHardwareConfig` structure fitting for TASBots display
///
/// # Input
/// * `pin`: An optional GPIO pin. Use the default pin otherwise.
/// * `brightness`: The optional maximum brightness of the LEDs. Use the default brightness otherwise.
pub fn get_tasbot_eye_config(pin: Option<u8>, brightness: Option<u8>) -> LEDHardwareConfig {
    LEDHardwareConfig {
        frequenz: TARGET_FREQ,
        dma: DMA as i32,
        pin: pin.unwrap_or(DEFAULT_GPIO_PIN) as i32,
        count: NUM_PIXELS as i32,
        strip_type: STRIP_TYPE,
        brightness: brightness.unwrap_or(DEFAULT_BRIGHTNESS),
        inverted: INVERTED,
    }
}