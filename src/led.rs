#[derive(Copy, Clone)]
pub enum Led<T> {
    Single(T),
    Rgb(T, T, T),
}

#[derive(Clone)]
pub enum Dtype {
    On,
    Off,
    Hex(String),
    Percent(u8),
    Cycle(f64),
    U8(u8),
}

impl Dtype {
    pub fn to_cycle(&self) -> f64 {
        match self {
            Dtype::Cycle(val) => *val,
            Dtype::Percent(val) => f64::from(*val) / 100.0,
            Dtype::U8(val) => f64::from(*val) / f64::from(u8::MAX),
            Dtype::Hex(val) => todo!("Can't convert hexcodes yet"),
            _ => unimplemented!(),
        }
    }
}

pub trait ToDutyCycle {
    fn to_duty_cycle(&self) -> Led<f64>;
}
impl ToDutyCycle for Led<f64> {
    fn to_duty_cycle(&self) -> Led<f64> { *self }
}
impl ToDutyCycle for Led<Dtype> {
    fn to_duty_cycle(&self) -> Led<f64> {
        match self {
            Led::Single(val) => Led::Single(val.to_cycle()),
            Led::Rgb(r, g, b) => Led::Rgb(r.to_cycle(), g.to_cycle(), b.to_cycle()),
            _ => unreachable!(),
        }
    }
}
impl Led<Dtype> {
    pub fn get_off(&self) -> Led<Dtype> {
        match self {
            Led::Single(_) => Led::Single(Dtype::Off),
            Led::Rgb(..) => Led::Rgb(Dtype::Off, Dtype::Off, Dtype::Off),
            _ => unreachable!(),
        }
    }
}

use std::ops;

const THRESHOLD: f64 = 0.0001;
fn cycle_to_dtype(cycle: f64) -> Dtype {
    if cycle < THRESHOLD {
        return Dtype::Off;
    } else if cycle > 1.0 - THRESHOLD {
        return Dtype::On;
    } else {
        return Dtype::Cycle(cycle);
    }
}
impl Led<f64> {
    pub fn as_dtype(&self) -> Led<Dtype> {
        match self {
            Led::Single(v) => Led::Single(cycle_to_dtype(*v)),
            Led::Rgb(r,g,b) => {
                Led::Rgb(cycle_to_dtype(*r), cycle_to_dtype(*g), cycle_to_dtype(*b))
            },
        }
    }
}

impl ops::Add<f64> for Led<f64> {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        match self {
            Led::Single(v) => Led::Single(v+other),
            Led::Rgb(r,g,b) => Led::Rgb(r+other, g+other, b+other),
        }
    }
}
impl ops::Sub<f64> for Led<f64> {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        match self {
            Led::Single(v) => Led::Single(v-other),
            Led::Rgb(r,g,b) => Led::Rgb(r-other, g-other, b-other),
        }
    }
}
impl ops::Mul<f64> for Led<f64> {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        match self {
            Led::Single(v) => Led::Single(v*other),
            Led::Rgb(r,g,b) => Led::Rgb(r*other, g*other, b*other),
        }
    }
}
impl ops::Div<f64> for Led<f64> {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        match self {
            Led::Single(v) => Led::Single(v/other),
            Led::Rgb(r,g,b) => Led::Rgb(r/other, g/other, b/other),
        }
    }
}

impl ops::Add<Led<f64>> for Led<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Led::Single(v) => match other {
                Led::Single(o_v) => Led::Single(v + o_v),
                _ => unimplemented!(),
            }
            Led::Rgb(r,g,b) => match other {
                Led::Rgb(o_r, o_g, o_b) => Led::Rgb(r + o_r, g + o_g, b + o_b),
                _ => unimplemented!(),
            }
        }
    }
}
impl ops::Sub<Led<f64>> for Led<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match self {
            Led::Single(v) => match other {
                Led::Single(o_v) => Led::Single(v - o_v),
                _ => unimplemented!(),
            }
            Led::Rgb(r,g,b) => match other {
                Led::Rgb(o_r, o_g, o_b) => Led::Rgb(r - o_r, g - o_g, b - o_b),
                _ => unimplemented!(),
            }
        }
    }
}
impl ops::Mul<Led<f64>> for Led<f64> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match self {
            Led::Single(v) => match other {
                Led::Single(o_v) => Led::Single(v * o_v),
                _ => unimplemented!(),
            }
            Led::Rgb(r,g,b) => match other {
                Led::Rgb(o_r, o_g, o_b) => Led::Rgb(r * o_r, g * o_g, b * o_b),
                _ => unimplemented!(),
            }
        }
    }
}
impl ops::Div<Led<f64>> for Led<f64> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match self {
            Led::Single(v) => match other {
                Led::Single(o_v) => Led::Single(v / o_v),
                _ => unimplemented!(),
            }
            Led::Rgb(r,g,b) => match other {
                Led::Rgb(o_r, o_g, o_b) => Led::Rgb(r / o_r, g / o_g, b / o_b),
                _ => unimplemented!(),
            }
        }
    }
}
