#![no_std]
#![no_main]

use embedded_can::{self, blocking::Can};
use esp_backtrace as _;
use esp_hal::{self, twai::{self, TwaiConfiguration}, delay::Delay, prelude::*, peripherals::TWAI0, io::IO};
use esp_hal::twai::TwaiMode;
use embedded_can::Frame;
// use embedded_hal::can::Frame;
// use crate::twai::TwaiMode;
use crate::twai::Id;
use crate::nb::block;
use esp_println::println;


// struct CanInst;

// impl Can for CanInst {
//     type Frame;

//     type Error;

//     fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
//         todo!()
//     }

//     fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
//         todo!()
//     }
// }

const CAN_BAUDRATE: twai::BaudRate = twai::BaudRate::B1000K;
const CAN_TX_PIN: i8 = io.pins.gpio4;
const CAN_RX_PIN: i8 = io.pins.gpio5;


#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    
    
    // Use GPIO pins 2 and 3 to connect to the respective pins on the TWAI
    // transceiver.
    let twai_rx_pin = peripherals.GPIO3;
    let twai_tx_pin = peripherals.GPIO2;

    // The speed of the TWAI bus.
    const TWAI_BAUDRATE: twai::BaudRate = BaudRate::B1000K;

    // Begin configuring the TWAI peripheral. The peripheral is in a reset like
    // state that prevents transmission but allows configuration.
    let mut twai_config = twai::TwaiConfiguration::new(
        peripherals.TWAI0,
        twai_rx_pin,
        twai_tx_pin,
        TWAI_BAUDRATE,
        TwaiMode::Normal
    );

    // Partially filter the incoming messages to reduce overhead of receiving
    // undesired messages
    twai_config.set_filter(const { SingleStandardFilter::new(b"xxxxxxxxxx0",
    b"x", [b"xxxxxxxx", b"xxxxxxxx"]) });

    // Start the peripheral. This locks the configuration settings of the
    // peripheral and puts it into operation mode, allowing packets to be sent
    // and received.
    let mut twai = twai_config.start();

    loop {
        // Wait for a frame to be received.
        let frame = block!(twai.receive())?;

        // Transmit the frame back.
        let _result = block!(twai.transmit(&frame))?;
    }
    

    

    esp_println::logger::init_logger_from_env();

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
