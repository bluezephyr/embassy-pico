#![no_std]
#![no_main]

use core::fmt::Write;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{Async, Config as I2cConfig, I2c, InterruptHandler};
use embassy_rp::peripherals::I2C0;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Duration;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, ascii::FONT_7X13_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::String;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};

// Define channels to communicate between the tasks
static BUTTON_CHANNEL: Channel<ThreadModeRawMutex, u32, 1> = Channel::new();
static DISPLAY_CHANNEL: Channel<ThreadModeRawMutex, u32, 1> = Channel::new();

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    // Init peripherals
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_16;
    let scl = p.PIN_17;
    let button = Input::new(p.PIN_0, Pull::Up);
    let led = Output::new(p.PIN_22, Level::Low);
    let i2c = I2c::new_async(p.I2C0, scl, sda, Irqs, I2cConfig::default());
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Spawn the tasks
    spawner.spawn(button_task(button)).unwrap();
    spawner.spawn(led_task(led)).unwrap();
    spawner.spawn(display_task(display)).unwrap();
}

// Button task
#[embassy_executor::task]
async fn button_task(mut button: Input<'static>) {
    let mut count: u32 = 0;
    loop {
        button.wait_for_low().await;
        count += 1;
        BUTTON_CHANNEL.send(count).await;
        DISPLAY_CHANNEL.send(count).await;
        embassy_time::Timer::after(Duration::from_millis(200)).await;
        button.wait_for_high().await;
    }
}

// LED task
#[embassy_executor::task]
async fn led_task(mut led: Output<'static>) {
    loop {
        let count = BUTTON_CHANNEL.receive().await;
        info!("Button presses: {}", count);
        led.toggle();
    }
}

// display task
#[embassy_executor::task]
async fn display_task(
    mut display: Ssd1306<
        I2CInterface<I2c<'static, embassy_rp::peripherals::I2C0, Async>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
) {
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    loop {
        let count = DISPLAY_CHANNEL.receive().await;

        let mut msg: String<64> = String::new();
        core::write!(&mut msg, "Count: {}", count).unwrap();

        let style_small = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let style_big = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
        display.clear(BinaryColor::Off).unwrap();
        Text::with_baseline("Hello button presser!", Point::zero(), style_small, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(msg.as_str(), Point::new(0, 16), style_big, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}
