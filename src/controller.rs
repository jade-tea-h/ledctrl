use std::error::Error;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

use crate::led::{self, Led, ToDutyCycle};

pub mod rpi;

const EVALUATE_PERIOD: f64 = 0.01;

enum PinState {
    High,
    Low,
    Pwm(f64),
}

pub trait PinControl<E: Error>: Sized {
    fn new(pin_number: u8) -> Result<Self, E>;
    fn set(&mut self, state: PinState) -> Result<(), E> {
        match state {
            PinState::High => self.set_high(),
            PinState::Low => self.set_low(),
            PinState::Pwm(cycle) => self.set_pwm(cycle),
        }
    }
    fn set_high(&mut self) -> Result<(), E>;
    fn set_low(&mut self) -> Result<(), E>;
    fn set_pwm(&mut self, duty_cycle: f64) -> Result<(), E>;
}

enum Request {
    Static(Led<led::Dtype>),
    Blink(Led<led::Dtype>, f64, f64),
    Fade(Led<led::Dtype>, f64),
    Sequence(Vec<Led<led::Dtype>>, f64),
    BlinkSequence(Vec<Led<led::Dtype>>, f64, f64),
    FadeSequence(Vec<Led<led::Dtype>>, f64),
    Off,
}

struct Controller<T, E>
where
    T: PinControl<E>,
    E: Error,
{
    control: Led<T>,
    queue: Box<dyn FnMut(Duration) -> Option<Led<led::Dtype>>>,
    reversed: bool,
    stamp: Instant,

    _phantom: PhantomData<E>,
}

impl<T, E> Controller<T, E>
where
    T: PinControl<E>,
    E: Error,
{
    fn new(pins: Led<u8>, reverse: Option<bool>) -> Result<Self, E> {
        let pins = match pins {
            Led::Single(pin) => Led::Single(T::new(pin)?),
            Led::Rgb(r, g, b) => Led::Rgb(T::new(r)?, T::new(g)?, T::new(b)?),
        };
        Ok(Controller {
            control: pins,
            queue: Box::new(|_| None),
            reversed: reverse.unwrap_or(false),
            stamp: Instant::now(),

            _phantom: PhantomData,
        })
    }
    fn send_request(&mut self, request: Request) -> Result<(), E> {
        match request {
            Request::Static(value) => {
                self.set_pins(&value)?;
                self.queue = Box::new(|_| None);
            }
            Request::Blink(value, time_on, time_off) => {
                self.set_pins(&value)?;
                let mut times = [time_on.clone(), time_off.clone()].iter().cycle();
                let off = value.get_off();
                let mut values = [off, value.clone()].iter().cycle();
                let mut next_time = Duration::from_secs_f64(*times.next().unwrap());
                self.queue = Box::new(
                    |delta| {
                        if delta > next_time {
                            next_time = Duration::from_secs_f64(*times.next().unwrap());
                            return Some(values.next().unwrap().clone());
                        }
                        None
                    }
                )
            }
            Request::Fade(value, time) => {
                let value = value.to_duty_cycle();
                self.set_pins(&value)?;
                let step = value / EVALUATE_PERIOD;
                let mut values = std::iter::successors(Some(value), |prev| Some(*prev - step));
                let period = Duration::from_secs_f64(EVALUATE_PERIOD);
                self.queue = Box::new(
                    |delta| {
                        if delta > period {
                            return Some(values.next().unwrap().as_dtype());
                        }
                        None
                    }
                );
            }
            _ => todo!(),
        }
        self.stamp = Instant::now();
        Ok(())
    }

    fn evaluate(&mut self) -> Result<bool, E> {
        let delta = Instant::now() - self.stamp;
        match (self.queue)(delta) {
            Some(value) => {
                self.set_pins(&value)?;
                Ok(true)
            },
            None => Ok(false),
        }
    }

    fn set_pins(&mut self, value: &impl ToDutyCycle) -> Result<(), E> {
        // self.control.set(value, self.reversed);
        Ok(())
    }
}
