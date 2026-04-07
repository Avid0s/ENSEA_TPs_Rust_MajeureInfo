use crate::embassy_stm32::interrupt::TIM2;
use crate::embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::qei::Qei;
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::pac::gpio::vals::Pupdr::PULL_DOWN;
use embassy_stm32::{Config, Peripherals};
use crate::bsp_ensea;


pub(crate) use crate::bsp_ensea::*;


impl RotaryEncoder {
    pub fn new(tim2: embassy_stm32::peripherals::TIM2, encoder_pins: RotaryEncoderPins) -> Self {

        let enc_button = Input::new(encoder_pins.enc_button, Pull::Down);
        let qei = Qei::new(tim2, encoder_pins.enc_a, encoder_pins.enc_b, Config::Default);

        Self{
            enc_button,
            encoder_qei: qei,
        }
    }
    pub fn encoder_value(&self) -> Self {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.arr().write_value(10_000); // ARR : Auto-Reload Register, valeur maximale du compteur
        tim2.cnt().write_value(5_000); // CNT : Counter Register, définit la valeur actuelle du compteur
    }





}


