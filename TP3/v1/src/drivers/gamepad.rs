use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::pac::gpio::vals::Pupdr::PULL_DOWN;
use embassy_stm32::Peripherals;

use crate::bsp_ensea;


pub(crate) use crate::bsp_ensea::*;


impl Gamepad {
    pub fn new(gamepad_pins: GamepadPins) -> Self {
        let bp_center = Input::new(gamepad_pins.bp_center, Pull::Down);
        let bp_top = Input::new(gamepad_pins.bp_top, Pull::Down);
        let bp_right = Input::new(gamepad_pins.bp_right, Pull::Down);
        let bp_bottom = Input::new(gamepad_pins.bp_bottom, Pull::Down);
        let bp_left = Input::new(gamepad_pins.bp_left, Pull::Down);
        Self{

            bp_center,
            bp_left,
            bp_right,
            bp_top,
            bp_bottom,


        }
    }
    pub fn is_pressed(&self, button:Button) -> bool {
        match button {
            Button::Up=> self.bp_top.is_high(),
            Button::Down=> self.bp_bottom.is_high(),
            Button::Left=> self.bp_left.is_high(),
            Button::Right=> self.bp_right.is_high(),
            Button::Center=> self.bp_center.is_high(),
        }
    }

    pub fn poll(&mut self)->GamepadState {
        let status:GamepadState = {
            self.bp_top.is_high(),
            self.bp_bottom
        }

    }


}


