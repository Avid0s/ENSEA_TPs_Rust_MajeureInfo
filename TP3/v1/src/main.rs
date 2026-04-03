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
    bargraph.set_value(70); // Allume 6 LEDs

    //let mut count :u8 = 1;
    loop {
       Timer::after_millis(500).await;
        //bargraph.set_value(count.0*10+1);
        //count.0 +=1;


    }
}


