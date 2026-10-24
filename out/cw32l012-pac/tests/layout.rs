//! 生成产物回归:布局尺寸与中断表密度(生成时自动核对)。

use cw32l012_pac::*;

#[test]
fn block_sizes_match_vendor_headers() {
    assert_eq!(core::mem::size_of::<ADC>(), 124, "ADC");
    assert_eq!(core::mem::size_of::<ATIM>(), 116, "ATIM");
    assert_eq!(core::mem::size_of::<BGR>(), 4, "BGR");
    assert_eq!(core::mem::size_of::<BTIM>(), 48, "BTIM");
    assert_eq!(core::mem::size_of::<CORDIC>(), 16, "CORDIC");
    assert_eq!(core::mem::size_of::<CRC>(), 16, "CRC");
    assert_eq!(core::mem::size_of::<DAC>(), 56, "DAC");
    assert_eq!(core::mem::size_of::<DMA>(), 160, "DMA");
    assert_eq!(core::mem::size_of::<DMACHANNEL>(), 32, "DMACHANNEL");
    assert_eq!(core::mem::size_of::<EAU>(), 20, "EAU");
    assert_eq!(core::mem::size_of::<FLASH>(), 116, "FLASH");
    assert_eq!(core::mem::size_of::<GPIO>(), 100, "GPIO");
    assert_eq!(core::mem::size_of::<GTIM>(), 116, "GTIM");
    assert_eq!(core::mem::size_of::<HALLTIM>(), 36, "HALLTIM");
    assert_eq!(core::mem::size_of::<I2C>(), 372, "I2C");
    assert_eq!(core::mem::size_of::<IRMOD>(), 12, "IRMOD");
    assert_eq!(core::mem::size_of::<IWDT>(), 40, "IWDT");
    assert_eq!(core::mem::size_of::<LPTIM>(), 36, "LPTIM");
    assert_eq!(core::mem::size_of::<LVD>(), 12, "LVD");
    assert_eq!(core::mem::size_of::<OPA>(), 8, "OPA");
    assert_eq!(core::mem::size_of::<RAM>(), 16, "RAM");
    assert_eq!(core::mem::size_of::<RTC>(), 92, "RTC");
    assert_eq!(core::mem::size_of::<SPI>(), 32, "SPI");
    assert_eq!(core::mem::size_of::<SYSCTRL>(), 116, "SYSCTRL");
    assert_eq!(core::mem::size_of::<UART>(), 64, "UART");
    assert_eq!(core::mem::size_of::<VC>(), 16, "VC");
    assert_eq!(core::mem::size_of::<VCREF>(), 4, "VCREF");
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
        (Interrupt::Dmach12 , 9),
        (Interrupt::Dmach34 , 10),
        (Interrupt::Cordic , 11),
        (Interrupt::Adc1 , 12),
        (Interrupt::Atim , 13),
        (Interrupt::Vc13 , 14),
        (Interrupt::Vc24 , 15),
        (Interrupt::Gtim1 , 16),
        (Interrupt::Gtim2 , 17),
        (Interrupt::Gtim34 , 18),
        (Interrupt::Lptim , 19),
        (Interrupt::Btim1 , 20),
        (Interrupt::Btim2 , 21),
        (Interrupt::Btim3Halltim , 22),
        (Interrupt::I2C1 , 23),
        (Interrupt::I2C2 , 24),
        (Interrupt::Spi1 , 25),
        (Interrupt::Spi23 , 26),
        (Interrupt::Uart1 , 27),
        (Interrupt::Uart2 , 28),
        (Interrupt::Uart3 , 29),
        (Interrupt::Adc2Dac , 30),
        (Interrupt::Clkfault , 31),
    ];
    for (i, (_, n)) in all.iter().enumerate() {
        assert_eq!(*n as usize, i, "IRQ 表必须按 0..n 稠密排列");
    }
    assert_eq!(all.len(), 1 + all.last().unwrap().1 as usize);
    // 真实中断名(非 RESERVED 占位)共 32 项,生成期锁定
    let real = all.iter().filter(|(i, _)| !format!("{:?}", i).contains("Reserved")).count();
    assert_eq!(real, 32);
}