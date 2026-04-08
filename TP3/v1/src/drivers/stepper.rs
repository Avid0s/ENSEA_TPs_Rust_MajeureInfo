use crate::bsp_ensea;
use crate::embassy_stm32::peripherals::TIM2;
use cortex_m::prelude::_embedded_hal_Pwm;
use embassy_stm32::gpio::OutputType::PushPull;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::interrupt::InterruptExt;
use embassy_stm32::pac::TIM3;
use embassy_stm32::pac::gpio::vals::Pupdr::PULL_DOWN;
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::qei::{Direction, Qei};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::{Config, Peri, Peripherals, pac};

pub(crate) use crate::bsp_ensea::*;
use crate::drivers::stepper::MicrosteppingMode::EighthStep;

impl Stepper {
    pub(crate) fn new(stepper_pins: StepperPins) -> Self {
        let step_stp = SimplePwm::new(
            stepper_pins.timer,
            Some(PwmPin::new(stepper_pins.step_stp, PushPull)),
            None,
            None,
            None,
            embassy_stm32::time::hz(1000),
            Default::default(),
        );

        let direction = Output::new(stepper_pins.direction, Level::Low, Speed::Low);
        let microstep_ms1 = Output::new(stepper_pins.microstep_ms1, Level::Low, Speed::Low);
        let microstep_ms2 = Output::new(stepper_pins.microstep_ms2, Level::Low, Speed::Low);
        let enable_enn = Output::new(stepper_pins.enable_enn, Level::Low, Speed::Low);
        let microstep_mode = EighthStep;

        Self {
            direction,
            microstep_ms1,
            microstep_ms2,
            enable_enn,
            step_stp,
            microstep_mode,
        }
    }
    ///Based on datasheet TMC2226 |MS2, MS1: 00: 1/8, 01: 1/32, 10: 1/64 11: 1/16
    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
        match mode {
            MicrosteppingMode::EighthStep => {
                self.microstep_ms1.set_low();
                self.microstep_ms2.set_low();
            }
            MicrosteppingMode::ThirtyTwoStep => {
                self.microstep_ms1.set_high();
                self.microstep_ms2.set_low();
            }
            MicrosteppingMode::SixtyFourStep => {
                self.microstep_ms1.set_low();
                self.microstep_ms2.set_high();
            }
            MicrosteppingMode::SixteenthStep => {
                self.microstep_ms1.set_high();
                self.microstep_ms2.set_high();
            }
        }
        self.microstep_mode = mode;
    }

    pub fn set_speed(&mut self, speed: u32, direction: Direction) {
        match direction {
            Direction::Upcounting => self.direction.set_high(),
            Direction::Downcounting => self.direction.set_low(),
        };

        if speed == 0 {
            // Si la vitesse est 0, on coupe le signal pour arrêter le moteur
            self.step_stp.disable(Channel::Ch1);
            self.enable_enn.set_high();
        } else {
            // 1. On change la vitesse de "battement" du timer
            let mut frequency = 1;
            match self.microstep_mode {
                MicrosteppingMode::EighthStep => {
                    frequency = speed * 8;
                }
                MicrosteppingMode::SixteenthStep => {
                    frequency = speed * 16;
                }
                MicrosteppingMode::ThirtyTwoStep => {
                    frequency = speed * 32;
                }
                MicrosteppingMode::SixtyFourStep => {
                    frequency = speed * 64;
                }
            }

            self.step_stp
                .set_frequency(embassy_stm32::time::hz(frequency));

            // 2. On s'assure que l'impulsion est bien formée (50% de largeur)
            let max = self.step_stp.get_max_duty();
            self.step_stp.set_duty(Channel::Ch1, max / 2);

            // 3. On lance le bal (si ce n'était pas déjà fait)
            self.step_stp.enable(Channel::Ch1);
            self.enable_enn.set_low();
        }
    }
}
