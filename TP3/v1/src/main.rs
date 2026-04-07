#![no_main]
#![no_std]

use core::num::Wrapping;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
mod bsp_ensea;
mod drivers;
use crate::bsp_ensea::{Bargraph, Board};
use crate::drivers::bargraph::BargraphPins;

#[embassy_executor::main]
async fn main(_spawner:Spawner) {

    let board = Board::new();

    // Création du bargraph
    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(10, 90);
    bargraph.set_value(10); // Allume 0 LEDs

    let mut gamepad = crate::drivers::gamepad::Gamepad::new(board.gamepad_pins);

    let mut count :u8 = 1;
    loop {
        if(gamepad.is_pressed(crate::drivers::gamepad::Button::Center)){
            
            bargraph.set_value(count*10);
            count = (count + 1) % 10;
        }

        let gamepad_state = gamepad.poll();
        info!("Gamepad state:\n up={}\n left={}, center={}, right={},\n down={},", 
            gamepad_state.up, gamepad_state.down, gamepad_state.left, gamepad_state.right, gamepad_state.center);
        Timer::after_millis(250).await;
    }
}


