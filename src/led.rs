use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};

/*
    [0, 0, 0, 255] //white
    [0, 0, 255, 0] //red
    [0, 255, 0, 0] //green
    [255, 0, 0, 0] //blue
 */

/// Required parameter for setting up an hardware instance
pub struct LEDHardwareConfig {
    /// The frequency the for the pulse (i.e., rectangular) wave signal
    pub frequenz: u32,

    /// The DMA channel of the Raspberry Pi between 0 and 13
    pub dma: i32,

    /// Which GPIO the hardware is connected to
    pub pin: i32,

    /// The total count of all LEDs
    pub count: i32,

    ///Which type of strip (and color order) the hardware is (e.g. WS2812B, SK6812, SK6812W,...)
    pub strip_type: StripType,

    ///How brights the LEDs should be
    pub brightness: u8,

    ///If the LEDs are in the inverted order
    pub inverted: bool,
}

/// Build a LED controller
///
/// # Input
/// The `LEDHardwareConfig`, that is to use
///
/// # Output
/// A `Result<Controller, WS2811Error>`, were
/// * `Controller` is the successfully build LED controller
/// * `WS2811Error` is thrown, when the controller can't be build
pub fn build_controller(config: LEDHardwareConfig) -> Result<Controller, WS2811Error> {
    //#[cfg(not(target_arch = "arm"))]
    //return Err(WS2811Error::HwNotSupported);

    // Build a single channel controller
    //#[cfg(target_arch = "arm")]
    ControllerBuilder::new()
        .freq(config.frequenz)
        .dma(config.dma)
        .channel(
            0, // Channel Index, 0 is fineee
            ChannelBuilder::new()
                .pin(config.pin)
                .count(config.count) // Number of LEDs
                .strip_type(config.strip_type)
                .brightness(config.brightness) // max: 255
                .invert(config.inverted)
                .build(),
        ).build()
}