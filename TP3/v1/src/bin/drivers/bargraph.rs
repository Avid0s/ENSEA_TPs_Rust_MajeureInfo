impl Bargraph {
    pub fn new(pins: [AnyPin; num_pins]) -> Self {
        let leds = pins.map(|pin| {
            Output::new(pin, Level::Low, Speed::Low
            )});
        Self {
            leds,
            min: 0,
            max: 100,
        }
    }

    pub fn set_range(&mut self, min: u8, max: u8) {
        // Implémentation pour définir la plage de valeurs
    }

    pub fn set_value(&mut self, value: u8) {
        // Implémentation pour allumer les LEDs en fonction de la valeur
    }
}

