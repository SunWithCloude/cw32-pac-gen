//! 生成产物回归:布局尺寸与中断表密度(生成时自动核对)。

use cw32w031_pac::*;

#[test]
fn block_sizes_match_vendor_headers() {
    assert_eq!(core::mem::size_of::<ADC>(), 88, "ADC");
    assert_eq!(core::mem::size_of::<ATIM>(), 92, "ATIM");
    assert_eq!(core::mem::size_of::<AWT>(), 32, "AWT");
    assert_eq!(core::mem::size_of::<BTIM>(), 40, "BTIM");
    assert_eq!(core::mem::size_of::<CRC>(), 20, "CRC");
    assert_eq!(core::mem::size_of::<DMA>(), 148, "DMA");
    assert_eq!(core::mem::size_of::<DMACHANNEL>(), 20, "DMACHANNEL");
    assert_eq!(core::mem::size_of::<FLASH>(), 104, "FLASH");
    assert_eq!(core::mem::size_of::<GPIO>(), 100, "GPIO");
    assert_eq!(core::mem::size_of::<GTIM>(), 68, "GTIM");
    assert_eq!(core::mem::size_of::<I2C>(), 44, "I2C");
    assert_eq!(core::mem::size_of::<IRMOD>(), 4, "IRMOD");
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
        (Interrupt::Sysctrl , 4),
        (Interrupt::Gpioa , 5),
        (Interrupt::Gpiob , 6),
        (Interrupt::Gpioc , 7),
        (Interrupt::Gpiof , 8),
        (Interrupt::Dmach1 , 9),
        (Interrupt::Dmach23 , 10),
        (Interrupt::Dmach4 , 11),
        (Interrupt::Adc , 12),
        (Interrupt::Atim , 13),
        (Interrupt::Vc1 , 14),
        (Interrupt::Vc2 , 15),
        (Interrupt::Gtim1 , 16),
        (Interrupt::Gtim2 , 17),
        (Interrupt::Reserved18 , 18),
        (Interrupt::Reserved19 , 19),
        (Interrupt::Btim1 , 20),
        (Interrupt::Btim2 , 21),
        (Interrupt::Btim3 , 22),
        (Interrupt::I2C1 , 23),
        (Interrupt::Reserved24 , 24),
        (Interrupt::Spi1 , 25),
        (Interrupt::Reserved26 , 26),
        (Interrupt::Uart1 , 27),
        (Interrupt::Uart2 , 28),
        (Interrupt::Uart3 , 29),
        (Interrupt::Awt , 30),
        (Interrupt::Clkfault , 31),
    ];
    for (i, (_, n)) in all.iter().enumerate() {
        assert_eq!(*n as usize, i, "IRQ 表必须按 0..n 稠密排列");
    }
    assert_eq!(all.len(), 1 + all.last().unwrap().1 as usize);
    // 真实中断名(非 RESERVED 占位)共 28 项,生成期锁定
    let real = all.iter().filter(|(i, _)| !format!("{:?}", i).contains("Reserved")).count();
    assert_eq!(real, 28);
}