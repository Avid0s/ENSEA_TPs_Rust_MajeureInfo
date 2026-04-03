use embassy_stm32::gpio::{Input, Level, Output, Pull, Pin};

struct Board{
    pub bargraph_pins: BargraphPins,
    pub stepper_pins: StepperPins,
    pub gamepad: GamepadPins,
    pub spi2: Spi2Pins,
    pub rotary_encoder_pins: RotaryEncoderPins,
    pub gps_pin: GpsPin,
    pub i2c1_pins: I2c1Pins,
    pub magnetometer_pins: MagnetometerPins,
}

struct BargraphPins{
    pub led_1: Pin,
    pub led_2: Pin,
    pub led_3: Pin,
    pub led_4: Pin,
    pub led_5: Pin, 
    pub led_6: Pin,
    pub led_7: Pin,
    pub led_8: Pin
}

struct StepperPins{
    pub direction: Pin,
    pub microstep_ms1: Pin,
    pub microstep_ms2: Pin,
    pub enable_enn: Pin,
    pub step_stp: Pin
}

struct GamepadPins{
    pub bp_top: Input<Pull::Down>,
    pub bp_right: Input<Pull::Down>,
    pub bp_bottom: Input<Pull::Down>,
    pub bp_left: Input<Pull::Down>,
    pub bp_center: Input<Pull::Down>,
}

struct Spi2Pins{
    pub sck: Pin,
    pub miso: Pin,
    pub mosi: Pin,
    pub cs: Pin
}

struct RotaryEncoderPins{
    pub enc_a: Input<Pull::Down>,
    pub enc_b: Input<Pull::Down>,
    pub enc_button: Input<Pull::Down>,
}

struct GpsPin{
    pub gps_enn: Pin
}

struct I2c1Pins{
    pub sda: Pin,
    pub scl: Pin
}

struct MagnetometerPins{
    pub drdy: Input<Pull::Down>,
    pub int2: Input<Pull::Down>
}