#![no_std]
#![no_main]
// we use the panic handler defined in lib.rs
#[allow(unused_imports)]
use template_rp2040::panic as _;

use bsp::entry;
use embedded_hal::digital::OutputPin;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Use a gpio, which has a led connected!
    // If you have a pico (H) you can instead use the onboard led by specifying gpio 25.
    // If you have a pico W(H), the onboard led is connected to the wifi which and is not easily accessable.
    // Therefore you have to connect your own led on any gpio :)
    let mut led_pin = pins.gpio20.into_push_pull_output();

    // let the led blink!
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
