#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp235x_hal::{pac, Watchdog, entry, Sio, Timer};
use rp235x_hal::clocks::init_clocks_and_plls;
use rp235x_hal::gpio::Pins;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);
    let mut led_pin = pins.gpio13.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(200);

        led_pin.set_low().unwrap();
        timer.delay_ms(200);
    }
}
