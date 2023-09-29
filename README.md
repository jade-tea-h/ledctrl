# ledctrl

This library is designed to be a modular interface for controlling different kinds of boards.
Any device should be usable after simply implementing the `PinControl` trait.
Button controls and various general light patterns will eventually be implemented.
The plan is for a closure to be passed to a `Controller` which will be continuously run, with a few predefined closures wrapped in an enum.

```rust
let mut my_controller = Controller<Rpi>::new(Led::Single(21))?; // use gpio pin 21
my_controller.set_led(Led::Rgb<led::Dtype>(On,Off,Off));
let mut rgb = [
    Led::Rgb<led::Dtype>(On,Off,Off),
    Led::Rgb<led::Dtype>(Off,On,Off),
    Led::Rgb<led::Dtype>(Off,Off,On),
].iter().cycle();
my_controller.set_pattern(|time_elapsed| { // accepts Duration since last LED change
    if time_elapsed > 1.0 {
        rgb.next()
    }
    None // if None returned no change happens and the timestamp is not reset
})
```

Not exactly sure how CLI interaction is going to work, for software pwm the main program needs to be running constantly.
