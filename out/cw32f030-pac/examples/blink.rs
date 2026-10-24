//! CW32F030 最小点灯(PC13,若板子 LED 在别的脚改 LED_PIN/端口即可)。
//!
//! cargo build --target thumbv6m-none-eabi --release --features rt --example blink

#![no_std]
#![no_main]

use cw32f030_pac::Peripherals;
// 强制链接 cortex-m-rt(rt 模块只有数据,不引用其符号时 rlib 会被剔除)
use cortex_m_rt as _;

/// LED 所在引脚号(默认 PC13,按板改)
const LED_PIN: u32 = 13;

#[no_mangle]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    // SYSCTRL 写保护键 + GPIOC 时钟使能(AHBEN bit6)
    const KEY: u32 = 0x5A5A << 16;
    p.sysctrl.ahben.write(KEY | (1 << 6));

    let gpioc = p.gpioc;
    let m = 1 << LED_PIN;
    gpioc.analog.modify(|v| v & !m); // 数字模式
    gpioc.dir.modify(|v| v & !m); // 输出(DIR=0)
    gpioc.bsrr.write(m); // 初始高

    loop {
        // 上电默认 HSI(约 48MHz),忙等约 0.2s
        for _ in 0..3_000_000 {
            core::hint::spin_loop();
        }
        gpioc.tog.write(m); // 原子翻转
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
