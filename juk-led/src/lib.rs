#![no_std]

use esp_hal::{
    Async,
    Blocking,
    DriverMode,
    gpio::{Level, interconnect::PeripheralOutput},
    rmt::{Channel, PulseCode, Tx, TxChannelConfig, TxChannelCreator},
};

const T0H: u32 = 350;
const T0L: u32 = 800;

const T1H: u32 = 700;
const T1L: u32 = 600;

const PULSE_0: PulseCode = PulseCode::new(
    Level::High,
    ((T0H * 80) / 1000) as u16,
    Level::Low,
    ((T0L * 80) / 1000) as u16,
);

const PULSE_1: PulseCode = PulseCode::new(
    Level::High,
    ((T1H * 80) / 1000) as u16,
    Level::Low,
    ((T1L * 80) / 1000) as u16,
);

#[derive(defmt::Format, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    fn to_pulses(&self, pulses: &mut [PulseCode; 25]) {
        for pos in 0..8 {
            match self.g & (1 << pos) {
                0 => pulses[pos] = PULSE_0,
                _ => pulses[pos] = PULSE_1,
            }
        }
        for pos in 0..8 {
            match self.r & (1 << pos) {
                0 => pulses[8 + pos] = PULSE_0,
                _ => pulses[8 + pos] = PULSE_1,
            }
        }
        for pos in 0..8 {
            match self.b & (1 << pos) {
                0 => pulses[16 + pos] = PULSE_0,
                _ => pulses[16 + pos] = PULSE_1,
            }
        }
    }
}

pub struct LEDAdapter<'ch, Dm>
where
    Dm: DriverMode,
{
    channel: Option<Channel<'ch, Dm, Tx>>,
    buffer: [PulseCode; 25],
}

impl<'ch, Dm> LEDAdapter<'ch, Dm>
where
    Dm: DriverMode,
{
    fn channel_config() -> TxChannelConfig {
        TxChannelConfig::default()
            .with_clk_divider(1)
            .with_idle_output(true)
            .with_idle_output_level(Level::Low)
            .with_carrier_modulation(false)
    }

    pub fn new<C, O>(channel: C, pin: O) -> Self
    where
        C: TxChannelCreator<'ch, Dm>,
        O: PeripheralOutput<'ch>,
    {
        let channel = defmt::expect!(
            channel.configure_tx(pin, Self::channel_config()),
            "Failed to configure the RMT channel"
        );

        Self {
            channel: Some(channel),
            buffer: [PulseCode::end_marker(); 25],
        }
    }
}

impl<'ch> LEDAdapter<'ch, Blocking> {
    pub fn set_color(&mut self, color: &RGB) {
        color.to_pulses(&mut self.buffer);
        defmt::debug!("Setting LED color to: {:?}", color);
        defmt::trace!("Transmitting: {=[?; 25]}", self.buffer);

        let ch = defmt::expect!(
            self.channel.take(),
            "At this point `self.channel` should be `Some`"
        );

        match ch.transmit(&self.buffer) {
            Ok(tx) => match tx.wait() {
                Ok(ch) => self.channel = Some(ch),
                Err((e, ch)) => {
                    defmt::warn!("LED color not set: {}", e);
                    self.channel = Some(ch);
                }
            },
            Err(_) => {
                defmt::unreachable!("`self.buffer` is always a valid input to `ch.transmit()`")
            }
        }
    }
}

impl<'ch> LEDAdapter<'ch, Async> {
    pub async fn set_color(&mut self, color: &RGB) {
        color.to_pulses(&mut self.buffer);
        defmt::debug!("Setting LED color to: {:?}", color);
        defmt::trace!("Transmitting: {=[?; 25]}", self.buffer);

        let ch = defmt::expect!(
            self.channel.as_mut(),
            "We never leave this value as `None` in the async adapter"
        );

        if let Err(e) = ch.transmit(&self.buffer).await {
            defmt::warn!("LED color not set: {}", e);
        }
    }
}
