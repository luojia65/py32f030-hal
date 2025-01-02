#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use embedded_hal_027::digital::v2::InputPin;
use hal::delay;
use hal::gpio::{Input, PinPullUpDown, PinSpeed};
use py32f030_hal as hal;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("examples: key");

    let p = hal::init(Default::default());

    let gpioa = p.GPIOF.split();

    let key = Input::new(gpioa.PF4_BOOT0, PinPullUpDown::No, PinSpeed::Low);

    loop {
        defmt::info!("key: {}", key.is_low());
        delay::delay_ms(1000);
    }
}
