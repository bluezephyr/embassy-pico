#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Duration;
use {defmt_rtt as _, panic_probe as _};

// Define a channel to communicate between the tasks
static CHANNEL: Channel<ThreadModeRawMutex, u32, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    // Init peripherals
    let p = embassy_rp::init(Default::default());
    let button = Input::new(p.PIN_0, Pull::Up);
    let led = Output::new(p.PIN_22, Level::Low);

    // Spawn the tasks
    spawner.spawn(button_task(button)).unwrap();
    spawner.spawn(led_task(led)).unwrap();
}

// Button task
#[embassy_executor::task]
async fn button_task(mut button: Input<'static>) {
    let mut count: u32 = 0;
    loop {
        button.wait_for_high().await;
        button.wait_for_low().await;
        count += 1;
        CHANNEL.send(count).await;
        embassy_time::Timer::after(Duration::from_millis(100)).await;
    }
}

// LED task
#[embassy_executor::task]
async fn led_task(mut led: Output<'static>) {
    loop {
        let count = CHANNEL.receive().await;
        info!("Button presses: {}", count);
        led.toggle();
    }
}
