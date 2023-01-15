#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embedded_hal::{digital::v2::OutputPin, PwmPin};
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    entry, pac,
    sio::Sio,
    watchdog::Watchdog,
};

const LOW: u16 = 0;
const HIGH: u16 = 25000;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led_pin = pins.led.into_push_pull_output();

    let mut pwm_slices = bsp::hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_b;
    channel.output_to(led_pin);

    loop {
        for i in LOW..=HIGH {
            delay.delay_us(20);
            channel.set_duty(i);
        }

        for i in (LOW..=HIGH).rev() {
            delay.delay_us(20);
            channel.set_duty(i);
        }
    }
}

// End of file
