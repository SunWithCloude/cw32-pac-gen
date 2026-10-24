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
        (Interrupt::Reserved0 , 0),
        (Interrupt::Reserved1 , 1),
        (Interrupt::Reserved2 , 2),
        (Interrupt::Reserved3 , 3),
        (Interrupt::Reserved4 , 4),
        (Interrupt::Reserved5 , 5),
        (Interrupt::Reserved6 , 6),
        (Interrupt::Reserved7 , 7),
        (Interrupt::Reserved8 , 8),
        (Interrupt::Reserved9 , 9),
        (Interrupt::Reserved10 , 10),
        (Interrupt::Reserved11 , 11),
        (Interrupt::Reserved12 , 12),
        (Interrupt::Reserved13 , 13),
        (Interrupt::Reserved14 , 14),
        (Interrupt::Reserved15 , 15),
        (Interrupt::Reserved16 , 16),
        (Interrupt::Reserved17 , 17),
        (Interrupt::Reserved18 , 18),
        (Interrupt::Reserved19 , 19),
        (Interrupt::Reserved20 , 20),
        (Interrupt::Reserved21 , 21),
        (Interrupt::Reserved22 , 22),
        (Interrupt::Reserved23 , 23),
        (Interrupt::Reserved24 , 24),
        (Interrupt::Reserved25 , 25),
        (Interrupt::Reserved26 , 26),
        (Interrupt::Reserved27 , 27),
        (Interrupt::Reserved28 , 28),
        (Interrupt::Reserved29 , 29),
        (Interrupt::Reserved30 , 30),
        (Interrupt::Reserved31 , 31),
    ];
    for (i, (_, n)) in all.iter().enumerate() {
        assert_eq!(*n as usize, i, "IRQ 表必须按 0..n 稠密排列");
    }
    assert_eq!(all.len(), 1 + all.last().unwrap().1 as usize);
}