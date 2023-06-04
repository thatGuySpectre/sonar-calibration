#![no_std]
#![no_main]

use core::cmp::max;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let s0 = pins.d11;
    let s1 = pins.d12;
    let s2 = pins.d13;

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        let mut b0 = false;
        let mut b1 = false;
        let mut b2 = false;
        let mut v0 = 0;
        let mut v1 = 0;
        let mut v2 = 0;

        for _ in 0..1000 {
            b0 |= s0.is_high();
            b1 |= s1.is_high();
            b2 |= s2.is_high();
            v0 = max(a0.analog_read(&mut adc), v0);
            v1 = max(a1.analog_read(&mut adc), v1);
            v2 = max(a2.analog_read(&mut adc), v2);
        }

        ufmt::uwriteln!(&mut serial,
            "{}:{}:{};{}:{}:{}",
            b0 as u8,
            b1 as u8,
            b2 as u8,
            v0,
            v1,
            v2
        );
    }
}
