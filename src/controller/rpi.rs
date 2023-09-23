use rppal::gpio::{Error, Gpio, OutputPin, Result};

use crate::controller::PinControl;

const FREQUENCY: f64 = 5000.0;

pub struct Rpi {
    pin: OutputPin,
    frequency: f64,
}

impl PinControl<Error> for Rpi {
    fn new(pin_number: u8) -> Result<Self> {
        static GPIO: Gpio = Gpio::new().unwrap();
        Ok(Rpi {
            pin: GPIO.get(pin_number)?.into_output(),
            frequency: FREQUENCY,
        })
    }

    fn set_high(&mut self) -> Result<()> {
        Ok(self.pin.set_high())
    }

    fn set_low(&mut self) -> Result<()> {
        Ok(self.pin.set_low())
    }

    fn set_pwm(&mut self, duty_cycle: f64) -> Result<()> {
        self.pin.set_pwm_frequency(self.frequency, duty_cycle)
    }
}
