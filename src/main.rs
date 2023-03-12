use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_LED: u8 = 23;
const GPIO_BUTTON: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let mut led_pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    // ボタン(タクトスイッチ)はプルアップ抵抗で接続
    let button_pin = Gpio::new()?.get(GPIO_BUTTON)?.into_input();

    let mut count = 0;
    let mut status: Level = Level::High;
    loop {
        let pre_state = status;
        match button_pin.read() {
            Level::Low => {
                if status == Level::High {
                    count += 1;
                    if count > 3 {
                        status = Level::Low;
                        count = 0;
                    }
                } else {
                    count = 0;
                }
            }
            Level::High => {
                if status == Level::Low {
                    count += 1;
                    if count > 3 {
                        status = Level::High;
                        count = 0;
                    }
                } else {
                    count = 0;
                }
            }
        }

        if pre_state != status {
            match status {
                Level::Low => led_pin.set_high(),
                Level::High => {
                    led_pin.set_low();
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}
