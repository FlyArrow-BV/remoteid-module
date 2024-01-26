//! A "Hello, world!" program that blinks the LED on the IMXRT1020-EVK board.

#![no_main]
#![no_std]

use imxrt_hal as hal;
use imxrt_ral as ral;

// use imxrt1020evk_fcb as _;
use imxrt_rt::entry;
use panic_halt as _;

/// The entry point for this application.
#[entry]
fn main() -> ! {
    // Safety: we're the only code that "owns" the IOMUXC and GPIO1 peripherals.
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let gpio1 = unsafe { ral::gpio::GPIO1::instance() };

    let mut gpio1 = hal::gpio::Port::new(gpio1);
    let pads = hal::iomuxc::into_pads(iomuxc);

    // Configures the pad named "GPIO_11" as a GPIO output.
    let led = gpio1.output(pads.gpio_ad_b0.p11);
    // Turn on the LED.
    led.set();

    loop {}
}
