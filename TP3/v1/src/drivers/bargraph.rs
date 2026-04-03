use embassy_stm32::gpio::Pin;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peripherals;

use crate::bsp_ensea;

pub(crate) use crate::bsp_ensea::BargraphPins;
use crate::bsp_ensea::{Bargraph, Board};

impl Board {
    pub fn new() -> Self {
        // Initialisation de la carte
        let p: embassy_stm32::Peripherals = embassy_stm32::init(embassy_stm32::Config::default());

        Self {
            bargraph_pins: BargraphPins {
                leds: [
                    p.PC7.into(),
                    p.PB2.into(),
                    p.PA8.into(),
                    p.PB1.into(),
                    p.PB15.into(),
                    p.PB4.into(),
                    p.PB14.into(),
                    p.PB5.into(),
                ],

            },/*
            stepper_pins: StepperPins {

            },
            gamepad: GamepadPins {

            },
            spi2: Spi2Pins {},
            rotary_encoder_pins: RotaryEncoderPins {},
            gps_pin: GpsPin {},
            i2c1_pins: I2c1Pins {},
            magnetometer_pins: MagnetometerPins {},
            */
        }
    }
}
impl Bargraph {
    pub fn new(pins: BargraphPins) -> Self {
        let leds = pins.leds.map(|pin| {Output::new(pin, Level::Low, Speed::Low )});

        Self {
            leds,
            min_val: 10,
            max_val: 100,
        }
    }


    pub fn set_range(&mut self, min: u8, max: u8) {
        // Implémentation pour définir la plage de valeurs
        self.min_val = min;
        self.max_val = max;
    }

    pub fn set_value(&mut self, value: u8) {
        // Implémentation pour allumer les LEDs en fonction de la valeur

        let range = self.max_val - self.min_val;
        let step = range / 8;

        for i in 0..8 {
            if value >= self.min_val + (i as u8 + 1) * step {
                self.leds[i].set_high();
            } else {
                self.leds[i].set_low();
            }
        }
    }
}

