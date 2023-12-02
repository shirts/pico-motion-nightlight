#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config as NetConfig, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Input, Pull, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::Timer;
use embedded_io_async::Write;
use static_cell::make_static;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
  spawner.spawn(nightlight()).unwrap();
}

// Motion sensor into GPIO 2
// LEDs into into GPIO 14, 15, 16
#[embassy_executor::task]
async fn nightlight() -> ! {
  let p = embassy_rp::init(Default::default());

  let motion = Input::new(p.PIN_2, Pull::Down);
  let mut led14 = Output::new(p.PIN_14, Level::Low);
  let mut led15 = Output::new(p.PIN_15, Level::Low);
  let mut led16 = Output::new(p.PIN_16, Level::Low);

  led14.set_low();
  led15.set_low();
  led16.set_low();

  let sleep_duration = 60 * 5;

  loop {
    if motion.is_high() {
      led14.set_high();
      led15.set_high();
      led16.set_high();
      Timer::after_secs(sleep_duration).await;
      led16.set_low();
      led15.set_low();
      led14.set_low();
    }
  }
}
