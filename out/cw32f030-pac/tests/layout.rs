//! 生成产物回归:布局尺寸与中断表密度(生成时自动核对)。

use cw32f030_pac::*;

#[test]
fn block_sizes_match_vendor_headers() {
    assert_eq!(core::mem::size_of::<ADC>(), 140, "ADC");
    assert_eq!(core::mem::size_of::<ATIM>(), 92, "ATIM");
    assert_eq!(core::mem::size_of::<AWT>(), 32, "AWT");
    assert_eq!(core::mem::size_of::<BTIM>(), 36, "BTIM");
    assert_eq!(core::mem::size_of::<CRC>(), 16, "CRC");
    assert_eq!(core::mem::size_of::<DMA>(), 180, "DMA");
    assert_eq!(core::mem::size_of::<DMACHANNEL>(), 20, "DMACHANNEL");
    assert_eq!(core::mem::size_of::<FLASH>(), 44, "FLASH");
    assert_eq!(core::mem::size_of::<GPIO>(), 100, "GPIO");
    assert_eq!(core::mem::size_of::<GPIOC>(), 100, "GPIOC");
    assert_eq!(core::mem::size_of::<GPIOF>(), 100, "GPIOF");
    assert_eq!(core::mem::size_of::<GTIM>(), 836, "GTIM");
    assert_eq!(core::mem::size_of::<I2C>(), 44, "I2C");
    assert_eq!(core::mem::size_of::<IWDT>(), 40, "IWDT");
    assert_eq!(core::mem::size_of::<LVD>(), 12, "LVD");
    assert_eq!(core::mem::size_of::<RAM>(), 16, "RAM");
    assert_eq!(core::mem::size_of::<RTC>(), 60, "RTC");
    assert_eq!(core::mem::size_of::<SPI>(), 28, "SPI");
    assert_eq!(core::mem::size_of::<SYSCTRL>(), 120, "SYSCTRL");
    assert_eq!(core::mem::size_of::<UART>(), 56, "UART");
    assert_eq!(core::mem::size_of::<VC>(), 16, "VC");
    assert_eq!(core::mem::size_of::<WWDT>(), 12, "WWDT");
}

#[test]
fn interrupt_table_dense_and_ordered() {
    let all: [(Interrupt, u32); 32] = [
        (Interrupt::Wdt , 0),
        (Interrupt::Lvd , 1),
        (Interrupt::Rtc , 2),
        (Interrupt::Flashram , 3),
        (Interrupt::Rcc , 4),
        (Interrupt::Gpioa , 5),
        (Interrupt::Gpiob , 6),
        (Interrupt::Gpioc , 7),
        (Interrupt::Gpiof , 8),
        (Interrupt::Dmach1 , 9),
        (Interrupt::Dmach23 , 10),
        (Interrupt::Dmach45 , 11),
        (Interrupt::Adc , 12),
        (Interrupt::Atim , 13),
        (Interrupt::Vc1 , 14),
        (Interrupt::Vc2 , 15),
        (Interrupt::Gtim1 , 16),
        (Interrupt::Gtim2 , 17),
        (Interrupt::Gtim3 , 18),
        (Interrupt::Gtim4 , 19),
        (Interrupt::Btim1 , 20),
        (Interrupt::Btim2 , 21),
        (Interrupt::Btim3 , 22),
        (Interrupt::I2C1 , 23),
        (Interrupt::I2C2 , 24),
        (Interrupt::Spi1 , 25),
        (Interrupt::Spi2 , 26),
        (Interrupt::Uart1 , 27),
        (Interrupt::Uart2 , 28),
        (Interrupt::Uart3 , 29),
        (Interrupt::Awt , 30),
        (Interrupt::Fault , 31),
    ];
    for (i, (_, n)) in all.iter().enumerate() {
        assert_eq!(*n as usize, i, "IRQ 表必须按 0..n 稠密排列");
    }
    assert_eq!(all.len(), 1 + all.last().unwrap().1 as usize);
    // 真实中断名(非 RESERVED 占位)共 32 项,生成期锁定
    let real = all.iter().filter(|(i, _)| !format!("{:?}", i).contains("Reserved")).count();
    assert_eq!(real, 32);
}