use cortex_m::asm::delay;

pub fn delay_us(us: usize) {
    for _ in 0..us {
        // 16M： 4
        delay(4);
    }
}

pub fn delay_ms(ms: usize) {
    for _ in 0..ms {
        delay_us(1000);
    }
}

pub fn delay_s(s: usize) {
    for _ in 0..s {
        delay_us(1000_1000);
    }
}

#[inline]
pub fn wait_for_true_timeout_block<F>(timeout_tick: usize, f: F) -> Result<(), ()>
where
    F: Fn() -> bool,
{
    for _ in 0..timeout_tick {
        if f() {
            return Ok(());
        }
        cortex_m::asm::delay(1);
    }
    Err(())
}
