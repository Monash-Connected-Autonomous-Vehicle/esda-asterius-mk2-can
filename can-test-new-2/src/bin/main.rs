#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;
use esp_hal::{
    delay::Delay,
    twai::{self, filter::SingleStandardFilter, EspTwaiFrame, StandardId, TwaiMode},
};
use esp_hal::{peripheral};
use nb::block;
use esp_backtrace as _;

const IS_FIRST_SENDER: bool = true;


#[main]
fn main() -> ! {
    // generator version: 0.3.1
    // esp_log::logger::init();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(esp_hal::Config::default());

    loop {
        println!("Hello world");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
