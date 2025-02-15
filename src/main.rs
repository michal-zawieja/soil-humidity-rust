#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use panic_probe as _;

const TEST_INTERVAL: u64 = 5 * 60 * 1_000;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Program start");

    let peripherials = embassy_rp::init(Default::default());

    let mut led_pin = Output::new(peripherials.PIN_25, Level::Low);
    let humidity_digital_output = Input::new(peripherials.PIN_21, Pull::Up);

    loop {
        if humidity_digital_output.is_high() {
            info!("GP21 is high");
            led_pin.set_low();
        } else {
            info!("GP21 is low");
            led_pin.set_high();
        }

        Timer::after(Duration::from_millis(TEST_INTERVAL)).await;
    }
}

// End of file
