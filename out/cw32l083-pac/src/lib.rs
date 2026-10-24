//! # cw32l083-pac
//!
//! CW32L083 (Cortex-M0+) 的寄存器访问层,由 cw32-pac-gen 从厂商
//! SDK 头文件生成(布局/偏移/尺寸取自 svd2header 注释),并吸收
//! fan2nd/cw32-hal2 数据层的人工修正。零依赖,只使用 core。
//!
//! 数据来源与生成方式见仓库 THIRD_PARTY.md。

#![no_std]
#![allow(rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_op_in_unsafe_fn)]

// ================= 通用寄存器原语 =================

/// 可读可写寄存器(volatile 读 / 写 / 读改写)
pub struct Rw<T: Copy>(T);

impl<T: Copy> Rw<T> {
    /// 读
    #[inline]
    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(self.0)) }
    }
    /// 写
    #[inline]
    pub fn write(&self, value: T) {
        unsafe { core::ptr::write_volatile(core::ptr::addr_of!(self.0) as *mut T, value) }
    }
    /// 读改写
    #[inline]
    pub fn modify<F: FnOnce(T) -> T>(&self, f: F) {
        let v = self.read();
        self.write(f(v));
    }
}

/// 只读寄存器
pub struct Ro<T: Copy>(T);

impl<T: Copy> Ro<T> {
    /// 读
    #[inline]
    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!(self.0)) }
    }
}

/// 只写寄存器
pub struct Wo<T: Copy>(T);

impl<T: Copy> Wo<T> {
    /// 写
    #[inline]
    pub fn write(&self, value: T) {
        unsafe { core::ptr::write_volatile(core::ptr::addr_of!(self.0) as *mut T, value) }
    }
}


// ================= 外设基地址(厂商头文件注释) =================

/// ADC(ADC)基地址
pub const ADC_BASE: usize = 0x40012400;
/// AES(AES)基地址
pub const AES_BASE: usize = 0x40026000;
/// ATIM(ATIM)基地址
pub const ATIM_BASE: usize = 0x40012c00;
/// AUTOTRIM(AUTOTRIM)基地址
pub const AUTOTRIM_BASE: usize = 0x40014c00;
/// BTIM1(BTIM)基地址
pub const BTIM1_BASE: usize = 0x40014800;
/// CRC(CRC)基地址
pub const CRC_BASE: usize = 0x40023000;
/// DMA(DMA)基地址
pub const DMA_BASE: usize = 0x40020000;
/// DMACHANNEL1(DMACHANNEL)基地址
pub const DMACHANNEL1_BASE: usize = 0x40020020;
/// FLASH(FLASH)基地址
pub const FLASH_BASE: usize = 0x40022000;
/// GPIOA(GPIO)基地址
pub const GPIOA_BASE: usize = 0x48000000;
/// GTIM1(GTIM)基地址
pub const GTIM1_BASE: usize = 0x40000400;
/// I2C1(I2C)基地址
pub const I2C1_BASE: usize = 0x40005400;
/// IWDT(IWDT)基地址
pub const IWDT_BASE: usize = 0x40003000;
/// LCD(LCD)基地址
pub const LCD_BASE: usize = 0x40002400;
/// LPTIM(LPTIM)基地址
pub const LPTIM_BASE: usize = 0x40007800;
/// LVD(LVD)基地址
pub const LVD_BASE: usize = 0x40012a80;
/// RAM(RAM)基地址
pub const RAM_BASE: usize = 0x40022400;
/// RTC(RTC)基地址
pub const RTC_BASE: usize = 0x40002800;
/// SPI1(SPI)基地址
pub const SPI1_BASE: usize = 0x40013000;
/// SYSCTRL(SYSCTRL)基地址
pub const SYSCTRL_BASE: usize = 0x40010000;
/// TRNG(TRNG)基地址
pub const TRNG_BASE: usize = 0x40025000;
/// UART1(UART)基地址
pub const UART1_BASE: usize = 0x40013800;
/// VC1(VC)基地址
pub const VC1_BASE: usize = 0x40012a00;
/// WWDT(WWDT)基地址
pub const WWDT_BASE: usize = 0x40002c00;

// ================= 寄存器块 =================

/// `ADC` 寄存器组(生成自厂商头文件)
///
/// - `ADC` 基地址 `0X40012400`
#[allow(dead_code)]
#[repr(C)]
pub struct ADC {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 START(RW)
    pub start: Rw<u32>,
    _pad0: [u32; 1], // 0x0c..0x10
    /// 0x10 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x14 VTH(RW)
    pub vth: Rw<u32>,
    /// 0x18 VTL(RW)
    pub vtl: Rw<u32>,
    /// 0x1C TRIGGER(RW)
    pub trigger: Rw<u32>,
    /// 0x20 RESULT0(RO)
    pub result0: Ro<u32>,
    /// 0x24 RESULT1(RO)
    pub result1: Ro<u32>,
    /// 0x28 RESULT2(RO)
    pub result2: Ro<u32>,
    /// 0x2C RESULT3(RO)
    pub result3: Ro<u32>,
    /// 0x30 RESULT4(RO)
    pub result4: Ro<u32>,
    /// 0x34 RESULT5(RO)
    pub result5: Ro<u32>,
    /// 0x38 RESULT6(RO)
    pub result6: Ro<u32>,
    /// 0x3C RESULT7(RO)
    pub result7: Ro<u32>,
    /// 0x40 RESULTACC(RO)
    pub resultacc: Ro<u32>,
    /// 0x44 IER(RW)
    pub ier: Rw<u32>,
    /// 0x48 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x4C ISR(RO)
    pub isr: Ro<u32>,
    /// 0x50 SQR0(RW)
    pub sqr0: Rw<u32>,
    /// 0x54 SQR1(RW)
    pub sqr1: Rw<u32>,
}

/// `AES` 寄存器组(生成自厂商头文件)
///
/// - `AES` 基地址 `0X40026000`
#[allow(dead_code)]
#[repr(C)]
pub struct AES {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    _pad0: [u32; 3], // 0x04..0x10
    /// 0x10 DATA0(RW)
    pub data0: Rw<u32>,
    /// 0x14 DATA1(RW)
    pub data1: Rw<u32>,
    /// 0x18 DATA2(RW)
    pub data2: Rw<u32>,
    /// 0x1C DATA3(RW)
    pub data3: Rw<u32>,
    /// 0x20 KEY0(WO)
    pub key0: Wo<u32>,
    /// 0x24 KEY1(WO)
    pub key1: Wo<u32>,
    /// 0x28 KEY2(WO)
    pub key2: Wo<u32>,
    /// 0x2C KEY3(WO)
    pub key3: Wo<u32>,
    /// 0x30 KEY4(WO)
    pub key4: Wo<u32>,
    /// 0x34 KEY5(WO)
    pub key5: Wo<u32>,
    /// 0x38 KEY6(WO)
    pub key6: Wo<u32>,
    /// 0x3C KEY7(WO)
    pub key7: Wo<u32>,
}

/// `ATIM` 寄存器组(生成自厂商头文件)
///
/// - `ATIM` 基地址 `0X40012C00`
#[allow(dead_code)]
#[repr(C)]
pub struct ATIM {
    /// 0x00 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x04 CNT(RW)
    pub cnt: Rw<u32>,
    _pad0: [u32; 1], // 0x08..0x0c
    /// 0x0C CR(RW)
    pub cr: Rw<u32>,
    /// 0x10 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x14 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x18 MSCR(RW)
    pub mscr: Rw<u32>,
    /// 0x1C FLTR(RW)
    pub fltr: Rw<u32>,
    /// 0x20 TRIG(RW)
    pub trig: Rw<u32>,
    /// 0x24 CH1CR(RW)
    pub ch1cr: Rw<u32>,
    /// 0x28 CH2CR(RW)
    pub ch2cr: Rw<u32>,
    /// 0x2C CH3CR(RW)
    pub ch3cr: Rw<u32>,
    /// 0x30 DTR(RW)
    pub dtr: Rw<u32>,
    /// 0x34 RCR(RW)
    pub rcr: Rw<u32>,
    _pad1: [u32; 1], // 0x38..0x3c
    /// 0x3C CH1CCRA(RW)
    pub ch1ccra: Rw<u32>,
    /// 0x40 CH1CCRB(RW)
    pub ch1ccrb: Rw<u32>,
    /// 0x44 CH2CCRA(RW)
    pub ch2ccra: Rw<u32>,
    /// 0x48 CH2CCRB(RW)
    pub ch2ccrb: Rw<u32>,
    /// 0x4C CH3CCRA(RW)
    pub ch3ccra: Rw<u32>,
    /// 0x50 CH3CCRB(RW)
    pub ch3ccrb: Rw<u32>,
    /// 0x54 CH4CCR(RW)
    pub ch4ccr: Rw<u32>,
    /// 0x58 CH4CR(RW)
    pub ch4cr: Rw<u32>,
}

/// `AUTOTRIM` 寄存器组(生成自厂商头文件)
///
/// - `AUTOTRIM` 基地址 `0X40014C00`
#[allow(dead_code)]
#[repr(C)]
pub struct AUTOTRIM {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    /// 0x04 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x08 CNT(RO)
    pub cnt: Ro<u32>,
    _pad0: [u32; 1], // 0x0c..0x10
    /// 0x10 IER(RW)
    pub ier: Rw<u32>,
    /// 0x14 ISR(RO)
    pub isr: Ro<u32>,
    _pad1: [u32; 1], // 0x18..0x1c
    /// 0x1C ICR(RW)
    pub icr: Rw<u32>,
    _pad2: [u32; 24], // 0x20..0x80
    /// 0x80 FCAP(RO)
    pub fcap: Ro<u32>,
    /// 0x84 TVAL(RO)
    pub tval: Ro<u32>,
    /// 0x88 FLIM(RO)
    pub flim: Ro<u32>,
}

/// `BTIM` 寄存器组(生成自厂商头文件)
///
/// - `BTIM1` 基地址 `0X40014800`
#[allow(dead_code)]
#[repr(C)]
pub struct BTIM {
    /// 0x00 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x04 CNT(RW)
    pub cnt: Rw<u32>,
    _pad0: [u32; 1], // 0x08..0x0c
    /// 0x0C ACR(RW)
    pub acr: Rw<u32>,
    /// 0x10 BCR(RW)
    pub bcr: Rw<u32>,
    /// 0x14 IER(RW)
    pub ier: Rw<u32>,
    /// 0x18 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x1C ICR(RW)
    pub icr: Rw<u32>,
    /// 0x20 DMA(RW)
    pub dma: Rw<u32>,
    /// 0x24 PSC(RW)
    pub psc: Rw<u32>,
}

/// `CRC` 寄存器组(生成自厂商头文件)
///
/// - `CRC` 基地址 `0X40023000`
#[allow(dead_code)]
#[repr(C)]
pub struct CRC {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    _pad0: [u32; 1], // 0x04..0x08
    /// 0x08 DR(RW)
    pub dr: Rw<u32>,
    /// 0x0C RESULT(RO)
    pub result: Ro<u32>,
    _pad1: [u32; 1], // 尾部到 Size=20
}

/// `DMA` 寄存器组(生成自厂商头文件)
///
/// - `DMA` 基地址 `0X40020000`
#[allow(dead_code)]
#[repr(C)]
pub struct DMA {
    /// 0x00 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x04 ICR(RW)
    pub icr: Rw<u32>,
    _pad0: [u32; 6], // 0x08..0x20
    /// 0x20 CSR1(RW)
    pub csr1: Rw<u32>,
    /// 0x24 CNT1(RW)
    pub cnt1: Rw<u32>,
    /// 0x28 SRCADDR1(RW)
    pub srcaddr1: Rw<u32>,
    /// 0x2C DSTADDR1(RW)
    pub dstaddr1: Rw<u32>,
    /// 0x30 TRIG1(RW)
    pub trig1: Rw<u32>,
    _pad1: [u32; 3], // 0x34..0x40
    /// 0x40 CSR2(RW)
    pub csr2: Rw<u32>,
    /// 0x44 CNT2(RW)
    pub cnt2: Rw<u32>,
    /// 0x48 SRCADDR2(RW)
    pub srcaddr2: Rw<u32>,
    /// 0x4C DSTADDR2(RW)
    pub dstaddr2: Rw<u32>,
    /// 0x50 TRIG2(RW)
    pub trig2: Rw<u32>,
    _pad2: [u32; 3], // 0x54..0x60
    /// 0x60 CSR3(RW)
    pub csr3: Rw<u32>,
    /// 0x64 CNT3(RW)
    pub cnt3: Rw<u32>,
    /// 0x68 SRCADDR3(RW)
    pub srcaddr3: Rw<u32>,
    /// 0x6C DSTADDR3(RW)
    pub dstaddr3: Rw<u32>,
    /// 0x70 TRIG3(RW)
    pub trig3: Rw<u32>,
    _pad3: [u32; 3], // 0x74..0x80
    /// 0x80 CSR4(RW)
    pub csr4: Rw<u32>,
    /// 0x84 CNT4(RW)
    pub cnt4: Rw<u32>,
    /// 0x88 SRCADDR4(RW)
    pub srcaddr4: Rw<u32>,
    /// 0x8C DSTADDR4(RW)
    pub dstaddr4: Rw<u32>,
    /// 0x90 TRIG4(RW)
    pub trig4: Rw<u32>,
    _pad4: [u32; 3], // 0x94..0xa0
    /// 0xA0 CSR5(RW)
    pub csr5: Rw<u32>,
    /// 0xA4 CNT5(RW)
    pub cnt5: Rw<u32>,
    /// 0xA8 SRCADDR5(RW)
    pub srcaddr5: Rw<u32>,
    /// 0xAC DSTADDR5(RW)
    pub dstaddr5: Rw<u32>,
    /// 0xB0 TRIG5(RW)
    pub trig5: Rw<u32>,
}

/// `DMACHANNEL` 寄存器组(生成自厂商头文件)
///
/// - `DMACHANNEL1` 基地址 `0X40020020`
#[allow(dead_code)]
#[repr(C)]
pub struct DMACHANNEL {
    /// 0x00 CSR(RW)
    pub csr: Rw<u32>,
    /// 0x04 CNT(RW)
    pub cnt: Rw<u32>,
    /// 0x08 SRCADDR(RW)
    pub srcaddr: Rw<u32>,
    /// 0x0C DSTADDR(RW)
    pub dstaddr: Rw<u32>,
    /// 0x10 TRIG(RW)
    pub trig: Rw<u32>,
}

/// `FLASH` 寄存器组(生成自厂商头文件)
///
/// - `FLASH` 基地址 `0X40022000`
#[allow(dead_code)]
#[repr(C)]
pub struct FLASH {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 PAGELOCK1(RW)
    pub pagelock1: Rw<u32>,
    /// 0x0C PAGELOCK2(RW)
    pub pagelock2: Rw<u32>,
    /// 0x10 PAGELOCK3(RW)
    pub pagelock3: Rw<u32>,
    /// 0x14 PAGELOCK4(RW)
    pub pagelock4: Rw<u32>,
    _pad0: [u32; 2], // 0x18..0x20
    /// 0x20 IER(RW)
    pub ier: Rw<u32>,
    /// 0x24 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x28 ICR(RW)
    pub icr: Rw<u32>,
    _pad1: [u32; 15], // 尾部到 Size=104
}

/// `GPIO` 寄存器组(生成自厂商头文件)
///
/// - `GPIOA` 基地址 `0X48000000`
#[allow(dead_code)]
#[repr(C)]
pub struct GPIO {
    /// 0x00 DIR(RW)
    pub dir: Rw<u32>,
    /// 0x04 OPENDRAIN(RW)
    pub opendrain: Rw<u32>,
    _pad0: [u32; 1], // 0x08..0x0c
    /// 0x0C PDR(RW)
    pub pdr: Rw<u32>,
    /// 0x10 PUR(RW)
    pub pur: Rw<u32>,
    /// 0x14 AFRH(RW)
    pub afrh: Rw<u32>,
    /// 0x18 AFRL(RW)
    pub afrl: Rw<u32>,
    /// 0x1C ANALOG(RW)
    pub analog: Rw<u32>,
    _pad1: [u32; 1], // 0x20..0x24
    /// 0x24 RISEIE(RW)
    pub riseie: Rw<u32>,
    /// 0x28 FALLIE(RW)
    pub fallie: Rw<u32>,
    /// 0x2C HIGHIE(RW)
    pub highie: Rw<u32>,
    /// 0x30 LOWIE(RW)
    pub lowie: Rw<u32>,
    /// 0x34 ISR(RW)
    pub isr: Rw<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x3C LCKR(RW)
    pub lckr: Rw<u32>,
    /// 0x40 FILTER(RW)
    pub filter: Rw<u32>,
    _pad2: [u32; 3], // 0x44..0x50
    /// 0x50 IDR(RO)
    pub idr: Ro<u32>,
    /// 0x54 ODR(RW)
    pub odr: Rw<u32>,
    /// 0x58 BRR(RW)
    pub brr: Rw<u32>,
    /// 0x5C BSRR(RW)
    pub bsrr: Rw<u32>,
    /// 0x60 TOG(RW)
    pub tog: Rw<u32>,
}

/// `GTIM` 寄存器组(生成自厂商头文件)
///
/// - `GTIM1` 基地址 `0X40000400`
#[allow(dead_code)]
#[repr(C)]
pub struct GTIM {
    _pad0: [u32; 192], // 0x00..0x300
    /// 0x300 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x304 CNT(RW)
    pub cnt: Rw<u32>,
    /// 0x308 CMMR(RW)
    pub cmmr: Rw<u32>,
    /// 0x30C ETR(RW)
    pub etr: Rw<u32>,
    /// 0x310 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x314 IER(RW)
    pub ier: Rw<u32>,
    /// 0x318 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x31C ICR(RW)
    pub icr: Rw<u32>,
    /// 0x320 CCR1(RW)
    pub ccr1: Rw<u32>,
    /// 0x324 CCR2(RW)
    pub ccr2: Rw<u32>,
    /// 0x328 CCR3(RW)
    pub ccr3: Rw<u32>,
    /// 0x32C CCR4(RW)
    pub ccr4: Rw<u32>,
    /// 0x330 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x334 PSC(RW)
    pub psc: Rw<u32>,
    _pad1: [u32; 2], // 0x338..0x340
    /// 0x340 DMA(RW)
    pub dma: Rw<u32>,
}

/// `I2C` 寄存器组(生成自厂商头文件)
///
/// - `I2C1` 基地址 `0X40005400`
#[allow(dead_code)]
#[repr(C)]
pub struct I2C {
    /// 0x00 BRREN(RW)
    pub brren: Rw<u32>,
    /// 0x04 BRR(RW)
    pub brr: Rw<u32>,
    /// 0x08 CR(RW)
    pub cr: Rw<u32>,
    /// 0x0C DR(RW)
    pub dr: Rw<u32>,
    /// 0x10 ADDR0(RW)
    pub addr0: Rw<u32>,
    /// 0x14 STAT(RO)
    pub stat: Ro<u32>,
    _pad0: [u32; 2], // 0x18..0x20
    /// 0x20 ADDR1(RW)
    pub addr1: Rw<u32>,
    /// 0x24 ADDR2(RW)
    pub addr2: Rw<u32>,
    /// 0x28 MATCH(RO)
    pub r#match: Ro<u32>,
}

/// `IWDT` 寄存器组(生成自厂商头文件)
///
/// - `IWDT` 基地址 `0X40003000`
#[allow(dead_code)]
#[repr(C)]
pub struct IWDT {
    /// 0x00 KR(WO)
    pub kr: Wo<u32>,
    /// 0x04 CR(RW)
    pub cr: Rw<u32>,
    /// 0x08 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x0C SR(RW)
    pub sr: Rw<u32>,
    /// 0x10 WINR(RW)
    pub winr: Rw<u32>,
    _pad0: [u32; 4], // 0x14..0x24
    /// 0x24 CNT(RO)
    pub cnt: Ro<u32>,
}

/// `LCD` 寄存器组(生成自厂商头文件)
///
/// - `LCD` 基地址 `0X40002400`
#[allow(dead_code)]
#[repr(C)]
pub struct LCD {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 INTCLR(RW)
    pub intclr: Rw<u32>,
    /// 0x0C PINEN1(RW)
    pub pinen1: Rw<u32>,
    /// 0x10 PINEN2(RW)
    pub pinen2: Rw<u32>,
    _pad0: [u32; 11], // 0x14..0x40
    /// 0x40 RAM0(RW)
    pub ram0: Rw<u32>,
    /// 0x44 RAM1(RW)
    pub ram1: Rw<u32>,
    /// 0x48 RAM2(RW)
    pub ram2: Rw<u32>,
    /// 0x4C RAM3(RW)
    pub ram3: Rw<u32>,
    /// 0x50 RAM4(RW)
    pub ram4: Rw<u32>,
    /// 0x54 RAM5(RW)
    pub ram5: Rw<u32>,
    /// 0x58 RAM6(RW)
    pub ram6: Rw<u32>,
    /// 0x5C RAM7(RW)
    pub ram7: Rw<u32>,
    /// 0x60 RAM8(RW)
    pub ram8: Rw<u32>,
    /// 0x64 RAM9(RW)
    pub ram9: Rw<u32>,
    /// 0x68 RAM10(RW)
    pub ram10: Rw<u32>,
    /// 0x6C RAM11(RW)
    pub ram11: Rw<u32>,
    /// 0x70 RAM12(RW)
    pub ram12: Rw<u32>,
    /// 0x74 RAM13(RW)
    pub ram13: Rw<u32>,
}

/// `LPTIM` 寄存器组(生成自厂商头文件)
///
/// - `LPTIM` 基地址 `0X40007800`
#[allow(dead_code)]
#[repr(C)]
pub struct LPTIM {
    /// 0x00 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x04 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x08 IER(RW)
    pub ier: Rw<u32>,
    /// 0x0C CFGR(RW)
    pub cfgr: Rw<u32>,
    /// 0x10 CR(RW)
    pub cr: Rw<u32>,
    /// 0x14 CMP(RW)
    pub cmp: Rw<u32>,
    /// 0x18 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x1C CNT(RO)
    pub cnt: Ro<u32>,
}

/// `LVD` 寄存器组(生成自厂商头文件)
///
/// - `LVD` 基地址 `0X40012A80`
#[allow(dead_code)]
#[repr(C)]
pub struct LVD {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 SR(RW)
    pub sr: Rw<u32>,
}

/// `RAM` 寄存器组(生成自厂商头文件)
///
/// - `RAM` 基地址 `0X40022400`
#[allow(dead_code)]
#[repr(C)]
pub struct RAM {
    /// 0x00 IER(RW)
    pub ier: Rw<u32>,
    /// 0x04 ADDR(RO)
    pub addr: Ro<u32>,
    /// 0x08 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x0C ICR(RW)
    pub icr: Rw<u32>,
}

/// `RTC` 寄存器组(生成自厂商头文件)
///
/// - `RTC` 基地址 `0X40002800`
#[allow(dead_code)]
#[repr(C)]
pub struct RTC {
    /// 0x00 KEY(WO)
    pub key: Wo<u32>,
    /// 0x04 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x08 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x0C CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x10 COMPEN(RW)
    pub compen: Rw<u32>,
    /// 0x14 DATE(RW)
    pub date: Rw<u32>,
    /// 0x18 TIME(RW)
    pub time: Rw<u32>,
    /// 0x1C ALARMA(RW)
    pub alarma: Rw<u32>,
    /// 0x20 ALARMB(RW)
    pub alarmb: Rw<u32>,
    /// 0x24 TAMPDATE(RO)
    pub tampdate: Ro<u32>,
    /// 0x28 TAMPTIME(RO)
    pub tamptime: Ro<u32>,
    /// 0x2C AWTARR(RW)
    pub awtarr: Rw<u32>,
    /// 0x30 IER(RW)
    pub ier: Rw<u32>,
    /// 0x34 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
}

/// `SPI` 寄存器组(生成自厂商头文件)
///
/// - `SPI1` 基地址 `0X40013000`
#[allow(dead_code)]
#[repr(C)]
pub struct SPI {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 IER(RW)
    pub ier: Rw<u32>,
    /// 0x08 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x0C SSI(RW)
    pub ssi: Rw<u32>,
    /// 0x10 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x14 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x18 DR(RW)
    pub dr: Rw<u32>,
}

/// `SYSCTRL` 寄存器组(生成自厂商头文件)
///
/// - `SYSCTRL` 基地址 `0X40010000`
#[allow(dead_code)]
#[repr(C)]
pub struct SYSCTRL {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x0C IER(RW)
    pub ier: Rw<u32>,
    /// 0x10 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x14 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x18 HSI(RW)
    pub hsi: Rw<u32>,
    /// 0x1C HSE(RW)
    pub hse: Rw<u32>,
    /// 0x20 LSI(RW)
    pub lsi: Rw<u32>,
    /// 0x24 LSE(RW)
    pub lse: Rw<u32>,
    /// 0x28 PLL(RW)
    pub pll: Rw<u32>,
    /// 0x2C DEBUG(RW)
    pub debug: Rw<u32>,
    /// 0x30 AHBEN(RW)
    pub ahben: Rw<u32>,
    /// 0x34 APBEN2(RW)
    pub apben2: Rw<u32>,
    /// 0x38 APBEN1(RW)
    pub apben1: Rw<u32>,
    _pad0: [u32; 1], // 0x3c..0x40
    /// 0x40 AHBRST(RW)
    pub ahbrst: Rw<u32>,
    /// 0x44 APBRST2(RW)
    pub apbrst2: Rw<u32>,
    /// 0x48 APBRST1(RW)
    pub apbrst1: Rw<u32>,
    /// 0x4C RESETFLAG(RW)
    pub resetflag: Rw<u32>,
    /// 0x50 GTIM1CAP(RW)
    pub gtim1cap: Rw<u32>,
    /// 0x54 GTIM2CAP(RW)
    pub gtim2cap: Rw<u32>,
    /// 0x58 GTIM3CAP(RW)
    pub gtim3cap: Rw<u32>,
    /// 0x5C GTIM4CAP(RW)
    pub gtim4cap: Rw<u32>,
    /// 0x60 ATIMETR(RW)
    pub atimetr: Rw<u32>,
    /// 0x64 GTIMETR(RW)
    pub gtimetr: Rw<u32>,
    _pad1: [u32; 1], // 0x68..0x6c
    /// 0x6C TIMITR(RW)
    pub timitr: Rw<u32>,
    /// 0x70 MCO(RW)
    pub mco: Rw<u32>,
    /// 0x74 IRMOD(RW)
    pub irmod: Rw<u32>,
}

/// `TRNG` 寄存器组(生成自厂商头文件)
///
/// - `TRNG` 基地址 `0X40025000`
#[allow(dead_code)]
#[repr(C)]
pub struct TRNG {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    _pad0: [u32; 1], // 0x08..0x0c
    /// 0x0C DATA0(RO)
    pub data0: Ro<u32>,
    /// 0x10 DATA1(RO)
    pub data1: Ro<u32>,
}

/// `UART` 寄存器组(生成自厂商头文件)
///
/// - `UART1` 基地址 `0X40013800`
#[allow(dead_code)]
#[repr(C)]
pub struct UART {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 IER(RW)
    pub ier: Rw<u32>,
    /// 0x0C BRRI(RW)
    pub brri: Rw<u32>,
    /// 0x10 BRRF(RW)
    pub brrf: Rw<u32>,
    _pad0: [u32; 2], // 0x14..0x1c
    /// 0x1C ISR(RO)
    pub isr: Ro<u32>,
    /// 0x20 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x24 RDR(RO)
    pub rdr: Ro<u32>,
    /// 0x28 TDR(WO)
    pub tdr: Wo<u32>,
    _pad1: [u32; 1], // 0x2c..0x30
    /// 0x30 ADDR(RW)
    pub addr: Rw<u32>,
    /// 0x34 MASK(RW)
    pub mask: Rw<u32>,
}

/// `VC` 寄存器组(生成自厂商头文件)
///
/// - `VC1` 基地址 `0X40012A00`
#[allow(dead_code)]
#[repr(C)]
pub struct VC {
    /// 0x00 DIV(RW)
    pub div: Rw<u32>,
    /// 0x04 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x08 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x0C SR(RW)
    pub sr: Rw<u32>,
}

/// `WWDT` 寄存器组(生成自厂商头文件)
///
/// - `WWDT` 基地址 `0X40002C00`
#[allow(dead_code)]
#[repr(C)]
pub struct WWDT {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 SR(RW)
    pub sr: Rw<u32>,
}

// ================= 中断号 =================
/// 外部中断号(IRQ0..31,厂商 IRQn 枚举)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    /// IRQ0:RESERVED0
    Reserved0 = 0,
    /// IRQ1:RESERVED1
    Reserved1 = 1,
    /// IRQ2:RESERVED2
    Reserved2 = 2,
    /// IRQ3:RESERVED3
    Reserved3 = 3,
    /// IRQ4:RESERVED4
    Reserved4 = 4,
    /// IRQ5:RESERVED5
    Reserved5 = 5,
    /// IRQ6:RESERVED6
    Reserved6 = 6,
    /// IRQ7:RESERVED7
    Reserved7 = 7,
    /// IRQ8:RESERVED8
    Reserved8 = 8,
    /// IRQ9:RESERVED9
    Reserved9 = 9,
    /// IRQ10:RESERVED10
    Reserved10 = 10,
    /// IRQ11:RESERVED11
    Reserved11 = 11,
    /// IRQ12:RESERVED12
    Reserved12 = 12,
    /// IRQ13:RESERVED13
    Reserved13 = 13,
    /// IRQ14:RESERVED14
    Reserved14 = 14,
    /// IRQ15:RESERVED15
    Reserved15 = 15,
    /// IRQ16:RESERVED16
    Reserved16 = 16,
    /// IRQ17:RESERVED17
    Reserved17 = 17,
    /// IRQ18:RESERVED18
    Reserved18 = 18,
    /// IRQ19:RESERVED19
    Reserved19 = 19,
    /// IRQ20:RESERVED20
    Reserved20 = 20,
    /// IRQ21:RESERVED21
    Reserved21 = 21,
    /// IRQ22:RESERVED22
    Reserved22 = 22,
    /// IRQ23:RESERVED23
    Reserved23 = 23,
    /// IRQ24:RESERVED24
    Reserved24 = 24,
    /// IRQ25:RESERVED25
    Reserved25 = 25,
    /// IRQ26:RESERVED26
    Reserved26 = 26,
    /// IRQ27:RESERVED27
    Reserved27 = 27,
    /// IRQ28:RESERVED28
    Reserved28 = 28,
    /// IRQ29:RESERVED29
    Reserved29 = 29,
    /// IRQ30:RESERVED30
    Reserved30 = 30,
    /// IRQ31:RESERVED31
    Reserved31 = 31,
}

impl Interrupt {
    /// 在 NVIC 中使能该中断
    pub fn enable(self) {
        let irq = self as u32;
        unsafe { core::ptr::write_volatile(0xE000_E100 as *mut u32, 1 << irq); }
    }
    /// 在 NVIC 中禁用该中断
    pub fn disable(self) {
        let irq = self as u32;
        unsafe { core::ptr::write_volatile(0xE000_E180 as *mut u32, 1 << irq); }
    }
    /// 置为挂起
    pub fn pend(self) {
        let irq = self as u32;
        unsafe { core::ptr::write_volatile(0xE000_E200 as *mut u32, 1 << irq); }
    }
    /// 清除挂起
    pub fn unpend(self) {
        let irq = self as u32;
        unsafe { core::ptr::write_volatile(0xE000_E280 as *mut u32, 1 << irq); }
    }
}

// ================= typelevel 中断(供 HAL / embassy 绑定) =================

/// typelevel 中断标记类型(单源生成:标记 + SYMBOLS + rt 向量表)。
pub mod interrupt {
    /// typelevel 中断号契约
    ///
    /// # Safety
    ///
    /// 实现者必须保证 `NUMBER` 与向量槽位一致(本模块由生成器保证)。
    pub unsafe trait Interrupt {
        /// IRQ 编号
        const NUMBER: u16;
        /// IRQ 编号(便捷访问)
        fn number() -> u16 {
            Self::NUMBER
        }
    }

    #[allow(non_camel_case_types)]
    macro_rules! typelevel_irqs {
        ($( $(#[$meta:meta])* $name:ident = $num:expr; )*) => {
            $(
                $(#[$meta])*
                #[allow(non_camel_case_types)]
                pub struct $name;
                unsafe impl Interrupt for $name { const NUMBER: u16 = $num; }
            )*

            /// (向量符号名, IRQ 编号)表,与上方标记类型同源生成,
            /// 供测试与 build.rs 的 device.x 名单交叉校验。
            pub const SYMBOLS: &[(&str, u16)] = &[ $( (stringify!($name), $num) ),* ];

            /// 向量表生成(feature `rt`,cortex-m-rt `device` 模式)。
            /// `__INTERRUPTS` 位于 `.vector_table.interrupts` 段
            /// (ORIGIN(FLASH)+0x40,即 IRQ0 槽位)。
            #[cfg(all(feature = "rt", target_arch = "arm", target_os = "none"))]
            #[doc(hidden)]
            pub mod vector {
                /// 向量表项(Cortex-M ABI:函数指针或 0)
                #[repr(C)]
                pub union Vector {
                    /// 中断服务程序
                    pub handler: unsafe extern "C" fn(),
                    /// 保留槽位(填 0)
                    pub reserved: usize,
                }

                const IRQ_COUNT: usize = [$( stringify!($name) ),*].len();

                extern "C" {
                    $( fn $name(); )*
                }

                #[link_section = ".vector_table.interrupts"]
                #[no_mangle]
                pub static __INTERRUPTS: [Vector; IRQ_COUNT] = [
                    $( Vector { handler: $name } ),*
                ];
            }
        };
    }

    typelevel_irqs! {
        /// IRQ0:向量符号 `RESERVED0`
        RESERVED0 = 0;
        /// IRQ1:向量符号 `RESERVED1`
        RESERVED1 = 1;
        /// IRQ2:向量符号 `RESERVED2`
        RESERVED2 = 2;
        /// IRQ3:向量符号 `RESERVED3`
        RESERVED3 = 3;
        /// IRQ4:向量符号 `RESERVED4`
        RESERVED4 = 4;
        /// IRQ5:向量符号 `RESERVED5`
        RESERVED5 = 5;
        /// IRQ6:向量符号 `RESERVED6`
        RESERVED6 = 6;
        /// IRQ7:向量符号 `RESERVED7`
        RESERVED7 = 7;
        /// IRQ8:向量符号 `RESERVED8`
        RESERVED8 = 8;
        /// IRQ9:向量符号 `RESERVED9`
        RESERVED9 = 9;
        /// IRQ10:向量符号 `RESERVED10`
        RESERVED10 = 10;
        /// IRQ11:向量符号 `RESERVED11`
        RESERVED11 = 11;
        /// IRQ12:向量符号 `RESERVED12`
        RESERVED12 = 12;
        /// IRQ13:向量符号 `RESERVED13`
        RESERVED13 = 13;
        /// IRQ14:向量符号 `RESERVED14`
        RESERVED14 = 14;
        /// IRQ15:向量符号 `RESERVED15`
        RESERVED15 = 15;
        /// IRQ16:向量符号 `RESERVED16`
        RESERVED16 = 16;
        /// IRQ17:向量符号 `RESERVED17`
        RESERVED17 = 17;
        /// IRQ18:向量符号 `RESERVED18`
        RESERVED18 = 18;
        /// IRQ19:向量符号 `RESERVED19`
        RESERVED19 = 19;
        /// IRQ20:向量符号 `RESERVED20`
        RESERVED20 = 20;
        /// IRQ21:向量符号 `RESERVED21`
        RESERVED21 = 21;
        /// IRQ22:向量符号 `RESERVED22`
        RESERVED22 = 22;
        /// IRQ23:向量符号 `RESERVED23`
        RESERVED23 = 23;
        /// IRQ24:向量符号 `RESERVED24`
        RESERVED24 = 24;
        /// IRQ25:向量符号 `RESERVED25`
        RESERVED25 = 25;
        /// IRQ26:向量符号 `RESERVED26`
        RESERVED26 = 26;
        /// IRQ27:向量符号 `RESERVED27`
        RESERVED27 = 27;
        /// IRQ28:向量符号 `RESERVED28`
        RESERVED28 = 28;
        /// IRQ29:向量符号 `RESERVED29`
        RESERVED29 = 29;
        /// IRQ30:向量符号 `RESERVED30`
        RESERVED30 = 30;
        /// IRQ31:向量符号 `RESERVED31`
        RESERVED31 = 31;
    }
}

/// 外设中断向量表的稳定入口(实现经 `interrupt::vector` 单源生成)。
#[cfg(all(feature = "rt", target_arch = "arm", target_os = "none"))]
pub mod rt {
    #[allow(unused_imports)]
    pub use crate::interrupt::vector::{__INTERRUPTS, Vector};
}


// ================= 外设单例 =================
/// 芯片外设单例。[`Peripherals::take`] 只能成功一次。
pub struct Peripherals {
    /// ADC
    pub adc: &'static ADC,
    /// AES
    pub aes: &'static AES,
    /// ATIM
    pub atim: &'static ATIM,
    /// AUTOTRIM
    pub autotrim: &'static AUTOTRIM,
    /// BTIM1
    pub btim1: &'static BTIM,
    /// CRC
    pub crc: &'static CRC,
    /// DMA
    pub dma: &'static DMA,
    /// DMACHANNEL1
    pub dmachannel1: &'static DMACHANNEL,
    /// FLASH
    pub flash: &'static FLASH,
    /// GPIOA
    pub gpioa: &'static GPIO,
    /// GTIM1
    pub gtim1: &'static GTIM,
    /// I2C1
    pub i2_c1: &'static I2C,
    /// IWDT
    pub iwdt: &'static IWDT,
    /// LCD
    pub lcd: &'static LCD,
    /// LPTIM
    pub lptim: &'static LPTIM,
    /// LVD
    pub lvd: &'static LVD,
    /// RAM
    pub ram: &'static RAM,
    /// RTC
    pub rtc: &'static RTC,
    /// SPI1
    pub spi1: &'static SPI,
    /// SYSCTRL
    pub sysctrl: &'static SYSCTRL,
    /// TRNG
    pub trng: &'static TRNG,
    /// UART1
    pub uart1: &'static UART,
    /// VC1
    pub vc1: &'static VC,
    /// WWDT
    pub wwdt: &'static WWDT,
}

impl Peripherals {
    /// 获取外设单例;第二次及以后调用返回 `None`。
    pub fn take() -> Option<Self> {
        static mut TAKEN: bool = false;
        // SAFETY: 单核裸机环境;约定首次调用发生在使能中断之前。
        unsafe {
            if TAKEN { None } else { TAKEN = true; Some(Self::steal()) }
        }
    }

    /// 无视单例直接取得外设引用。
    ///
    /// # Safety
    /// 调用方需自行保证不出现别名所有权冲突。
    pub unsafe fn steal() -> Self {
        Peripherals {
            adc: unsafe { &*(ADC_BASE as *const ADC) },
            aes: unsafe { &*(AES_BASE as *const AES) },
            atim: unsafe { &*(ATIM_BASE as *const ATIM) },
            autotrim: unsafe { &*(AUTOTRIM_BASE as *const AUTOTRIM) },
            btim1: unsafe { &*(BTIM1_BASE as *const BTIM) },
            crc: unsafe { &*(CRC_BASE as *const CRC) },
            dma: unsafe { &*(DMA_BASE as *const DMA) },
            dmachannel1: unsafe { &*(DMACHANNEL1_BASE as *const DMACHANNEL) },
            flash: unsafe { &*(FLASH_BASE as *const FLASH) },
            gpioa: unsafe { &*(GPIOA_BASE as *const GPIO) },
            gtim1: unsafe { &*(GTIM1_BASE as *const GTIM) },
            i2_c1: unsafe { &*(I2C1_BASE as *const I2C) },
            iwdt: unsafe { &*(IWDT_BASE as *const IWDT) },
            lcd: unsafe { &*(LCD_BASE as *const LCD) },
            lptim: unsafe { &*(LPTIM_BASE as *const LPTIM) },
            lvd: unsafe { &*(LVD_BASE as *const LVD) },
            ram: unsafe { &*(RAM_BASE as *const RAM) },
            rtc: unsafe { &*(RTC_BASE as *const RTC) },
            spi1: unsafe { &*(SPI1_BASE as *const SPI) },
            sysctrl: unsafe { &*(SYSCTRL_BASE as *const SYSCTRL) },
            trng: unsafe { &*(TRNG_BASE as *const TRNG) },
            uart1: unsafe { &*(UART1_BASE as *const UART) },
            vc1: unsafe { &*(VC1_BASE as *const VC) },
            wwdt: unsafe { &*(WWDT_BASE as *const WWDT) },
        }
    }
}

// ================= 布局静态断言(厂商头 Size 注释) =================
const _: () = {
    assert!(core::mem::size_of::<ADC>() == 88,
        "ADC 布局错误:厂商头文件 Size = 88");
    assert!(core::mem::size_of::<AES>() == 64,
        "AES 布局错误:厂商头文件 Size = 64");
    assert!(core::mem::size_of::<ATIM>() == 92,
        "ATIM 布局错误:厂商头文件 Size = 92");
    assert!(core::mem::size_of::<AUTOTRIM>() == 140,
        "AUTOTRIM 布局错误:厂商头文件 Size = 140");
    assert!(core::mem::size_of::<BTIM>() == 40,
        "BTIM 布局错误:厂商头文件 Size = 40");
    assert!(core::mem::size_of::<CRC>() == 20,
        "CRC 布局错误:厂商头文件 Size = 20");
    assert!(core::mem::size_of::<DMA>() == 180,
        "DMA 布局错误:厂商头文件 Size = 180");
    assert!(core::mem::size_of::<DMACHANNEL>() == 20,
        "DMACHANNEL 布局错误:厂商头文件 Size = 20");
    assert!(core::mem::size_of::<FLASH>() == 104,
        "FLASH 布局错误:厂商头文件 Size = 104");
    assert!(core::mem::size_of::<GPIO>() == 100,
        "GPIO 布局错误:厂商头文件 Size = 100");
    assert!(core::mem::size_of::<GTIM>() == 836,
        "GTIM 布局错误:厂商头文件 Size = 836");
    assert!(core::mem::size_of::<I2C>() == 44,
        "I2C 布局错误:厂商头文件 Size = 44");
    assert!(core::mem::size_of::<IWDT>() == 40,
        "IWDT 布局错误:厂商头文件 Size = 40");
    assert!(core::mem::size_of::<LCD>() == 120,
        "LCD 布局错误:厂商头文件 Size = 120");
    assert!(core::mem::size_of::<LPTIM>() == 32,
        "LPTIM 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<LVD>() == 12,
        "LVD 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<RAM>() == 16,
        "RAM 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<RTC>() == 60,
        "RTC 布局错误:厂商头文件 Size = 60");
    assert!(core::mem::size_of::<SPI>() == 28,
        "SPI 布局错误:厂商头文件 Size = 28");
    assert!(core::mem::size_of::<SYSCTRL>() == 120,
        "SYSCTRL 布局错误:厂商头文件 Size = 120");
    assert!(core::mem::size_of::<TRNG>() == 20,
        "TRNG 布局错误:厂商头文件 Size = 20");
    assert!(core::mem::size_of::<UART>() == 56,
        "UART 布局错误:厂商头文件 Size = 56");
    assert!(core::mem::size_of::<VC>() == 16,
        "VC 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<WWDT>() == 12,
        "WWDT 布局错误:厂商头文件 Size = 12");
};