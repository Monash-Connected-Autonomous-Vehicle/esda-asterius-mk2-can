#![no_std]
#![no_main]

use embedded_can::{self, blocking::Can};
use esp_backtrace as _;
use esp_hal::{self, delay::Delay, gpio::{GpioPin, Input, Io, Level, Output, Pull}, peripheral, peripherals::TWAI0, prelude::*};
use nb::block;

const CAN_BAUDRATE: esp_hal::twai::BaudRate = esp_hal::twai::BaudRate::B1000K;

#[entry]
fn main() -> ! {

    esp_println::logger::init_logger_from_env();
    

    
    loop{
        
    }
    
}
