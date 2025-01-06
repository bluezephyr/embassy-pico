#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_22, Level::Low);

    loop {
        info!("LED On");
        led.set_high();
        Timer::after_millis(500).await;

        info!("LED Off");
        led.set_low();
        Timer::after_millis(500).await;
    }
}
