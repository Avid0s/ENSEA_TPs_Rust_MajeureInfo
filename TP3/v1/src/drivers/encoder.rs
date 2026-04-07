
use crate::embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::qei::Qei;
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::pac::gpio::vals::Pupdr::PULL_DOWN;
use embassy_stm32::{pac, Config, Peri, Peripherals};
use crate::bsp_ensea;


pub(crate) use crate::bsp_ensea::*;


impl RotaryEncoder {
    pub fn new(encoder_pins: RotaryEncoderPins) -> Self {

        let enc_button = Input::new(encoder_pins.enc_button, Pull::Down);
        let qei = Qei::new(encoder_pins.timer, encoder_pins.enc_a, encoder_pins.enc_b, embassy_stm32::timer::qei::Config::default());

        Self{
            enc_button,
            encoder_qei: qei,
        }
    }
    /// Lit la position actuelle de l'encodeur
    pub fn read_value(&self) -> u32 {
        // On lit le registre CNT du bloc TIM2
        pac::TIM2.cnt().read()
    }

    pub fn set_range(&self, max_value: u32) {
        let regs = pac::TIM2;
        regs.arr().write_value(max_value); // Définit la valeur max
        regs.cnt().write_value(0);         // Remet à zéro
    }





}


