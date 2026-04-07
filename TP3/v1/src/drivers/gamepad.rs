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
            Button::Up=> self.bp_top.is_low(),
            Button::Down=> self.bp_bottom.is_low(),
            Button::Left=> self.bp_left.is_low(),
            Button::Right=> self.bp_right.is_low(),
            Button::Center=> self.bp_center.is_low(),
        }
    }

    pub fn poll(&mut self)->GamepadState {
        let status:GamepadState = GamepadState{
            up: self.bp_top.is_low(),
            down: self.bp_bottom.is_low(),
            left: self.bp_left.is_low(),
            right: self.bp_right.is_low(),
            center: self.bp_center.is_low(),
        };

        status
    }


}


