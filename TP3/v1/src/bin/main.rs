#![no_std]
#![no_main]

mod bsp_ensea;

fn main() {
    // Initialisation de la carte
    let board = Board::new();

    // Création du bargraph
    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(10, 90);
    bargraph.set_value(50); // Allume 4 LEDs
}


