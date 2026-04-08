#![no_main]
#![no_std]

use core::num::{Saturating, Wrapping};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32;
use embassy_stm32::pac::TIM2;
use embassy_stm32::peripherals::TIM2;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
mod bsp_ensea;
mod drivers;
use crate::bsp_ensea::{Bargraph, Board, RotaryEncoder, Stepper};
use crate::drivers::bargraph::BargraphPins;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();

    // Création du bargraph
    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(0, 80);
    bargraph.set_value(0); // Allume 0 LEDs

    let mut gamepad = crate::drivers::gamepad::Gamepad::new(board.gamepad_pins);
    let mut count: u8 = 1;

    //Encoder
    let mut encoder = RotaryEncoder::new(board.rotary_encoder_pins);
    let mut position: Wrapping<u32> = Wrapping(0);
    encoder.set_range(80);

    let mut stepper = Stepper::new(board.stepper_pins);

    loop {
        /*
        if(gamepad.is_pressed(crate::drivers::gamepad::Button::Center)){

            bargraph.set_value(count*10);
            count = (count + 1) % 10;
        }
        */

        let gamepad_state = gamepad.poll();
        info!(
            "Gamepad state:\n up={}\n left={}, center={}, right={},\n down={},",
            gamepad_state.up,
            gamepad_state.left,
            gamepad_state.center,
            gamepad_state.right,
            gamepad_state.down
        );

        // test Encoder :

        position = Wrapping(encoder.read_value());

        // Affichage dans la console de débug
        defmt::info!("Position de l'encodeur : {}", position.0);
        bargraph.set_value(position.0 as u8);

        //Button = reset
        if (encoder.enc_button.is_low()) {
            encoder.reset();
        }

        // --- LECTURE ---
        let pos = encoder.read_value();

        // --- TRAITEMENT (Mapping) ---
        // On transforme la position (ex: 0 à 100) en fréquence (ex: 0 à 1000 Hz)
        // On sature à 2000 Hz pour ne pas faire siffler le moteur inutilement
        let speed_hz = (pos * 10);

        // --- ACTION ---
        stepper.set_speed(speed_hz, encoder.encoder_qei.read_direction());

        // Debug dans la console
        if speed_hz > 0 {
            defmt::info!("Vitesse moteur : {} Hz", speed_hz);
        }

        // Petite pause pour ne pas saturer la console
        Timer::after_millis(250).await;
    }
}
