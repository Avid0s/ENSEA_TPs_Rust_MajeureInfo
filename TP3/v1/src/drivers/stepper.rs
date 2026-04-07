use cortex_m::prelude::_embedded_hal_Pwm;
use crate::embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::qei::{Direction, Qei};
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::pac::gpio::vals::Pupdr::PULL_DOWN;
use embassy_stm32::{pac, Config, Peri, Peripherals};
use embassy_stm32::gpio::OutputType::PushPull;
use embassy_stm32::pac::TIM3;
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use crate::bsp_ensea;


pub(crate) use crate::bsp_ensea::*;

impl Stepper{
    pub(crate) fn new(stepper_pins: StepperPins) -> Self {
        let step_stp = SimplePwm::new(stepper_pins.timer, Some(PwmPin::new(stepper_pins.step_stp, PushPull)), None, None, None, embassy_stm32::time::hz(1000), Default::default() );

        let direction = Output::new(stepper_pins.direction, Level::Low, Speed::Low);
        let microstep_ms1 = Output::new(stepper_pins.microstep_ms1, Level::Low, Speed::Low);
        let microstep_ms2 = Output::new(stepper_pins.microstep_ms2, Level::Low, Speed::Low);
        let enable_enn = Output::new(stepper_pins.enable_enn, Level::Low, Speed::Low);

        Self{
            direction,
            microstep_ms1,
            microstep_ms2,
            enable_enn,
            step_stp,
        }
    }

    pub fn set_speed(&mut self, frequency_hz: u32) {
        if frequency_hz == 0 {
            // Si la vitesse est 0, on coupe le signal pour arrêter le moteur
            self.step_stp.disable(Channel::Ch1);
        } else {
            // 1. On change la vitesse de "battement" du timer
            self.step_stp.set_frequency(embassy_stm32::time::hz(frequency_hz));

            // 2. On s'assure que l'impulsion est bien formée (50% de largeur)
            let max = self.step_stp.get_max_duty();
            self.step_stp.set_duty(Channel::Ch1, max / 2);

            // 3. On lance le bal (si ce n'était pas déjà fait)
            self.step_stp.enable(Channel::Ch1);
        }
    }
}