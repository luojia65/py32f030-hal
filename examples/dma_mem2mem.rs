#![no_std]
#![no_main]

use defmt::Debug2Format;
use hal::dma::{AnyDma, Burst, Priorities, RepeatMode};
use py32f030_hal::{self as hal, dma::Config, mode::Blocking};

use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default());
    defmt::info!("embassy_dma_mem2mem example!");

    let mut src: [u32; 4] = [1, 2, 3, 4];
    let mut dst: [u32; 4] = [0; 4];

    let mut dma: AnyDma<_, Blocking> = AnyDma::new(p.DMA);

    let [mut channel1, _, _] = dma.split();
    channel1.config(Config::new_mem2mem(
        src.as_mut_ptr() as u32,
        true,
        dst.as_mut_ptr() as u32,
        true,
        Priorities::Low,
        RepeatMode::OneTime(src.len() as u16),
        Burst::World,
    ));

    channel1.start();

    let rst = channel1.wait_complet();

    defmt::info!("rst: {:?}", Debug2Format(&rst));
    defmt::info!("src: {} ", src);
    defmt::info!("dst: {} ", dst);

    loop {
        cortex_m::asm::wfe();
    }
}
