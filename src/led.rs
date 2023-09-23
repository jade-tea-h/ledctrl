pub enum Led<T> {
    Single(T),
    Rgb(T, T, T),
}

pub enum Dtype {
    On,
    Off,
    Hex(String),
    Percent(u8),
    Cycle(f64),
    U8(u8),
}

impl Dtype {
    pub fn to_duty_cycle(&self) -> f64 {
        match self {
            Dtype::Cycle(val) => *val,
            Dtype::Percent(val) => f64::from(*val) / 100.0,
            Dtype::U8(val) => f64::from(*val) / f64::from(u8::MAX),
            Dtype::Hex(val) => todo!("Can't convert hexcodes yet"),
            _ => unimplemented!(),
        }
    }
}

impl Led<Dtype> {
    pub fn to_duty_cycle(&self) -> Led<f64> {
        match self {
            Led::Single(val) => Led::Single(val.to_duty_cycle()),
            Led::Rgb(r, g, b) => Led::Rgb(r.to_duty_cycle(), g.to_duty_cycle(), b.to_duty_cycle()),
        }
    }
}

// mod color {
//     pub const RED: LedValue = LedValue::Rgb { r: 100, g: 0, b: 0 };
//     pub const GREEN: LedValue = LedValue::Rgb { r: 0, g: 100, b: 0 };
//     pub const BLUE: LedValue = LedValue::Rgb { r: 0, g: 0, b: 100 };
//     pub const PURPLE: LedValue = LedValue::Rgb {
//         r: 50,
//         g: 40,
//         b: 95,
//     };

//     enum RepType {
//         Percentages(LedValue::Rgb),
//         U8(LedValue::Rgb),
//         Hex(String),
//     }

//     fn to_duty_cycle(value: LedValue, divisor: f64) -> CycleValue {
//         match value {
//             White(v) => f64::from(v) / divisor,
//             Rgb(v) => CycleValue::Rgb {
//                 r: f64::from(v.r) / divisor,
//                 g: f64::from(v.g) / divisor,
//                 b: f64::from(v.b) / divisor,
//             },
//         }
//     }

//     impl RepType {
//         fn convert(self) -> CycleValue {
//             match self {
//                 Percentages(v) => return to_duty_cycle(v, 100.0),
//                 U8(v) => return to_duty_cycle(v, 256.0),
//                 Hex(s) => todo!(),
//             }
//         }
//     }
// }

// use rppal::gpio::{self, Gpio, OutputPin};

// pub const FREQUENCY: f64 = 5000.0;

// #[derive(Debug)]
// pub struct Controller {
//     w_pin: OutputPin,
//     r_pin: OutputPin,
//     g_pin: OutputPin,
//     b_pin: OutputPin,
// }

// impl Controller {
//     pub fn new(w_pin: u8, r_pin: u8, g_pin: u8, b_pin: u8) -> gpio::Result<Controller> {
//         let gpio = Gpio::new()?;
//         let w = gpio.get(w_pin)?.into_output();
//         let r = gpio.get(r_pin)?.into_output();
//         let g = gpio.get(g_pin)?.into_output();
//         let b = gpio.get(b_pin)?.into_output();
//         Ok(Controller {
//             w_pin: w,
//             r_pin: r,
//             g_pin: g,
//             b_pin: b,
//         })
//     }

//     pub fn handle_request(&mut self, request: Request) /* -> gpio::Result<Option<Task>> */
//     {
//         match request {
//             Request::Static(color) => self.set_color(color.convert()),
//             Request::Blink(color, time_on, time_off) => {
//                 self.set_color(color.convert());
//                 todo!();
//             }
//             Request::Fade(color, time) => {
//                 self.set_color(color.convert());
//                 todo!();
//             }
//             Request::Sequence(colors, time) => {
//                 self.set_color(colors[0].convert());
//                 todo!();
//             }
//             Request::BlinkSequence(colors, time_on, time_off) => {
//                 self.set_color(colors[0].convert());
//                 todo!();
//             }
//             Request::FadeSequence(colors, time) => {
//                 self.set_color(colors[0].convert());
//                 todo!();
//             }
//             Request::Clear => self.turn_off(),
//         }
//         Ok(None)
//     }

//     pub fn turn_off(&mut self) {
//         self.w_pin.set_high();
//         self.r_pin.set_high();
//         self.g_pin.set_high();
//         self.b_pin.set_high();
//     }

//     pub fn set_color(&mut self, color: CycleValue) -> gpio::Result<()> {
//         self.turn_off();

//         match color {
//             CycleValue::White(v) => match v {
//                 0 => return Ok(()),
//                 100 => self.w_pin.set_low(),
//                 _ => self.w_pin.set_pwm_frequency(FREQUENCY, color)?,
//             },
//             CycleValue::Rgb { r, g, b } => {
//                 self.r_pin.set_pwm_frequency(FREQUENCY, r)?;
//                 self.g_pin.set_pwm_frequency(FREQUENCY, g)?;
//                 self.b_pin.set_pwm_frequency(FREQUENCY, b)?;
//             }
//         };
//         Ok(())
//     }
// }
