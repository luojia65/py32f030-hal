#![no_std]
#![no_main]

// use embedded_io::Write;
use defmt::{info, Debug2Format};
use embassy_executor::Spawner;
use embassy_time::Timer;
use heapless::String;
use py32f030_hal::{self as hal, usart::AnyUsart};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default());

    let gpioa = p.GPIOA.split();
    let rx = gpioa.PA10;
    let tx = gpioa.PA9;

    let usart = AnyUsart::new(p.USART1, Some(rx), Some(tx), None, None, Default::default());

    let (mut rx, mut tx) = usart.split();

    info!("usart start...");
    let buf: String<20> = "hello rust\r\n".into();

    let mut rx_buf: [u8; 10] = [0; 10];

    loop {
        // 使用标准接口来发送串口数据
        // let _ = write!(tx, "example for usart\r\n");
        let rst = rx.read(&mut rx_buf).await;
        // let rst = rx.read_with_idle(&mut rx_buf).await;
        // defmt::info!("recv: rst: {:?} {:x}", Debug2Format(&rst), rx_buf);
        // 使用自定义的驱动接口发送串口数据
        let rst = tx.write(buf.as_bytes()).await;

        defmt::info!("send: rst:{} {:x} ", Debug2Format(&rst), buf.as_bytes());
        Timer::after_secs(1).await;
    }
}
