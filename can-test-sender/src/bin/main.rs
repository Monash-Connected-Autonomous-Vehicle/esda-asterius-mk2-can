#![no_std]
#![no_main]

// use embedded_can::{self, blocking::Can};
use esp_backtrace as _;
use esp_hal::{self, delay::Delay, esp_riscv_rt::entry, gpio::{GpioPin, Input, Io, Level, Output, Pull}, peripheral, twai::{self, filter::SingleStandardFilter, EspTwaiFrame, StandardId, TwaiMode}
};
use nb::block;
use esp_println::println;

const IS_FIRST_SENDER: bool = true;

const CAN_BAUDRATE: esp_hal::twai::BaudRate = esp_hal::twai::BaudRate::B1000K;

#[entry]
fn main() -> ! {

    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    // let (rx_pin, tx_pin) = peripherals.GPIO2.split();
    let tx_pin = peripherals.GPIO4;
    let rx_pin = peripherals.GPIO5; 

    const TWAI_BAUDRATE: twai::BaudRate = twai::BaudRate::B1000K;

    // !!! Use `new` when using a transceiver. `new_no_transceiver` sets TX to open-drain
    // Self-testing also works using the regular `new` function.

    // Begin configuring the TWAI peripheral. The peripheral is in a reset like
    // state that prevents transmission but allows configuration.
    // For self-testing use `SelfTest` mode of the TWAI peripheral.
    let mut twai_config = twai::TwaiConfiguration::new_no_transceiver(
        peripherals.TWAI0,
        rx_pin,
        tx_pin,
        TWAI_BAUDRATE,
        TwaiMode::Normal,
    ).into_async();

    // Partially filter the incoming messages to reduce overhead of receiving
    // undesired messages. Note that due to how the hardware filters messages,
    // standard ids and extended ids may both match a filter. Frame ids should
    // be explicitly checked in the application instead of fully relying on
    // these partial acceptance filters to exactly match.
    // A filter that matches StandardId::ZERO.
    twai_config.set_filter(
        const { SingleStandardFilter::new(b"xxxxxxxxxx1", b"x", [b"xxxxxxxx", b"xxxxxxxx"]) },
    );

    // Start the peripheral. This locks the configuration settings of the peripheral
    // and puts it into operation mode, allowing packets to be sent and
    // received.
    let mut twai = twai_config.start();

    if IS_FIRST_SENDER {
        // Send a frame to the other ESP
        // Use `new_self_reception` if you want to use self-testing.
        let frame = EspTwaiFrame::new(StandardId::ZERO, &[1, 2, 3]).unwrap();
        twai.transmit(&frame).unwrap();
        println!("Sent a frame");
    }

    let delay = Delay::new();
    loop {

        println!("Hello world");
        // Wait for a frame to be received.
        // let frame = block!(twai.receive()).unwrap();
 
        // println!("Received a frame: {frame:?}");
        delay.delay_millis(250);

        let frame = EspTwaiFrame::new(StandardId::ZERO, &[1, 2, 3]).unwrap();
        // Transmit a new frame back to the other ESP
        twai.transmit(&frame);
        println!("Sent a frame {:?}", frame);


    }
    
    // loop{
        
    // }
    
}

