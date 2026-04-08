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
use crate::bsp_ensea::{Bargraph, Board, Button, RotaryEncoder, Stepper};
use crate::bsp_ensea::StepperDirection::{Clockwise, CounterClockwise};
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


        // On transforme la position (ex: 0 à 100) en fréquence (ex: 0 à 1000 Hz)
        // On sature à 2000 Hz pour ne pas faire siffler le moteur inutilement
        let speed_pas = (encoder.read_value() * 10).min(2000);

        // --- ACTION ---
        let dir_choice = stepper.dir_choice;
        stepper.set_speed(speed_pas, dir_choice);
        if(gamepad.is_pressed(Button::Left)){
            stepper.set_speed(speed_pas, Clockwise);
        }
        if (gamepad.is_pressed(Button::Right)){
            stepper.set_speed(speed_pas, CounterClockwise);
        }
        if (gamepad.is_pressed(Button::Down)){
            let new_mode = stepper.microstep_mode.previous();
            stepper.set_microstepping(new_mode);
        }
        if (gamepad.is_pressed(Button::Up)){
            let new_mode = stepper.microstep_mode.next();
            stepper.set_microstepping(new_mode);
        }



        // Debug dans la console
        if speed_pas > 0 {
            defmt::info!("Vitesse moteur : {} pas par tour.\n, Dir : {}\n, MS : {}", speed_pas, stepper.dir_choice, stepper.microstep_mode);
        }

        // Petite pause pour ne pas saturer la console
        Timer::after_millis(250).await;
    }
}
