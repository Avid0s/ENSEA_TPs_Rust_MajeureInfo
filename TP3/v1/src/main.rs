#![no_main]
#![no_std]


use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32;
use {defmt_rtt as _, panic_probe as _};
mod bsp_ensea;
mod drivers;
use crate::bsp_ensea::Board;
use crate::drivers::bargraph::BargraphPins;

#[embassy_executor::main]
async fn main(_spawner:Spawner) {
    // Initialisation de la carte
    let _p: embassy_stm32::Peripherals = embassy_stm32::init(embassy_stm32::Config::default());

    let board = Board::new();

    // Création du bargraph
    let mut bargraph = BargraphPins::new(board.bargraph_pins);
    bargraph.set_range(10, 90);
    bargraph.set_value(50); // Allume 4 LEDs
}


