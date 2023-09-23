struct Rgb { r: u8, g: u8, b: u8 }

enum CodeType {
    Percent(Rgb),
    U8(Rgb),
    Hex(String),
}

enum State {
    Pwm { r: f64, g: f64, b: f64 }
}

pub trait RgbControl<E> {
    fn set_light(&self, )
}
