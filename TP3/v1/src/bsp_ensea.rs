use core::any::Any;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Pin, AnyPin};
use embassy_stm32::pac::timer::Tim2ch;
use embassy_stm32::Peri;
use embassy_stm32::peripherals::{PA0, PA1, PA2, TIM2};
use embassy_stm32::timer::qei::Qei;
use embassy_stm32::timer::{Ch1, TimerPin};

pub struct Board{
    pub bargraph_pins: BargraphPins,
    pub gamepad_pins: GamepadPins,
    pub rotary_encoder_pins: RotaryEncoderPins,
    /*
    pub stepper_pins: StepperPins,

    pub spi2: Spi2Pins,

    pub gps_pin: GpsPin,
    pub i2c1_pins: I2c1Pins,
    pub magnetometer_pins: MagnetometerPins,

     */
}

//Configuration des pins : 

impl Board {
    pub fn new() -> Self {
        // Initialisation de la carte
        let p: embassy_stm32::Peripherals = embassy_stm32::init(embassy_stm32::Config::default());

        Self {
            bargraph_pins: BargraphPins {
                leds: [
                    p.PC7.into(),
                    p.PB2.into(),
                    p.PA8.into(),
                    p.PB1.into(),
                    p.PB15.into(),
                    p.PB4.into(),
                    p.PB14.into(),
                    p.PB5.into(),
                ],

            },
            gamepad_pins: GamepadPins {
                bp_top:     p.PC8.into(),
                bp_right:   p.PC9.into(),
                bp_bottom:  p.PB11.into(),
                bp_left:    p.PC6.into(),
                bp_center:  p.PC5.into(),
            },
            rotary_encoder_pins: RotaryEncoderPins {
                enc_a: p.PA0,
                enc_b: p.PA1,
                enc_button: p.PA15.into(),
                timer: p.TIM2.into(),
            },
            /*
                stepper_pins: StepperPins {
    
                },
                
                spi2: Spi2Pins {},

                gps_pin: GpsPin {},
                i2c1_pins: I2c1Pins {},
                magnetometer_pins: MagnetometerPins {},
                */
        }
    }
}

pub struct BargraphPins{
    pub leds : [Peri<'static, AnyPin>; 8],
}

pub struct Bargraph {
    pub leds: [Output<'static>; 8],
    pub min_val: u8,
    pub max_val: u8,
}





struct StepperPins{
    pub direction: AnyPin,
    pub microstep_ms1: AnyPin,
    pub microstep_ms2: AnyPin,
    pub enable_enn: AnyPin,
    pub step_stp: AnyPin
}



pub(crate) struct GamepadPins{
    pub bp_top:     Peri<'static, AnyPin>,
    pub bp_right:   Peri<'static, AnyPin>,
    pub bp_bottom:  Peri<'static, AnyPin>,
    pub bp_left:    Peri<'static, AnyPin>,
    pub bp_center:  Peri<'static, AnyPin>,
}

//GAMEPAD :
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Center,
}

pub struct GamepadState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}
pub struct Gamepad {
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

pub struct RotaryEncoderPins{
    pub enc_a:     Peri<'static, PA0>,
    pub enc_b:      Peri<'static, PA1>,
    pub enc_button:  Peri<'static, AnyPin>,
    pub timer: Peri<'static, TIM2>,
}
pub struct RotaryEncoder{
    pub enc_button:      Input<'static>,
    pub encoder_qei: Qei<'static, embassy_stm32::peripherals::TIM2>,
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