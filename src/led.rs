use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, WS2811Error};

pub struct LEDHardwareConfig {
    pub frequenz: u32,
    pub dma: i32,
    pub pin: i32,
    pub count: i32,
    pub strip_type: StripType,
    pub brightness: u8,
    pub inverted: bool
}

pub fn build_controller(config: LEDHardwareConfig) -> Result<Controller, WS2811Error> {
    // Build a single channel controller
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