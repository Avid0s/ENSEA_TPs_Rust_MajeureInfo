#![no_std]
#![no_main]

use embassy_stm32::gpio::{Input, Level, Output, Pull, Pin, AnyPin};

pub struct Board{
    pub bargraph_pins: BargraphPins,
    pub stepper_pins: StepperPins,
    pub gamepad: GamepadPins,
    pub spi2: Spi2Pins,
    pub rotary_encoder_pins: RotaryEncoderPins,
    pub gps_pin: GpsPin,
    pub i2c1_pins: I2c1Pins,
    pub magnetometer_pins: MagnetometerPins,
}

pub struct BargraphPins{
    pub leds : [AnyPin; 8],
    pub min_val: u8,
    pub max_val: u8
}

struct StepperPins{
    pub direction: AnyPin,
    pub microstep_ms1: AnyPin,
    pub microstep_ms2: AnyPin,
    pub enable_enn: AnyPin,
    pub step_stp: AnyPin
}

struct GamepadPins{
    pub bp_top:     Input<'static>,
    pub bp_right:   Input<'static>,
    pub bp_bottom:  Input<'static>,
    pub bp_left:    Input<'static>,
    pub bp_center:  Input<'static>,
}

struct Spi2Pins{
    pub sck:  AnyPin,
    pub miso: AnyPin,
    pub mosi: AnyPin,
    pub cs:   AnyPin
}

struct RotaryEncoderPins{
    pub enc_a:      Input<'static>,
    pub enc_b:      Input<'static>,
    pub enc_utton:  Input<'static>,
}

struct GpsPin{
    pub gps_enn: AnyPin
}

struct I2c1Pins{
    pub sda: AnyPin,
    pub scl: AnyPin
}

struct MagnetometerPins{
    pub drdy: Input<'static>,
    pub int2: Input<'static>
}