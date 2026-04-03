use core::pin::Pin;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use crate::bsp_ensea;
pub(crate) use crate::bsp_ensea::Bargraph;
pub(crate) use crate::bsp_ensea::BargraphPins;

impl BargraphPins {
    pub fn new(pins:BargraphPins) -> Self {
        let leds = pins.leds.map(|pin| {
            Output::new(pin, Level::Low, Speed::Low
            )});
        Self {
            leds,
            min_val: 0,
            max_val: 100,
        }
    }

    pub fn set_range(&mut self, min: u8, max: u8) {
        // Implémentation pour définir la plage de valeurs
    }

    pub fn set_value(&mut self, value: u8) {
        // Implémentation pour allumer les LEDs en fonction de la valeur
    }
}

