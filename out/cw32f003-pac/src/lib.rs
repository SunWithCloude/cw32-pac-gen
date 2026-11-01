//! # cw32f003-pac
//!
//! CW32F003 (Cortex-M0+) 的寄存器访问层,由 cw32-pac-gen 从厂商
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
/// ATIM(ATIM)基地址
pub const ATIM_BASE: usize = 0x40012c00;
/// AWT(AWT)基地址
pub const AWT_BASE: usize = 0x40014c00;
/// BTIM1(BTIM)基地址
pub const BTIM1_BASE: usize = 0x40014800;
/// BTIM2(BTIM)基地址
pub const BTIM2_BASE: usize = 0x40014900;
/// BTIM3(BTIM)基地址
pub const BTIM3_BASE: usize = 0x40014a00;
/// CRC(CRC)基地址
pub const CRC_BASE: usize = 0x40023000;
/// FLASH(FLASH)基地址
pub const FLASH_BASE: usize = 0x40022000;
/// GPIOA(GPIO)基地址
pub const GPIOA_BASE: usize = 0x48000000;
/// GPIOB(GPIO)基地址
pub const GPIOB_BASE: usize = 0x48000400;
/// GPIOC(GPIOC)基地址
pub const GPIOC_BASE: usize = 0x48000800;
/// GTIM(GTIM)基地址
pub const GTIM_BASE: usize = 0x40000400;
/// I2C(I2C)基地址
pub const I2C_BASE: usize = 0x40005400;
/// IWDT(IWDT)基地址
pub const IWDT_BASE: usize = 0x40003000;
/// LVD(LVD)基地址
pub const LVD_BASE: usize = 0x40012a80;
/// RAM(RAM)基地址
pub const RAM_BASE: usize = 0x40022400;
/// SPI(SPI)基地址
pub const SPI_BASE: usize = 0x40013000;
/// SYSCTRL(SYSCTRL)基地址
pub const SYSCTRL_BASE: usize = 0x40010000;
/// UART1(UART)基地址
pub const UART1_BASE: usize = 0x40013800;
/// UART2(UART)基地址
pub const UART2_BASE: usize = 0x40004400;
/// VC1(VC)基地址
pub const VC1_BASE: usize = 0x40012a00;
/// VC2(VC)基地址
pub const VC2_BASE: usize = 0x40012a10;
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
    /// 0x0C SQR(RW)
    pub sqr: Rw<u32>,
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
    /// 0x30 RESULTACC(RO)
    pub resultacc: Ro<u32>,
    /// 0x34 IER(RW)
    pub ier: Rw<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x3C ISR(RO)
    pub isr: Ro<u32>,
    /// 0x40 RESULT140(RO)
    pub result140: Ro<u32>,
    /// 0x44 RESULT141(RO)
    pub result141: Ro<u32>,
    /// 0x48 RESULT142(RO)
    pub result142: Ro<u32>,
    /// 0x4C RESULT143(RO)
    pub result143: Ro<u32>,
    _pad0: [u32; 14], // 0x50..0x88
    /// 0x88 ENABLE14(WO)
    pub enable14: Wo<u32>,
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

/// `AWT` 寄存器组(生成自厂商头文件)
///
/// - `AWT` 基地址 `0X40014C00`
#[allow(dead_code)]
#[repr(C)]
pub struct AWT {
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
}

/// `BTIM` 寄存器组(生成自厂商头文件)
///
/// - `BTIM1` 基地址 `0X40014800`
///
/// - `BTIM2` 基地址 `0X40014900`
///
/// - `BTIM3` 基地址 `0X40014A00`
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
    /// 0x08 PAGELOCK(RW)
    pub pagelock: Rw<u32>,
    _pad0: [u32; 5], // 0x0c..0x20
    /// 0x20 IER(RW)
    pub ier: Rw<u32>,
    /// 0x24 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x28 ICR(RW)
    pub icr: Rw<u32>,
}

/// `GPIO` 寄存器组(生成自厂商头文件)
///
/// - `GPIOA` 基地址 `0X48000000`
///
/// - `GPIOB` 基地址 `0X48000400`
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
    _pad1: [u32; 1], // 0x14..0x18
    /// 0x18 AFRL(RW)
    pub afrl: Rw<u32>,
    /// 0x1C ANALOG(RW)
    pub analog: Rw<u32>,
    _pad2: [u32; 1], // 0x20..0x24
    /// 0x24 RISEIE(RW)
    pub riseie: Rw<u32>,
    /// 0x28 FALLIE(RW)
    pub fallie: Rw<u32>,
    /// 0x2C HIGHIE(RW)
    pub highie: Rw<u32>,
    /// 0x30 LOWIE(RW)
    pub lowie: Rw<u32>,
    /// 0x34 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
    _pad3: [u32; 1], // 0x3c..0x40
    /// 0x40 FILTER(RW)
    pub filter: Rw<u32>,
    _pad4: [u32; 3], // 0x44..0x50
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

/// `GPIOC` 寄存器组(生成自厂商头文件)
///
/// - `GPIOC` 基地址 `0X48000800`
#[allow(dead_code)]
#[repr(C)]
pub struct GPIOC {
    /// 0x00 DIR(RW)
    pub dir: Rw<u32>,
    /// 0x04 OPENDRAIN(RW)
    pub opendrain: Rw<u32>,
    _pad0: [u32; 1], // 0x08..0x0c
    /// 0x0C PDR(RW)
    pub pdr: Rw<u32>,
    /// 0x10 PUR(RW)
    pub pur: Rw<u32>,
    _pad1: [u32; 1], // 0x14..0x18
    /// 0x18 AFRL(RW)
    pub afrl: Rw<u32>,
    /// 0x1C ANALOG(RW)
    pub analog: Rw<u32>,
    _pad2: [u32; 1], // 0x20..0x24
    /// 0x24 RISEIE(RW)
    pub riseie: Rw<u32>,
    /// 0x28 FALLIE(RW)
    pub fallie: Rw<u32>,
    /// 0x2C HIGHIE(RW)
    pub highie: Rw<u32>,
    /// 0x30 LOWIE(RW)
    pub lowie: Rw<u32>,
    /// 0x34 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
    _pad3: [u32; 1], // 0x3c..0x40
    /// 0x40 FILTER(RW)
    pub filter: Rw<u32>,
    _pad4: [u32; 3], // 0x44..0x50
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
/// - `GTIM` 基地址 `0X40000400`
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
}

/// `I2C` 寄存器组(生成自厂商头文件)
///
/// - `I2C` 基地址 `0X40005400`
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

/// `SPI` 寄存器组(生成自厂商头文件)
///
/// - `SPI` 基地址 `0X40013000`
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
    /// 0x1C HEX(RW)
    pub hex: Rw<u32>,
    /// 0x20 LSI(RW)
    pub lsi: Rw<u32>,
    _pad0: [u32; 2], // 0x24..0x2c
    /// 0x2C DEBUG(RW)
    pub debug: Rw<u32>,
    /// 0x30 AHBEN(RW)
    pub ahben: Rw<u32>,
    /// 0x34 APBEN2(RW)
    pub apben2: Rw<u32>,
    /// 0x38 APBEN1(RW)
    pub apben1: Rw<u32>,
    _pad1: [u32; 1], // 0x3c..0x40
    /// 0x40 AHBRST(RW)
    pub ahbrst: Rw<u32>,
    /// 0x44 APBRST2(RW)
    pub apbrst2: Rw<u32>,
    /// 0x48 APBRST1(RW)
    pub apbrst1: Rw<u32>,
    /// 0x4C RESETFLAG(RW)
    pub resetflag: Rw<u32>,
    /// 0x50 GTIMCAP(RW)
    pub gtimcap: Rw<u32>,
    _pad2: [u32; 3], // 0x54..0x60
    /// 0x60 ATIMETR(RW)
    pub atimetr: Rw<u32>,
    /// 0x64 GTIMETR(RW)
    pub gtimetr: Rw<u32>,
    _pad3: [u32; 1], // 0x68..0x6c
    /// 0x6C TIMITR(RW)
    pub timitr: Rw<u32>,
    /// 0x70 MCO(RW)
    pub mco: Rw<u32>,
    /// 0x74 IRMOD(RW)
    pub irmod: Rw<u32>,
}

/// `UART` 寄存器组(生成自厂商头文件)
///
/// - `UART1` 基地址 `0X40013800`
///
/// - `UART2` 基地址 `0X40004400`
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
///
/// - `VC2` 基地址 `0X40012A10`
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

// ================= 位段常量 =================
/// `ADC` 位段常量
pub mod adc {
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// MODE[3:1]
        pub const mode_POS: u32 = 1;
        pub const mode_MASK: u32 = 0xe;
        /// BGREN[4:4]
        pub const bgren_POS: u32 = 4;
        pub const bgren_MASK: u32 = 0x10;
        /// TSEN[5:5]
        pub const tsen_POS: u32 = 5;
        pub const tsen_MASK: u32 = 0x20;
        /// REF[7:6]
        pub const r#ref_POS: u32 = 6;
        pub const r#ref_MASK: u32 = 0xc0;
        /// CLK[10:8]
        pub const clk_POS: u32 = 8;
        pub const clk_MASK: u32 = 0x700;
        /// SAM[12:11]
        pub const sam_POS: u32 = 11;
        pub const sam_MASK: u32 = 0x1800;
        /// BUF[13:13]
        pub const buf_POS: u32 = 13;
        pub const buf_MASK: u32 = 0x2000;
        /// BIAS[15:14]
        pub const bias_POS: u32 = 14;
        pub const bias_MASK: u32 = 0xc000;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// CHMUX[3:0]
        pub const chmux_POS: u32 = 0;
        pub const chmux_MASK: u32 = 0xf;
        /// DISCARD[5:5]
        pub const discard_POS: u32 = 5;
        pub const discard_MASK: u32 = 0x20;
        /// ALIGN[6:6]
        pub const align_POS: u32 = 6;
        pub const align_MASK: u32 = 0x40;
        /// WDTCH[11:8]
        pub const wdtch_POS: u32 = 8;
        pub const wdtch_MASK: u32 = 0xf00;
        /// WDTALL[13:13]
        pub const wdtall_POS: u32 = 13;
        pub const wdtall_MASK: u32 = 0x2000;
    }
    /// START 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod start {
        /// START[0:0]
        pub const start_POS: u32 = 0;
        pub const start_MASK: u32 = 0x1;
        /// AUTOSTOP[1:1]
        pub const autostop_POS: u32 = 1;
        pub const autostop_MASK: u32 = 0x2;
    }
    /// SQR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sqr {
        /// SQR0[3:0]
        pub const sqr0_POS: u32 = 0;
        pub const sqr0_MASK: u32 = 0xf;
        /// SQR1[7:4]
        pub const sqr1_POS: u32 = 4;
        pub const sqr1_MASK: u32 = 0xf0;
        /// SQR2[11:8]
        pub const sqr2_POS: u32 = 8;
        pub const sqr2_MASK: u32 = 0xf00;
        /// SQR3[15:12]
        pub const sqr3_POS: u32 = 12;
        pub const sqr3_MASK: u32 = 0xf000;
        /// ENS[17:16]
        pub const ens_POS: u32 = 16;
        pub const ens_MASK: u32 = 0x30000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// CNT[7:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xff;
        /// ACCEN[8:8]
        pub const accen_POS: u32 = 8;
        pub const accen_MASK: u32 = 0x100;
        /// ACCRST[9:9]
        pub const accrst_POS: u32 = 9;
        pub const accrst_MASK: u32 = 0x200;
    }
    /// VTH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod vth {
        /// VTH[11:0]
        pub const vth_POS: u32 = 0;
        pub const vth_MASK: u32 = 0xfff;
    }
    /// VTL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod vtl {
        /// VTL[11:0]
        pub const vtl_POS: u32 = 0;
        pub const vtl_MASK: u32 = 0xfff;
    }
    /// TRIGGER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trigger {
        /// ATIM[0:0]
        pub const atim_POS: u32 = 0;
        pub const atim_MASK: u32 = 0x1;
        /// GTIM[1:1]
        pub const gtim_POS: u32 = 1;
        pub const gtim_MASK: u32 = 0x2;
        /// PA10[2:2]
        pub const pa10_POS: u32 = 2;
        pub const pa10_MASK: u32 = 0x4;
        /// PA32[3:3]
        pub const pa32_POS: u32 = 3;
        pub const pa32_MASK: u32 = 0x8;
        /// PA54[4:4]
        pub const pa54_POS: u32 = 4;
        pub const pa54_MASK: u32 = 0x10;
        /// BTIM1[5:5]
        pub const btim1_POS: u32 = 5;
        pub const btim1_MASK: u32 = 0x20;
        /// BTIM2[6:6]
        pub const btim2_POS: u32 = 6;
        pub const btim2_MASK: u32 = 0x40;
        /// BTIM3[7:7]
        pub const btim3_POS: u32 = 7;
        pub const btim3_MASK: u32 = 0x80;
        /// UART1[8:8]
        pub const uart1_POS: u32 = 8;
        pub const uart1_MASK: u32 = 0x100;
        /// UART2[9:9]
        pub const uart2_POS: u32 = 9;
        pub const uart2_MASK: u32 = 0x200;
        /// PA76[10:10]
        pub const pa76_POS: u32 = 10;
        pub const pa76_MASK: u32 = 0x400;
        /// SPI[11:11]
        pub const spi_POS: u32 = 11;
        pub const spi_MASK: u32 = 0x800;
        /// PB10[12:12]
        pub const pb10_POS: u32 = 12;
        pub const pb10_MASK: u32 = 0x1000;
        /// I2C[13:13]
        pub const i2c_POS: u32 = 13;
        pub const i2c_MASK: u32 = 0x2000;
        /// PB32[14:14]
        pub const pb32_POS: u32 = 14;
        pub const pb32_MASK: u32 = 0x4000;
        /// PB74[15:15]
        pub const pb74_POS: u32 = 15;
        pub const pb74_MASK: u32 = 0x8000;
    }
    /// RESULT0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result0 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result1 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result2 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result3 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULTACC 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod resultacc {
        /// RESULT[23:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffffff;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// EOC[0:0]
        pub const eoc_POS: u32 = 0;
        pub const eoc_MASK: u32 = 0x1;
        /// EOS[1:1]
        pub const eos_POS: u32 = 1;
        pub const eos_MASK: u32 = 0x2;
        /// EOA[2:2]
        pub const eoa_POS: u32 = 2;
        pub const eoa_MASK: u32 = 0x4;
        /// WDTL[3:3]
        pub const wdtl_POS: u32 = 3;
        pub const wdtl_MASK: u32 = 0x8;
        /// WDTH[4:4]
        pub const wdth_POS: u32 = 4;
        pub const wdth_MASK: u32 = 0x10;
        /// WDTR[5:5]
        pub const wdtr_POS: u32 = 5;
        pub const wdtr_MASK: u32 = 0x20;
        /// OVW[6:6]
        pub const ovw_POS: u32 = 6;
        pub const ovw_MASK: u32 = 0x40;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// EOC[0:0]
        pub const eoc_POS: u32 = 0;
        pub const eoc_MASK: u32 = 0x1;
        /// EOS[1:1]
        pub const eos_POS: u32 = 1;
        pub const eos_MASK: u32 = 0x2;
        /// EOA[2:2]
        pub const eoa_POS: u32 = 2;
        pub const eoa_MASK: u32 = 0x4;
        /// WDTL[3:3]
        pub const wdtl_POS: u32 = 3;
        pub const wdtl_MASK: u32 = 0x8;
        /// WDTH[4:4]
        pub const wdth_POS: u32 = 4;
        pub const wdth_MASK: u32 = 0x10;
        /// WDTR[5:5]
        pub const wdtr_POS: u32 = 5;
        pub const wdtr_MASK: u32 = 0x20;
        /// OVW[6:6]
        pub const ovw_POS: u32 = 6;
        pub const ovw_MASK: u32 = 0x40;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// EOC[0:0]
        pub const eoc_POS: u32 = 0;
        pub const eoc_MASK: u32 = 0x1;
        /// EOS[1:1]
        pub const eos_POS: u32 = 1;
        pub const eos_MASK: u32 = 0x2;
        /// EOA[2:2]
        pub const eoa_POS: u32 = 2;
        pub const eoa_MASK: u32 = 0x4;
        /// WDTL[3:3]
        pub const wdtl_POS: u32 = 3;
        pub const wdtl_MASK: u32 = 0x8;
        /// WDTH[4:4]
        pub const wdth_POS: u32 = 4;
        pub const wdth_MASK: u32 = 0x10;
        /// WDTR[5:5]
        pub const wdtr_POS: u32 = 5;
        pub const wdtr_MASK: u32 = 0x20;
        /// OVW[6:6]
        pub const ovw_POS: u32 = 6;
        pub const ovw_MASK: u32 = 0x40;
        /// READY[7:7]
        pub const ready_POS: u32 = 7;
        pub const ready_MASK: u32 = 0x80;
    }
    /// RESULT140 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result140 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT141 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result141 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT142 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result142 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT143 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result143 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// ENABLE14 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod enable14 {
        /// KEY[15:0]
        pub const key_POS: u32 = 0;
        pub const key_MASK: u32 = 0xffff;
    }
}
/// `ATIM` 位段常量
pub mod atim {
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
    }
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// COMP[1:1]
        pub const comp_POS: u32 = 1;
        pub const comp_MASK: u32 = 0x2;
        /// CT[2:2]
        pub const ct_POS: u32 = 2;
        pub const ct_MASK: u32 = 0x4;
        /// PWM2S[3:3]
        pub const pwm2s_POS: u32 = 3;
        pub const pwm2s_MASK: u32 = 0x8;
        /// PRS[6:4]
        pub const prs_POS: u32 = 4;
        pub const prs_MASK: u32 = 0x70;
        /// BUFPEN[7:7]
        pub const bufpen_POS: u32 = 7;
        pub const bufpen_MASK: u32 = 0x80;
        /// UIE[10:10]
        pub const uie_POS: u32 = 10;
        pub const uie_MASK: u32 = 0x400;
        /// MODE[13:12]
        pub const mode_POS: u32 = 12;
        pub const mode_MASK: u32 = 0x3000;
        /// ONESHOT[14:14]
        pub const oneshot_POS: u32 = 14;
        pub const oneshot_MASK: u32 = 0x4000;
        /// OCCS[16:16]
        pub const occs_POS: u32 = 16;
        pub const occs_MASK: u32 = 0x10000;
        /// URS[17:17]
        pub const urs_POS: u32 = 17;
        pub const urs_MASK: u32 = 0x20000;
        /// TIE[19:19]
        pub const tie_POS: u32 = 19;
        pub const tie_MASK: u32 = 0x80000;
        /// BIE[20:20]
        pub const bie_POS: u32 = 20;
        pub const bie_MASK: u32 = 0x100000;
        /// CISA[22:21]
        pub const cisa_POS: u32 = 21;
        pub const cisa_MASK: u32 = 0x600000;
        /// OCCE[23:23]
        pub const occe_POS: u32 = 23;
        pub const occe_MASK: u32 = 0x800000;
        /// TG[24:24]
        pub const tg_POS: u32 = 24;
        pub const tg_MASK: u32 = 0x1000000;
        /// UG[25:25]
        pub const ug_POS: u32 = 25;
        pub const ug_MASK: u32 = 0x2000000;
        /// BG[26:26]
        pub const bg_POS: u32 = 26;
        pub const bg_MASK: u32 = 0x4000000;
        /// DIR[27:27]
        pub const dir_POS: u32 = 27;
        pub const dir_MASK: u32 = 0x8000000;
        /// OVE[28:28]
        pub const ove_POS: u32 = 28;
        pub const ove_MASK: u32 = 0x10000000;
        /// UNDE[29:29]
        pub const unde_POS: u32 = 29;
        pub const unde_MASK: u32 = 0x20000000;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// C1AF[2:2]
        pub const c1af_POS: u32 = 2;
        pub const c1af_MASK: u32 = 0x4;
        /// C2AF[3:3]
        pub const c2af_POS: u32 = 3;
        pub const c2af_MASK: u32 = 0x8;
        /// C3AF[4:4]
        pub const c3af_POS: u32 = 4;
        pub const c3af_MASK: u32 = 0x10;
        /// C1BF[5:5]
        pub const c1bf_POS: u32 = 5;
        pub const c1bf_MASK: u32 = 0x20;
        /// C2BF[6:6]
        pub const c2bf_POS: u32 = 6;
        pub const c2bf_MASK: u32 = 0x40;
        /// C3BF[7:7]
        pub const c3bf_POS: u32 = 7;
        pub const c3bf_MASK: u32 = 0x80;
        /// C1AE[8:8]
        pub const c1ae_POS: u32 = 8;
        pub const c1ae_MASK: u32 = 0x100;
        /// C2AE[9:9]
        pub const c2ae_POS: u32 = 9;
        pub const c2ae_MASK: u32 = 0x200;
        /// C3AE[10:10]
        pub const c3ae_POS: u32 = 10;
        pub const c3ae_MASK: u32 = 0x400;
        /// C1BE[11:11]
        pub const c1be_POS: u32 = 11;
        pub const c1be_MASK: u32 = 0x800;
        /// C2BE[12:12]
        pub const c2be_POS: u32 = 12;
        pub const c2be_MASK: u32 = 0x1000;
        /// C3BE[13:13]
        pub const c3be_POS: u32 = 13;
        pub const c3be_MASK: u32 = 0x2000;
        /// BIF[14:14]
        pub const bif_POS: u32 = 14;
        pub const bif_MASK: u32 = 0x4000;
        /// TIF[15:15]
        pub const tif_POS: u32 = 15;
        pub const tif_MASK: u32 = 0x8000;
        /// OVF[16:16]
        pub const ovf_POS: u32 = 16;
        pub const ovf_MASK: u32 = 0x10000;
        /// UNDF[17:17]
        pub const undf_POS: u32 = 17;
        pub const undf_MASK: u32 = 0x20000;
        /// C4AF[18:18]
        pub const c4af_POS: u32 = 18;
        pub const c4af_MASK: u32 = 0x40000;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// C1AF[2:2]
        pub const c1af_POS: u32 = 2;
        pub const c1af_MASK: u32 = 0x4;
        /// C2AF[3:3]
        pub const c2af_POS: u32 = 3;
        pub const c2af_MASK: u32 = 0x8;
        /// C3AF[4:4]
        pub const c3af_POS: u32 = 4;
        pub const c3af_MASK: u32 = 0x10;
        /// C1BF[5:5]
        pub const c1bf_POS: u32 = 5;
        pub const c1bf_MASK: u32 = 0x20;
        /// C2BF[6:6]
        pub const c2bf_POS: u32 = 6;
        pub const c2bf_MASK: u32 = 0x40;
        /// C3BF[7:7]
        pub const c3bf_POS: u32 = 7;
        pub const c3bf_MASK: u32 = 0x80;
        /// C1AE[8:8]
        pub const c1ae_POS: u32 = 8;
        pub const c1ae_MASK: u32 = 0x100;
        /// C2AE[9:9]
        pub const c2ae_POS: u32 = 9;
        pub const c2ae_MASK: u32 = 0x200;
        /// C3AE[10:10]
        pub const c3ae_POS: u32 = 10;
        pub const c3ae_MASK: u32 = 0x400;
        /// C1BE[11:11]
        pub const c1be_POS: u32 = 11;
        pub const c1be_MASK: u32 = 0x800;
        /// C2BE[12:12]
        pub const c2be_POS: u32 = 12;
        pub const c2be_MASK: u32 = 0x1000;
        /// C3BE[13:13]
        pub const c3be_POS: u32 = 13;
        pub const c3be_MASK: u32 = 0x2000;
        /// BIF[14:14]
        pub const bif_POS: u32 = 14;
        pub const bif_MASK: u32 = 0x4000;
        /// TIF[15:15]
        pub const tif_POS: u32 = 15;
        pub const tif_MASK: u32 = 0x8000;
        /// OVF[16:16]
        pub const ovf_POS: u32 = 16;
        pub const ovf_MASK: u32 = 0x10000;
        /// UNDF[17:17]
        pub const undf_POS: u32 = 17;
        pub const undf_MASK: u32 = 0x20000;
        /// C4AF[18:18]
        pub const c4af_POS: u32 = 18;
        pub const c4af_MASK: u32 = 0x40000;
    }
    /// MSCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mscr {
        /// MMS[2:0]
        pub const mms_POS: u32 = 0;
        pub const mms_MASK: u32 = 0x7;
        /// TS[7:5]
        pub const ts_POS: u32 = 5;
        pub const ts_MASK: u32 = 0xe0;
        /// SMS[10:8]
        pub const sms_POS: u32 = 8;
        pub const sms_MASK: u32 = 0x700;
        /// IA1S[11:11]
        pub const ia1s_POS: u32 = 11;
        pub const ia1s_MASK: u32 = 0x800;
        /// IB1S[12:12]
        pub const ib1s_POS: u32 = 12;
        pub const ib1s_MASK: u32 = 0x1000;
    }
    /// FLTR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod fltr {
        /// OCM1AFLT1A[2:0]
        pub const ocm1aflt1a_POS: u32 = 0;
        pub const ocm1aflt1a_MASK: u32 = 0x7;
        /// CCP1A[3:3]
        pub const ccp1a_POS: u32 = 3;
        pub const ccp1a_MASK: u32 = 0x8;
        /// OCM1BFLT1B[6:4]
        pub const ocm1bflt1b_POS: u32 = 4;
        pub const ocm1bflt1b_MASK: u32 = 0x70;
        /// CCP1B[7:7]
        pub const ccp1b_POS: u32 = 7;
        pub const ccp1b_MASK: u32 = 0x80;
        /// OCM2AFLT2A[10:8]
        pub const ocm2aflt2a_POS: u32 = 8;
        pub const ocm2aflt2a_MASK: u32 = 0x700;
        /// CCP2A[11:11]
        pub const ccp2a_POS: u32 = 11;
        pub const ccp2a_MASK: u32 = 0x800;
        /// OCM2BFLT2B[14:12]
        pub const ocm2bflt2b_POS: u32 = 12;
        pub const ocm2bflt2b_MASK: u32 = 0x7000;
        /// CCP2B[15:15]
        pub const ccp2b_POS: u32 = 15;
        pub const ccp2b_MASK: u32 = 0x8000;
        /// OCM3AFLT3A[18:16]
        pub const ocm3aflt3a_POS: u32 = 16;
        pub const ocm3aflt3a_MASK: u32 = 0x70000;
        /// CCP3A[19:19]
        pub const ccp3a_POS: u32 = 19;
        pub const ccp3a_MASK: u32 = 0x80000;
        /// OCM3BFLT3B[22:20]
        pub const ocm3bflt3b_POS: u32 = 20;
        pub const ocm3bflt3b_MASK: u32 = 0x700000;
        /// CCP3B[23:23]
        pub const ccp3b_POS: u32 = 23;
        pub const ccp3b_MASK: u32 = 0x800000;
        /// FLTBK[26:24]
        pub const fltbk_POS: u32 = 24;
        pub const fltbk_MASK: u32 = 0x7000000;
        /// BKP[27:27]
        pub const bkp_POS: u32 = 27;
        pub const bkp_MASK: u32 = 0x8000000;
        /// FLTET[30:28]
        pub const fltet_POS: u32 = 28;
        pub const fltet_MASK: u32 = 0x70000000;
        /// ETP[31:31]
        pub const etp_POS: u32 = 31;
        pub const etp_MASK: u32 = 0x80000000;
    }
    /// TRIG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trig {
        /// UEVE[0:0]
        pub const ueve_POS: u32 = 0;
        pub const ueve_MASK: u32 = 0x1;
        /// CM1AE[1:1]
        pub const cm1ae_POS: u32 = 1;
        pub const cm1ae_MASK: u32 = 0x2;
        /// CM2AE[2:2]
        pub const cm2ae_POS: u32 = 2;
        pub const cm2ae_MASK: u32 = 0x4;
        /// CM3AE[3:3]
        pub const cm3ae_POS: u32 = 3;
        pub const cm3ae_MASK: u32 = 0x8;
        /// CM1BE[4:4]
        pub const cm1be_POS: u32 = 4;
        pub const cm1be_MASK: u32 = 0x10;
        /// CM2BE[5:5]
        pub const cm2be_POS: u32 = 5;
        pub const cm2be_MASK: u32 = 0x20;
        /// CM3BE[6:6]
        pub const cm3be_POS: u32 = 6;
        pub const cm3be_MASK: u32 = 0x40;
        /// ADTE[7:7]
        pub const adte_POS: u32 = 7;
        pub const adte_MASK: u32 = 0x80;
    }
    /// CH1CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch1cr {
        /// BKSA[1:0]
        pub const bksa_POS: u32 = 0;
        pub const bksa_MASK: u32 = 0x3;
        /// BKSB[3:2]
        pub const bksb_POS: u32 = 2;
        pub const bksb_MASK: u32 = 0xc;
        /// CSA[4:4]
        pub const csa_POS: u32 = 4;
        pub const csa_MASK: u32 = 0x10;
        /// CSB[5:5]
        pub const csb_POS: u32 = 5;
        pub const csb_MASK: u32 = 0x20;
        /// BUFEA[6:6]
        pub const bufea_POS: u32 = 6;
        pub const bufea_MASK: u32 = 0x40;
        /// BUFEB[7:7]
        pub const bufeb_POS: u32 = 7;
        pub const bufeb_MASK: u32 = 0x80;
        /// CIEA[8:8]
        pub const ciea_POS: u32 = 8;
        pub const ciea_MASK: u32 = 0x100;
        /// CIEB[9:9]
        pub const cieb_POS: u32 = 9;
        pub const cieb_MASK: u32 = 0x200;
        /// CISB[13:12]
        pub const cisb_POS: u32 = 12;
        pub const cisb_MASK: u32 = 0x3000;
        /// CCGA[14:14]
        pub const ccga_POS: u32 = 14;
        pub const ccga_MASK: u32 = 0x4000;
        /// CCGB[15:15]
        pub const ccgb_POS: u32 = 15;
        pub const ccgb_MASK: u32 = 0x8000;
    }
    /// CH2CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch2cr {
        /// BKSA[1:0]
        pub const bksa_POS: u32 = 0;
        pub const bksa_MASK: u32 = 0x3;
        /// BKSB[3:2]
        pub const bksb_POS: u32 = 2;
        pub const bksb_MASK: u32 = 0xc;
        /// CSA[4:4]
        pub const csa_POS: u32 = 4;
        pub const csa_MASK: u32 = 0x10;
        /// CSB[5:5]
        pub const csb_POS: u32 = 5;
        pub const csb_MASK: u32 = 0x20;
        /// BUFEA[6:6]
        pub const bufea_POS: u32 = 6;
        pub const bufea_MASK: u32 = 0x40;
        /// BUFEB[7:7]
        pub const bufeb_POS: u32 = 7;
        pub const bufeb_MASK: u32 = 0x80;
        /// CIEA[8:8]
        pub const ciea_POS: u32 = 8;
        pub const ciea_MASK: u32 = 0x100;
        /// CIEB[9:9]
        pub const cieb_POS: u32 = 9;
        pub const cieb_MASK: u32 = 0x200;
        /// CDEA[10:10]
        pub const cdea_POS: u32 = 10;
        pub const cdea_MASK: u32 = 0x400;
        /// CDEB[11:11]
        pub const cdeb_POS: u32 = 11;
        pub const cdeb_MASK: u32 = 0x800;
        /// CISB[13:12]
        pub const cisb_POS: u32 = 12;
        pub const cisb_MASK: u32 = 0x3000;
        /// CCGA[14:14]
        pub const ccga_POS: u32 = 14;
        pub const ccga_MASK: u32 = 0x4000;
        /// CCGB[15:15]
        pub const ccgb_POS: u32 = 15;
        pub const ccgb_MASK: u32 = 0x8000;
    }
    /// CH3CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch3cr {
        /// BKSA[1:0]
        pub const bksa_POS: u32 = 0;
        pub const bksa_MASK: u32 = 0x3;
        /// BKSB[3:2]
        pub const bksb_POS: u32 = 2;
        pub const bksb_MASK: u32 = 0xc;
        /// CSA[4:4]
        pub const csa_POS: u32 = 4;
        pub const csa_MASK: u32 = 0x10;
        /// CSB[5:5]
        pub const csb_POS: u32 = 5;
        pub const csb_MASK: u32 = 0x20;
        /// BUFEA[6:6]
        pub const bufea_POS: u32 = 6;
        pub const bufea_MASK: u32 = 0x40;
        /// BUFEB[7:7]
        pub const bufeb_POS: u32 = 7;
        pub const bufeb_MASK: u32 = 0x80;
        /// CIEA[8:8]
        pub const ciea_POS: u32 = 8;
        pub const ciea_MASK: u32 = 0x100;
        /// CIEB[9:9]
        pub const cieb_POS: u32 = 9;
        pub const cieb_MASK: u32 = 0x200;
        /// CDEA[10:10]
        pub const cdea_POS: u32 = 10;
        pub const cdea_MASK: u32 = 0x400;
        /// CDEB[11:11]
        pub const cdeb_POS: u32 = 11;
        pub const cdeb_MASK: u32 = 0x800;
        /// CISB[13:12]
        pub const cisb_POS: u32 = 12;
        pub const cisb_MASK: u32 = 0x3000;
        /// CCGA[14:14]
        pub const ccga_POS: u32 = 14;
        pub const ccga_MASK: u32 = 0x4000;
        /// CCGB[15:15]
        pub const ccgb_POS: u32 = 15;
        pub const ccgb_MASK: u32 = 0x8000;
    }
    /// DTR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dtr {
        /// DTR[7:0]
        pub const dtr_POS: u32 = 0;
        pub const dtr_MASK: u32 = 0xff;
        /// DTEN[9:9]
        pub const dten_POS: u32 = 9;
        pub const dten_MASK: u32 = 0x200;
        /// BKE[10:10]
        pub const bke_POS: u32 = 10;
        pub const bke_MASK: u32 = 0x400;
        /// AOE[11:11]
        pub const aoe_POS: u32 = 11;
        pub const aoe_MASK: u32 = 0x800;
        /// MOE[12:12]
        pub const moe_POS: u32 = 12;
        pub const moe_MASK: u32 = 0x1000;
        /// SAFEEN[13:13]
        pub const safeen_POS: u32 = 13;
        pub const safeen_MASK: u32 = 0x2000;
        /// VCE[14:14]
        pub const vce_POS: u32 = 14;
        pub const vce_MASK: u32 = 0x4000;
    }
    /// RCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod rcr {
        /// RCR[7:0]
        pub const rcr_POS: u32 = 0;
        pub const rcr_MASK: u32 = 0xff;
        /// OV[8:8]
        pub const ov_POS: u32 = 8;
        pub const ov_MASK: u32 = 0x100;
        /// UD[9:9]
        pub const ud_POS: u32 = 9;
        pub const ud_MASK: u32 = 0x200;
    }
    /// CH1CCRA 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch1ccra {
        /// CCR1A[15:0]
        pub const ccr1a_POS: u32 = 0;
        pub const ccr1a_MASK: u32 = 0xffff;
    }
    /// CH1CCRB 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch1ccrb {
        /// CCR1B[15:0]
        pub const ccr1b_POS: u32 = 0;
        pub const ccr1b_MASK: u32 = 0xffff;
    }
    /// CH2CCRA 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch2ccra {
        /// CCR2A[15:0]
        pub const ccr2a_POS: u32 = 0;
        pub const ccr2a_MASK: u32 = 0xffff;
    }
    /// CH2CCRB 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch2ccrb {
        /// CCR2B[15:0]
        pub const ccr2b_POS: u32 = 0;
        pub const ccr2b_MASK: u32 = 0xffff;
    }
    /// CH3CCRA 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch3ccra {
        /// CCR3A[15:0]
        pub const ccr3a_POS: u32 = 0;
        pub const ccr3a_MASK: u32 = 0xffff;
    }
    /// CH3CCRB 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch3ccrb {
        /// CCR3B[15:0]
        pub const ccr3b_POS: u32 = 0;
        pub const ccr3b_MASK: u32 = 0xffff;
    }
    /// CH4CCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch4ccr {
        /// CCR4[15:0]
        pub const ccr4_POS: u32 = 0;
        pub const ccr4_MASK: u32 = 0xffff;
    }
    /// CH4CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ch4cr {
        /// BUFE[0:0]
        pub const bufe_POS: u32 = 0;
        pub const bufe_MASK: u32 = 0x1;
        /// CIE[1:1]
        pub const cie_POS: u32 = 1;
        pub const cie_MASK: u32 = 0x2;
        /// CIS[4:3]
        pub const cis_POS: u32 = 3;
        pub const cis_MASK: u32 = 0x18;
        /// C4EN[5:5]
        pub const c4en_POS: u32 = 5;
        pub const c4en_MASK: u32 = 0x20;
    }
}
/// `AWT` 位段常量
pub mod awt {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// MD[2:1]
        pub const md_POS: u32 = 1;
        pub const md_MASK: u32 = 0x6;
        /// PRS[7:4]
        pub const prs_POS: u32 = 4;
        pub const prs_MASK: u32 = 0xf0;
        /// SRC[10:8]
        pub const src_POS: u32 = 8;
        pub const src_MASK: u32 = 0x700;
    }
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// UD[3:3]
        pub const ud_POS: u32 = 3;
        pub const ud_MASK: u32 = 0x8;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// UD[3:3]
        pub const ud_POS: u32 = 3;
        pub const ud_MASK: u32 = 0x8;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// UD[3:3]
        pub const ud_POS: u32 = 3;
        pub const ud_MASK: u32 = 0x8;
    }
}
/// `BTIM` 位段常量
pub mod btim {
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
    }
    /// ACR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod acr {
        /// ETRFLT[6:4]
        pub const etrflt_POS: u32 = 4;
        pub const etrflt_MASK: u32 = 0x70;
    }
    /// BCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod bcr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// MODE[2:1]
        pub const mode_POS: u32 = 1;
        pub const mode_MASK: u32 = 0x6;
        /// TRS[3:3]
        pub const trs_POS: u32 = 3;
        pub const trs_MASK: u32 = 0x8;
        /// POL[4:4]
        pub const pol_POS: u32 = 4;
        pub const pol_MASK: u32 = 0x10;
        /// ONESHOT[5:5]
        pub const oneshot_POS: u32 = 5;
        pub const oneshot_MASK: u32 = 0x20;
        /// TOGEN[6:6]
        pub const togen_POS: u32 = 6;
        pub const togen_MASK: u32 = 0x40;
        /// PRS[10:7]
        pub const prs_POS: u32 = 7;
        pub const prs_MASK: u32 = 0x780;
        /// PRSSTATUS[14:11]
        pub const prsstatus_POS: u32 = 11;
        pub const prsstatus_MASK: u32 = 0x7800;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// OV[0:0]
        pub const ov_POS: u32 = 0;
        pub const ov_MASK: u32 = 0x1;
        /// TI[1:1]
        pub const ti_POS: u32 = 1;
        pub const ti_MASK: u32 = 0x2;
        /// TOP[2:2]
        pub const top_POS: u32 = 2;
        pub const top_MASK: u32 = 0x4;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// OV[0:0]
        pub const ov_POS: u32 = 0;
        pub const ov_MASK: u32 = 0x1;
        /// TI[1:1]
        pub const ti_POS: u32 = 1;
        pub const ti_MASK: u32 = 0x2;
        /// TOP[2:2]
        pub const top_POS: u32 = 2;
        pub const top_MASK: u32 = 0x4;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// OV[0:0]
        pub const ov_POS: u32 = 0;
        pub const ov_MASK: u32 = 0x1;
        /// TI[1:1]
        pub const ti_POS: u32 = 1;
        pub const ti_MASK: u32 = 0x2;
        /// TOP[2:2]
        pub const top_POS: u32 = 2;
        pub const top_MASK: u32 = 0x4;
    }
}
/// `CRC` 位段常量
pub mod crc {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// MODE[3:0]
        pub const mode_POS: u32 = 0;
        pub const mode_MASK: u32 = 0xf;
    }
    /// DR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dr {
        /// DR[7:0]
        pub const dr_POS: u32 = 0;
        pub const dr_MASK: u32 = 0xff;
    }
    /// RESULT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result {
        /// RESULT16[15:0]
        pub const result16_POS: u32 = 0;
        pub const result16_MASK: u32 = 0xffff;
    }
}
/// `FLASH` 位段常量
pub mod flash {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// MODE[1:0]
        pub const mode_POS: u32 = 0;
        pub const mode_MASK: u32 = 0x3;
        /// STANDBY[4:4]
        pub const standby_POS: u32 = 4;
        pub const standby_MASK: u32 = 0x10;
        /// BUSY[5:5]
        pub const busy_POS: u32 = 5;
        pub const busy_MASK: u32 = 0x20;
        /// SECURITY[7:6]
        pub const security_POS: u32 = 6;
        pub const security_MASK: u32 = 0xc0;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// WAIT[2:0]
        pub const wait_POS: u32 = 0;
        pub const wait_MASK: u32 = 0x7;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// PAGELOCK 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod pagelock {
        /// LOCK0[0:0]
        pub const lock0_POS: u32 = 0;
        pub const lock0_MASK: u32 = 0x1;
        /// LOCK1[1:1]
        pub const lock1_POS: u32 = 1;
        pub const lock1_MASK: u32 = 0x2;
        /// LOCK2[2:2]
        pub const lock2_POS: u32 = 2;
        pub const lock2_MASK: u32 = 0x4;
        /// LOCK3[3:3]
        pub const lock3_POS: u32 = 3;
        pub const lock3_MASK: u32 = 0x8;
        /// LOCK4[4:4]
        pub const lock4_POS: u32 = 4;
        pub const lock4_MASK: u32 = 0x10;
        /// LOCK5[5:5]
        pub const lock5_POS: u32 = 5;
        pub const lock5_MASK: u32 = 0x20;
        /// LOCK6[6:6]
        pub const lock6_POS: u32 = 6;
        pub const lock6_MASK: u32 = 0x40;
        /// LOCK7[7:7]
        pub const lock7_POS: u32 = 7;
        pub const lock7_MASK: u32 = 0x80;
        /// LOCK8[8:8]
        pub const lock8_POS: u32 = 8;
        pub const lock8_MASK: u32 = 0x100;
        /// LOCK9[9:9]
        pub const lock9_POS: u32 = 9;
        pub const lock9_MASK: u32 = 0x200;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// PC[0:0]
        pub const pc_POS: u32 = 0;
        pub const pc_MASK: u32 = 0x1;
        /// PAGELOCK[1:1]
        pub const pagelock_POS: u32 = 1;
        pub const pagelock_MASK: u32 = 0x2;
        /// PROG[4:4]
        pub const prog_POS: u32 = 4;
        pub const prog_MASK: u32 = 0x10;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// PC[0:0]
        pub const pc_POS: u32 = 0;
        pub const pc_MASK: u32 = 0x1;
        /// PAGELOCK[1:1]
        pub const pagelock_POS: u32 = 1;
        pub const pagelock_MASK: u32 = 0x2;
        /// PROG[4:4]
        pub const prog_POS: u32 = 4;
        pub const prog_MASK: u32 = 0x10;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// PC[0:0]
        pub const pc_POS: u32 = 0;
        pub const pc_MASK: u32 = 0x1;
        /// PAGELOCK[1:1]
        pub const pagelock_POS: u32 = 1;
        pub const pagelock_MASK: u32 = 0x2;
        /// PROG[4:4]
        pub const prog_POS: u32 = 4;
        pub const prog_MASK: u32 = 0x10;
    }
}
/// `GPIO` 位段常量
pub mod gpio {
    /// DIR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dir {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// OPENDRAIN 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod opendrain {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// PDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod pdr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// PUR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod pur {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// AFRL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod afrl {
        /// AFR0[2:0]
        pub const afr0_POS: u32 = 0;
        pub const afr0_MASK: u32 = 0x7;
        /// AFR1[6:4]
        pub const afr1_POS: u32 = 4;
        pub const afr1_MASK: u32 = 0x70;
        /// AFR2[10:8]
        pub const afr2_POS: u32 = 8;
        pub const afr2_MASK: u32 = 0x700;
        /// AFR3[14:12]
        pub const afr3_POS: u32 = 12;
        pub const afr3_MASK: u32 = 0x7000;
        /// AFR4[18:16]
        pub const afr4_POS: u32 = 16;
        pub const afr4_MASK: u32 = 0x70000;
        /// AFR5[22:20]
        pub const afr5_POS: u32 = 20;
        pub const afr5_MASK: u32 = 0x700000;
        /// AFR6[26:24]
        pub const afr6_POS: u32 = 24;
        pub const afr6_MASK: u32 = 0x7000000;
        /// AFR7[30:28]
        pub const afr7_POS: u32 = 28;
        pub const afr7_MASK: u32 = 0x70000000;
    }
    /// ANALOG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod analog {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// RISEIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod riseie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// FALLIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod fallie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// HIGHIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod highie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// LOWIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod lowie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// FILTER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod filter {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
        /// FLTCLK[18:16]
        pub const fltclk_POS: u32 = 16;
        pub const fltclk_MASK: u32 = 0x70000;
    }
    /// IDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod idr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// ODR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod odr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
    /// BRR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod brr {
        /// BRR0[0:0]
        pub const brr0_POS: u32 = 0;
        pub const brr0_MASK: u32 = 0x1;
        /// BRR1[1:1]
        pub const brr1_POS: u32 = 1;
        pub const brr1_MASK: u32 = 0x2;
        /// BRR2[2:2]
        pub const brr2_POS: u32 = 2;
        pub const brr2_MASK: u32 = 0x4;
        /// BRR3[3:3]
        pub const brr3_POS: u32 = 3;
        pub const brr3_MASK: u32 = 0x8;
        /// BRR4[4:4]
        pub const brr4_POS: u32 = 4;
        pub const brr4_MASK: u32 = 0x10;
        /// BRR5[5:5]
        pub const brr5_POS: u32 = 5;
        pub const brr5_MASK: u32 = 0x20;
        /// BRR6[6:6]
        pub const brr6_POS: u32 = 6;
        pub const brr6_MASK: u32 = 0x40;
        /// BRR7[7:7]
        pub const brr7_POS: u32 = 7;
        pub const brr7_MASK: u32 = 0x80;
    }
    /// BSRR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod bsrr {
        /// BSS0[0:0]
        pub const bss0_POS: u32 = 0;
        pub const bss0_MASK: u32 = 0x1;
        /// BSS1[1:1]
        pub const bss1_POS: u32 = 1;
        pub const bss1_MASK: u32 = 0x2;
        /// BSS2[2:2]
        pub const bss2_POS: u32 = 2;
        pub const bss2_MASK: u32 = 0x4;
        /// BSS3[3:3]
        pub const bss3_POS: u32 = 3;
        pub const bss3_MASK: u32 = 0x8;
        /// BSS4[4:4]
        pub const bss4_POS: u32 = 4;
        pub const bss4_MASK: u32 = 0x10;
        /// BSS5[5:5]
        pub const bss5_POS: u32 = 5;
        pub const bss5_MASK: u32 = 0x20;
        /// BSS6[6:6]
        pub const bss6_POS: u32 = 6;
        pub const bss6_MASK: u32 = 0x40;
        /// BSS7[7:7]
        pub const bss7_POS: u32 = 7;
        pub const bss7_MASK: u32 = 0x80;
        /// BRR0[16:16]
        pub const brr0_POS: u32 = 16;
        pub const brr0_MASK: u32 = 0x10000;
        /// BRR1[17:17]
        pub const brr1_POS: u32 = 17;
        pub const brr1_MASK: u32 = 0x20000;
        /// BRR2[18:18]
        pub const brr2_POS: u32 = 18;
        pub const brr2_MASK: u32 = 0x40000;
        /// BRR3[19:19]
        pub const brr3_POS: u32 = 19;
        pub const brr3_MASK: u32 = 0x80000;
        /// BRR4[20:20]
        pub const brr4_POS: u32 = 20;
        pub const brr4_MASK: u32 = 0x100000;
        /// BRR5[21:21]
        pub const brr5_POS: u32 = 21;
        pub const brr5_MASK: u32 = 0x200000;
        /// BRR6[22:22]
        pub const brr6_POS: u32 = 22;
        pub const brr6_MASK: u32 = 0x400000;
        /// BRR7[23:23]
        pub const brr7_POS: u32 = 23;
        pub const brr7_MASK: u32 = 0x800000;
    }
    /// TOG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tog {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// PIN6[6:6]
        pub const pin6_POS: u32 = 6;
        pub const pin6_MASK: u32 = 0x40;
        /// PIN7[7:7]
        pub const pin7_POS: u32 = 7;
        pub const pin7_MASK: u32 = 0x80;
    }
}
/// `GPIOC` 位段常量
pub mod gpioc {
    /// DIR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dir {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// OPENDRAIN 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod opendrain {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// PDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod pdr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// PUR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod pur {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// AFRL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod afrl {
        /// AFR0[2:0]
        pub const afr0_POS: u32 = 0;
        pub const afr0_MASK: u32 = 0x7;
        /// AFR1[6:4]
        pub const afr1_POS: u32 = 4;
        pub const afr1_MASK: u32 = 0x70;
        /// AFR2[10:8]
        pub const afr2_POS: u32 = 8;
        pub const afr2_MASK: u32 = 0x700;
        /// AFR3[14:12]
        pub const afr3_POS: u32 = 12;
        pub const afr3_MASK: u32 = 0x7000;
        /// AFR4[18:16]
        pub const afr4_POS: u32 = 16;
        pub const afr4_MASK: u32 = 0x70000;
        /// AFR5[22:20]
        pub const afr5_POS: u32 = 20;
        pub const afr5_MASK: u32 = 0x700000;
    }
    /// ANALOG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod analog {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// RISEIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod riseie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// FALLIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod fallie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// HIGHIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod highie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// LOWIE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod lowie {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// FILTER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod filter {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
        /// FLTCLK[18:16]
        pub const fltclk_POS: u32 = 16;
        pub const fltclk_MASK: u32 = 0x70000;
    }
    /// IDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod idr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// ODR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod odr {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
    /// BRR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod brr {
        /// BRR0[0:0]
        pub const brr0_POS: u32 = 0;
        pub const brr0_MASK: u32 = 0x1;
        /// BRR1[1:1]
        pub const brr1_POS: u32 = 1;
        pub const brr1_MASK: u32 = 0x2;
        /// BRR2[2:2]
        pub const brr2_POS: u32 = 2;
        pub const brr2_MASK: u32 = 0x4;
        /// BRR3[3:3]
        pub const brr3_POS: u32 = 3;
        pub const brr3_MASK: u32 = 0x8;
        /// BRR4[4:4]
        pub const brr4_POS: u32 = 4;
        pub const brr4_MASK: u32 = 0x10;
        /// BRR5[5:5]
        pub const brr5_POS: u32 = 5;
        pub const brr5_MASK: u32 = 0x20;
    }
    /// BSRR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod bsrr {
        /// BSS0[0:0]
        pub const bss0_POS: u32 = 0;
        pub const bss0_MASK: u32 = 0x1;
        /// BSS1[1:1]
        pub const bss1_POS: u32 = 1;
        pub const bss1_MASK: u32 = 0x2;
        /// BSS2[2:2]
        pub const bss2_POS: u32 = 2;
        pub const bss2_MASK: u32 = 0x4;
        /// BSS3[3:3]
        pub const bss3_POS: u32 = 3;
        pub const bss3_MASK: u32 = 0x8;
        /// BSS4[4:4]
        pub const bss4_POS: u32 = 4;
        pub const bss4_MASK: u32 = 0x10;
        /// BSS5[5:5]
        pub const bss5_POS: u32 = 5;
        pub const bss5_MASK: u32 = 0x20;
        /// BRR0[16:16]
        pub const brr0_POS: u32 = 16;
        pub const brr0_MASK: u32 = 0x10000;
        /// BRR1[17:17]
        pub const brr1_POS: u32 = 17;
        pub const brr1_MASK: u32 = 0x20000;
        /// BRR2[18:18]
        pub const brr2_POS: u32 = 18;
        pub const brr2_MASK: u32 = 0x40000;
        /// BRR3[19:19]
        pub const brr3_POS: u32 = 19;
        pub const brr3_MASK: u32 = 0x80000;
        /// BRR4[20:20]
        pub const brr4_POS: u32 = 20;
        pub const brr4_MASK: u32 = 0x100000;
        /// BRR5[21:21]
        pub const brr5_POS: u32 = 21;
        pub const brr5_MASK: u32 = 0x200000;
    }
    /// TOG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tog {
        /// PIN0[0:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x1;
        /// PIN1[1:1]
        pub const pin1_POS: u32 = 1;
        pub const pin1_MASK: u32 = 0x2;
        /// PIN2[2:2]
        pub const pin2_POS: u32 = 2;
        pub const pin2_MASK: u32 = 0x4;
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
        /// PIN4[4:4]
        pub const pin4_POS: u32 = 4;
        pub const pin4_MASK: u32 = 0x10;
        /// PIN5[5:5]
        pub const pin5_POS: u32 = 5;
        pub const pin5_MASK: u32 = 0x20;
    }
}
/// `GTIM` 位段常量
pub mod gtim {
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
    }
    /// CMMR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cmmr {
        /// CC1M[3:0]
        pub const cc1m_POS: u32 = 0;
        pub const cc1m_MASK: u32 = 0xf;
        /// CC2M[7:4]
        pub const cc2m_POS: u32 = 4;
        pub const cc2m_MASK: u32 = 0xf0;
        /// CC3M[11:8]
        pub const cc3m_POS: u32 = 8;
        pub const cc3m_MASK: u32 = 0xf00;
        /// CC4M[15:12]
        pub const cc4m_POS: u32 = 12;
        pub const cc4m_MASK: u32 = 0xf000;
    }
    /// ETR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod etr {
        /// ETRFLT[6:4]
        pub const etrflt_POS: u32 = 4;
        pub const etrflt_MASK: u32 = 0x70;
    }
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// MODE[2:1]
        pub const mode_POS: u32 = 1;
        pub const mode_MASK: u32 = 0x6;
        /// TRS[3:3]
        pub const trs_POS: u32 = 3;
        pub const trs_MASK: u32 = 0x8;
        /// POL[4:4]
        pub const pol_POS: u32 = 4;
        pub const pol_MASK: u32 = 0x10;
        /// ONESHOT[5:5]
        pub const oneshot_POS: u32 = 5;
        pub const oneshot_MASK: u32 = 0x20;
        /// TOGEN[6:6]
        pub const togen_POS: u32 = 6;
        pub const togen_MASK: u32 = 0x40;
        /// PRS[10:7]
        pub const prs_POS: u32 = 7;
        pub const prs_MASK: u32 = 0x780;
        /// PRSSTATUS[14:11]
        pub const prsstatus_POS: u32 = 11;
        pub const prsstatus_MASK: u32 = 0x7800;
        /// ENCMODE[16:15]
        pub const encmode_POS: u32 = 15;
        pub const encmode_MASK: u32 = 0x18000;
        /// ENCRESET[18:17]
        pub const encreset_POS: u32 = 17;
        pub const encreset_MASK: u32 = 0x60000;
        /// ENCRELOAD[20:19]
        pub const encreload_POS: u32 = 19;
        pub const encreload_MASK: u32 = 0x180000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// OV[0:0]
        pub const ov_POS: u32 = 0;
        pub const ov_MASK: u32 = 0x1;
        /// TI[1:1]
        pub const ti_POS: u32 = 1;
        pub const ti_MASK: u32 = 0x2;
        /// UD[2:2]
        pub const ud_POS: u32 = 2;
        pub const ud_MASK: u32 = 0x4;
        /// CC1[3:3]
        pub const cc1_POS: u32 = 3;
        pub const cc1_MASK: u32 = 0x8;
        /// CC2[4:4]
        pub const cc2_POS: u32 = 4;
        pub const cc2_MASK: u32 = 0x10;
        /// CC3[5:5]
        pub const cc3_POS: u32 = 5;
        pub const cc3_MASK: u32 = 0x20;
        /// CC4[6:6]
        pub const cc4_POS: u32 = 6;
        pub const cc4_MASK: u32 = 0x40;
        /// DIRCHANGE[9:9]
        pub const dirchange_POS: u32 = 9;
        pub const dirchange_MASK: u32 = 0x200;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// OV[0:0]
        pub const ov_POS: u32 = 0;
        pub const ov_MASK: u32 = 0x1;
        /// TI[1:1]
        pub const ti_POS: u32 = 1;
        pub const ti_MASK: u32 = 0x2;
        /// UD[2:2]
        pub const ud_POS: u32 = 2;
        pub const ud_MASK: u32 = 0x4;
        /// CC1[3:3]
        pub const cc1_POS: u32 = 3;
        pub const cc1_MASK: u32 = 0x8;
        /// CC2[4:4]
        pub const cc2_POS: u32 = 4;
        pub const cc2_MASK: u32 = 0x10;
        /// CC3[5:5]
        pub const cc3_POS: u32 = 5;
        pub const cc3_MASK: u32 = 0x20;
        /// CC4[6:6]
        pub const cc4_POS: u32 = 6;
        pub const cc4_MASK: u32 = 0x40;
        /// DIRCHANGE[9:9]
        pub const dirchange_POS: u32 = 9;
        pub const dirchange_MASK: u32 = 0x200;
        /// DIR[10:10]
        pub const dir_POS: u32 = 10;
        pub const dir_MASK: u32 = 0x400;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// OV[0:0]
        pub const ov_POS: u32 = 0;
        pub const ov_MASK: u32 = 0x1;
        /// TI[1:1]
        pub const ti_POS: u32 = 1;
        pub const ti_MASK: u32 = 0x2;
        /// UD[2:2]
        pub const ud_POS: u32 = 2;
        pub const ud_MASK: u32 = 0x4;
        /// CC1[3:3]
        pub const cc1_POS: u32 = 3;
        pub const cc1_MASK: u32 = 0x8;
        /// CC2[4:4]
        pub const cc2_POS: u32 = 4;
        pub const cc2_MASK: u32 = 0x10;
        /// CC3[5:5]
        pub const cc3_POS: u32 = 5;
        pub const cc3_MASK: u32 = 0x20;
        /// CC4[6:6]
        pub const cc4_POS: u32 = 6;
        pub const cc4_MASK: u32 = 0x40;
        /// DIRCHANGE[9:9]
        pub const dirchange_POS: u32 = 9;
        pub const dirchange_MASK: u32 = 0x200;
    }
    /// CCR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr1 {
        /// CCR[15:0]
        pub const ccr_POS: u32 = 0;
        pub const ccr_MASK: u32 = 0xffff;
    }
    /// CCR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr2 {
        /// CCR[15:0]
        pub const ccr_POS: u32 = 0;
        pub const ccr_MASK: u32 = 0xffff;
    }
    /// CCR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr3 {
        /// CCR[15:0]
        pub const ccr_POS: u32 = 0;
        pub const ccr_MASK: u32 = 0xffff;
    }
    /// CCR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr4 {
        /// CCR[15:0]
        pub const ccr_POS: u32 = 0;
        pub const ccr_MASK: u32 = 0xffff;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// CH1FLT[2:0]
        pub const ch1flt_POS: u32 = 0;
        pub const ch1flt_MASK: u32 = 0x7;
        /// CH1POL[3:3]
        pub const ch1pol_POS: u32 = 3;
        pub const ch1pol_MASK: u32 = 0x8;
        /// CH2FLT[6:4]
        pub const ch2flt_POS: u32 = 4;
        pub const ch2flt_MASK: u32 = 0x70;
        /// CH2POL[7:7]
        pub const ch2pol_POS: u32 = 7;
        pub const ch2pol_MASK: u32 = 0x80;
        /// CH3FLT[10:8]
        pub const ch3flt_POS: u32 = 8;
        pub const ch3flt_MASK: u32 = 0x700;
        /// CH3POL[11:11]
        pub const ch3pol_POS: u32 = 11;
        pub const ch3pol_MASK: u32 = 0x800;
        /// CH4FLT[14:12]
        pub const ch4flt_POS: u32 = 12;
        pub const ch4flt_MASK: u32 = 0x7000;
        /// CH4POL[15:15]
        pub const ch4pol_POS: u32 = 15;
        pub const ch4pol_MASK: u32 = 0x8000;
    }
}
/// `I2C` 位段常量
pub mod i2c {
    /// BRREN 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod brren {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
    }
    /// BRR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod brr {
        /// BRR[7:0]
        pub const brr_POS: u32 = 0;
        pub const brr_MASK: u32 = 0xff;
    }
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// FLT[0:0]
        pub const flt_POS: u32 = 0;
        pub const flt_MASK: u32 = 0x1;
        /// AA[2:2]
        pub const aa_POS: u32 = 2;
        pub const aa_MASK: u32 = 0x4;
        /// SI[3:3]
        pub const si_POS: u32 = 3;
        pub const si_MASK: u32 = 0x8;
        /// STO[4:4]
        pub const sto_POS: u32 = 4;
        pub const sto_MASK: u32 = 0x10;
        /// STA[5:5]
        pub const sta_POS: u32 = 5;
        pub const sta_MASK: u32 = 0x20;
        /// EN[6:6]
        pub const en_POS: u32 = 6;
        pub const en_MASK: u32 = 0x40;
    }
    /// DR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dr {
        /// DR[7:0]
        pub const dr_POS: u32 = 0;
        pub const dr_MASK: u32 = 0xff;
    }
    /// ADDR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod addr0 {
        /// GC[0:0]
        pub const gc_POS: u32 = 0;
        pub const gc_MASK: u32 = 0x1;
        /// ADDR0[7:1]
        pub const addr0_POS: u32 = 1;
        pub const addr0_MASK: u32 = 0xfe;
    }
    /// STAT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod stat {
        /// STAT[7:0]
        pub const stat_POS: u32 = 0;
        pub const stat_MASK: u32 = 0xff;
    }
    /// ADDR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod addr1 {
        /// ADDR1[7:1]
        pub const addr1_POS: u32 = 1;
        pub const addr1_MASK: u32 = 0xfe;
    }
    /// ADDR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod addr2 {
        /// ADDR2[7:1]
        pub const addr2_POS: u32 = 1;
        pub const addr2_MASK: u32 = 0xfe;
    }
    /// MATCH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod r#match {
        /// ADDR0[0:0]
        pub const addr0_POS: u32 = 0;
        pub const addr0_MASK: u32 = 0x1;
        /// ADDR1[1:1]
        pub const addr1_POS: u32 = 1;
        pub const addr1_MASK: u32 = 0x2;
        /// ADDR2[2:2]
        pub const addr2_POS: u32 = 2;
        pub const addr2_MASK: u32 = 0x4;
    }
}
/// `IWDT` 位段常量
pub mod iwdt {
    /// KR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod kr {
        /// KR[15:0]
        pub const kr_POS: u32 = 0;
        pub const kr_MASK: u32 = 0xffff;
    }
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// PRS[2:0]
        pub const prs_POS: u32 = 0;
        pub const prs_MASK: u32 = 0x7;
        /// ACTION[3:3]
        pub const action_POS: u32 = 3;
        pub const action_MASK: u32 = 0x8;
        /// IE[4:4]
        pub const ie_POS: u32 = 4;
        pub const ie_MASK: u32 = 0x10;
        /// PAUSE[5:5]
        pub const pause_POS: u32 = 5;
        pub const pause_MASK: u32 = 0x20;
    }
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[11:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xfff;
    }
    /// SR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sr {
        /// CRF[0:0]
        pub const crf_POS: u32 = 0;
        pub const crf_MASK: u32 = 0x1;
        /// ARRF[1:1]
        pub const arrf_POS: u32 = 1;
        pub const arrf_MASK: u32 = 0x2;
        /// WINRF[2:2]
        pub const winrf_POS: u32 = 2;
        pub const winrf_MASK: u32 = 0x4;
        /// OV[3:3]
        pub const ov_POS: u32 = 3;
        pub const ov_MASK: u32 = 0x8;
        /// RUN[4:4]
        pub const run_POS: u32 = 4;
        pub const run_MASK: u32 = 0x10;
        /// RELOAD[5:5]
        pub const reload_POS: u32 = 5;
        pub const reload_MASK: u32 = 0x20;
    }
    /// WINR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod winr {
        /// WINR[11:0]
        pub const winr_POS: u32 = 0;
        pub const winr_MASK: u32 = 0xfff;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[11:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xfff;
    }
}
/// `LVD` 位段常量
pub mod lvd {
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// ACTION[1:1]
        pub const action_POS: u32 = 1;
        pub const action_MASK: u32 = 0x2;
        /// SOURCE[3:2]
        pub const source_POS: u32 = 2;
        pub const source_MASK: u32 = 0xc;
        /// VTH[7:4]
        pub const vth_POS: u32 = 4;
        pub const vth_MASK: u32 = 0xf0;
        /// IE[9:9]
        pub const ie_POS: u32 = 9;
        pub const ie_MASK: u32 = 0x200;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// FLTEN[0:0]
        pub const flten_POS: u32 = 0;
        pub const flten_MASK: u32 = 0x1;
        /// FLTTIME[3:1]
        pub const flttime_POS: u32 = 1;
        pub const flttime_MASK: u32 = 0xe;
        /// FLTCLK[4:4]
        pub const fltclk_POS: u32 = 4;
        pub const fltclk_MASK: u32 = 0x10;
        /// RISE[5:5]
        pub const rise_POS: u32 = 5;
        pub const rise_MASK: u32 = 0x20;
        /// FALL[6:6]
        pub const fall_POS: u32 = 6;
        pub const fall_MASK: u32 = 0x40;
        /// LEVEL[7:7]
        pub const level_POS: u32 = 7;
        pub const level_MASK: u32 = 0x80;
    }
    /// SR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sr {
        /// INTF[0:0]
        pub const intf_POS: u32 = 0;
        pub const intf_MASK: u32 = 0x1;
        /// FLTV[1:1]
        pub const fltv_POS: u32 = 1;
        pub const fltv_MASK: u32 = 0x2;
    }
}
/// `RAM` 位段常量
pub mod ram {
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// PARITY[1:1]
        pub const parity_POS: u32 = 1;
        pub const parity_MASK: u32 = 0x2;
    }
    /// ADDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod addr {
        /// ADDR[31:0]
        pub const addr_POS: u32 = 0;
        pub const addr_MASK: u32 = 0xffffffff;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// PARITY[0:0]
        pub const parity_POS: u32 = 0;
        pub const parity_MASK: u32 = 0x1;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// PARITY[0:0]
        pub const parity_POS: u32 = 0;
        pub const parity_MASK: u32 = 0x1;
    }
}
/// `SPI` 位段常量
pub mod spi {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// CPHA[0:0]
        pub const cpha_POS: u32 = 0;
        pub const cpha_MASK: u32 = 0x1;
        /// CPOL[1:1]
        pub const cpol_POS: u32 = 1;
        pub const cpol_MASK: u32 = 0x2;
        /// MSTR[2:2]
        pub const mstr_POS: u32 = 2;
        pub const mstr_MASK: u32 = 0x4;
        /// BR[5:3]
        pub const br_POS: u32 = 3;
        pub const br_MASK: u32 = 0x38;
        /// EN[6:6]
        pub const en_POS: u32 = 6;
        pub const en_MASK: u32 = 0x40;
        /// LSBF[7:7]
        pub const lsbf_POS: u32 = 7;
        pub const lsbf_MASK: u32 = 0x80;
        /// SMP[8:8]
        pub const smp_POS: u32 = 8;
        pub const smp_MASK: u32 = 0x100;
        /// SSM[9:9]
        pub const ssm_POS: u32 = 9;
        pub const ssm_MASK: u32 = 0x200;
        /// WIDTH[13:10]
        pub const width_POS: u32 = 10;
        pub const width_MASK: u32 = 0x3c00;
        /// MODE[15:14]
        pub const mode_POS: u32 = 14;
        pub const mode_MASK: u32 = 0xc000;
        /// MISOHD[18:18]
        pub const misohd_POS: u32 = 18;
        pub const misohd_MASK: u32 = 0x40000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// SSF[2:2]
        pub const ssf_POS: u32 = 2;
        pub const ssf_MASK: u32 = 0x4;
        /// SSR[3:3]
        pub const ssr_POS: u32 = 3;
        pub const ssr_MASK: u32 = 0x8;
        /// UD[4:4]
        pub const ud_POS: u32 = 4;
        pub const ud_MASK: u32 = 0x10;
        /// OV[5:5]
        pub const ov_POS: u32 = 5;
        pub const ov_MASK: u32 = 0x20;
        /// SSERR[6:6]
        pub const sserr_POS: u32 = 6;
        pub const sserr_MASK: u32 = 0x40;
        /// MODF[7:7]
        pub const modf_POS: u32 = 7;
        pub const modf_MASK: u32 = 0x80;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// HDOE[0:0]
        pub const hdoe_POS: u32 = 0;
        pub const hdoe_MASK: u32 = 0x1;
    }
    /// SSI 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ssi {
        /// SSI[0:0]
        pub const ssi_POS: u32 = 0;
        pub const ssi_MASK: u32 = 0x1;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// SSF[2:2]
        pub const ssf_POS: u32 = 2;
        pub const ssf_MASK: u32 = 0x4;
        /// SSR[3:3]
        pub const ssr_POS: u32 = 3;
        pub const ssr_MASK: u32 = 0x8;
        /// UD[4:4]
        pub const ud_POS: u32 = 4;
        pub const ud_MASK: u32 = 0x10;
        /// OV[5:5]
        pub const ov_POS: u32 = 5;
        pub const ov_MASK: u32 = 0x20;
        /// SSERR[6:6]
        pub const sserr_POS: u32 = 6;
        pub const sserr_MASK: u32 = 0x40;
        /// MODF[7:7]
        pub const modf_POS: u32 = 7;
        pub const modf_MASK: u32 = 0x80;
        /// BUSY[8:8]
        pub const busy_POS: u32 = 8;
        pub const busy_MASK: u32 = 0x100;
        /// SSLVL[9:9]
        pub const sslvl_POS: u32 = 9;
        pub const sslvl_MASK: u32 = 0x200;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// FLUSH[0:0]
        pub const flush_POS: u32 = 0;
        pub const flush_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// SSF[2:2]
        pub const ssf_POS: u32 = 2;
        pub const ssf_MASK: u32 = 0x4;
        /// SSR[3:3]
        pub const ssr_POS: u32 = 3;
        pub const ssr_MASK: u32 = 0x8;
        /// UD[4:4]
        pub const ud_POS: u32 = 4;
        pub const ud_MASK: u32 = 0x10;
        /// OV[5:5]
        pub const ov_POS: u32 = 5;
        pub const ov_MASK: u32 = 0x20;
        /// SSERR[6:6]
        pub const sserr_POS: u32 = 6;
        pub const sserr_MASK: u32 = 0x40;
        /// MODF[7:7]
        pub const modf_POS: u32 = 7;
        pub const modf_MASK: u32 = 0x80;
    }
    /// DR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dr {
        /// DR[15:0]
        pub const dr_POS: u32 = 0;
        pub const dr_MASK: u32 = 0xffff;
    }
}
/// `SYSCTRL` 位段常量
pub mod sysctrl {
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// SYSCLK[2:0]
        pub const sysclk_POS: u32 = 0;
        pub const sysclk_MASK: u32 = 0x7;
        /// PCLKPRS[4:3]
        pub const pclkprs_POS: u32 = 3;
        pub const pclkprs_MASK: u32 = 0x18;
        /// HCLKPRS[7:5]
        pub const hclkprs_POS: u32 = 5;
        pub const hclkprs_MASK: u32 = 0xe0;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// HSIEN[0:0]
        pub const hsien_POS: u32 = 0;
        pub const hsien_MASK: u32 = 0x1;
        /// HEXEN[1:1]
        pub const hexen_POS: u32 = 1;
        pub const hexen_MASK: u32 = 0x2;
        /// LSIEN[3:3]
        pub const lsien_POS: u32 = 3;
        pub const lsien_MASK: u32 = 0x8;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// RSTIO[0:0]
        pub const rstio_POS: u32 = 0;
        pub const rstio_MASK: u32 = 0x1;
        /// SWDIO[1:1]
        pub const swdio_POS: u32 = 1;
        pub const swdio_MASK: u32 = 0x2;
        /// LOCKUP[2:2]
        pub const lockup_POS: u32 = 2;
        pub const lockup_MASK: u32 = 0x4;
        /// WAKEUPCLK[3:3]
        pub const wakeupclk_POS: u32 = 3;
        pub const wakeupclk_MASK: u32 = 0x8;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// HSIRDY[0:0]
        pub const hsirdy_POS: u32 = 0;
        pub const hsirdy_MASK: u32 = 0x1;
        /// HEXRDY[1:1]
        pub const hexrdy_POS: u32 = 1;
        pub const hexrdy_MASK: u32 = 0x2;
        /// LSIRDY[3:3]
        pub const lsirdy_POS: u32 = 3;
        pub const lsirdy_MASK: u32 = 0x8;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// HSIRDY[0:0]
        pub const hsirdy_POS: u32 = 0;
        pub const hsirdy_MASK: u32 = 0x1;
        /// HEXRDY[1:1]
        pub const hexrdy_POS: u32 = 1;
        pub const hexrdy_MASK: u32 = 0x2;
        /// LSIRDY[3:3]
        pub const lsirdy_POS: u32 = 3;
        pub const lsirdy_MASK: u32 = 0x8;
        /// HSISTABLE[11:11]
        pub const hsistable_POS: u32 = 11;
        pub const hsistable_MASK: u32 = 0x800;
        /// HEXSTABLE[12:12]
        pub const hexstable_POS: u32 = 12;
        pub const hexstable_MASK: u32 = 0x1000;
        /// LSISTABLE[14:14]
        pub const lsistable_POS: u32 = 14;
        pub const lsistable_MASK: u32 = 0x4000;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// HSIRDY[0:0]
        pub const hsirdy_POS: u32 = 0;
        pub const hsirdy_MASK: u32 = 0x1;
        /// HEXRDY[1:1]
        pub const hexrdy_POS: u32 = 1;
        pub const hexrdy_MASK: u32 = 0x2;
        /// LSIRDY[3:3]
        pub const lsirdy_POS: u32 = 3;
        pub const lsirdy_MASK: u32 = 0x8;
    }
    /// HSI 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod hsi {
        /// TRIM[10:0]
        pub const trim_POS: u32 = 0;
        pub const trim_MASK: u32 = 0x7ff;
        /// DIV[14:11]
        pub const div_POS: u32 = 11;
        pub const div_MASK: u32 = 0x7800;
        /// STABLE[15:15]
        pub const stable_POS: u32 = 15;
        pub const stable_MASK: u32 = 0x8000;
    }
    /// HEX 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod hex {
        /// PINEN[6:6]
        pub const pinen_POS: u32 = 6;
        pub const pinen_MASK: u32 = 0x40;
        /// PINMUX[7:7]
        pub const pinmux_POS: u32 = 7;
        pub const pinmux_MASK: u32 = 0x80;
        /// STABLE[19:19]
        pub const stable_POS: u32 = 19;
        pub const stable_MASK: u32 = 0x80000;
    }
    /// LSI 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod lsi {
        /// TRIM[9:0]
        pub const trim_POS: u32 = 0;
        pub const trim_MASK: u32 = 0x3ff;
        /// WAITCYCLE[11:10]
        pub const waitcycle_POS: u32 = 10;
        pub const waitcycle_MASK: u32 = 0xc00;
        /// STABLE[15:15]
        pub const stable_POS: u32 = 15;
        pub const stable_MASK: u32 = 0x8000;
    }
    /// DEBUG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod debug {
        /// ATIM[0:0]
        pub const atim_POS: u32 = 0;
        pub const atim_MASK: u32 = 0x1;
        /// GTIM[1:1]
        pub const gtim_POS: u32 = 1;
        pub const gtim_MASK: u32 = 0x2;
        /// BTIM123[5:5]
        pub const btim123_POS: u32 = 5;
        pub const btim123_MASK: u32 = 0x20;
        /// AWT[6:6]
        pub const awt_POS: u32 = 6;
        pub const awt_MASK: u32 = 0x40;
        /// IWDT[9:9]
        pub const iwdt_POS: u32 = 9;
        pub const iwdt_MASK: u32 = 0x200;
        /// WWDT[10:10]
        pub const wwdt_POS: u32 = 10;
        pub const wwdt_MASK: u32 = 0x400;
    }
    /// AHBEN 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ahben {
        /// FLASH[1:1]
        pub const flash_POS: u32 = 1;
        pub const flash_MASK: u32 = 0x2;
        /// CRC[2:2]
        pub const crc_POS: u32 = 2;
        pub const crc_MASK: u32 = 0x4;
        /// GPIOA[4:4]
        pub const gpioa_POS: u32 = 4;
        pub const gpioa_MASK: u32 = 0x10;
        /// GPIOB[5:5]
        pub const gpiob_POS: u32 = 5;
        pub const gpiob_MASK: u32 = 0x20;
        /// GPIOC[6:6]
        pub const gpioc_POS: u32 = 6;
        pub const gpioc_MASK: u32 = 0x40;
    }
    /// APBEN2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apben2 {
        /// ADC[2:2]
        pub const adc_POS: u32 = 2;
        pub const adc_MASK: u32 = 0x4;
        /// VC[4:4]
        pub const vc_POS: u32 = 4;
        pub const vc_MASK: u32 = 0x10;
        /// ATIM[7:7]
        pub const atim_POS: u32 = 7;
        pub const atim_MASK: u32 = 0x80;
        /// SPI[8:8]
        pub const spi_POS: u32 = 8;
        pub const spi_MASK: u32 = 0x100;
        /// UART1[9:9]
        pub const uart1_POS: u32 = 9;
        pub const uart1_MASK: u32 = 0x200;
        /// BTIM[12:12]
        pub const btim_POS: u32 = 12;
        pub const btim_MASK: u32 = 0x1000;
        /// AWT[13:13]
        pub const awt_POS: u32 = 13;
        pub const awt_MASK: u32 = 0x2000;
    }
    /// APBEN1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apben1 {
        /// GTIM[1:1]
        pub const gtim_POS: u32 = 1;
        pub const gtim_MASK: u32 = 0x2;
        /// WWDT[4:4]
        pub const wwdt_POS: u32 = 4;
        pub const wwdt_MASK: u32 = 0x10;
        /// IWDT[5:5]
        pub const iwdt_POS: u32 = 5;
        pub const iwdt_MASK: u32 = 0x20;
        /// UART2[7:7]
        pub const uart2_POS: u32 = 7;
        pub const uart2_MASK: u32 = 0x80;
        /// I2C[11:11]
        pub const i2c_POS: u32 = 11;
        pub const i2c_MASK: u32 = 0x800;
    }
    /// AHBRST 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ahbrst {
        /// FLASH[1:1]
        pub const flash_POS: u32 = 1;
        pub const flash_MASK: u32 = 0x2;
        /// CRC[2:2]
        pub const crc_POS: u32 = 2;
        pub const crc_MASK: u32 = 0x4;
        /// GPIOA[4:4]
        pub const gpioa_POS: u32 = 4;
        pub const gpioa_MASK: u32 = 0x10;
        /// GPIOB[5:5]
        pub const gpiob_POS: u32 = 5;
        pub const gpiob_MASK: u32 = 0x20;
        /// GPIOC[6:6]
        pub const gpioc_POS: u32 = 6;
        pub const gpioc_MASK: u32 = 0x40;
    }
    /// APBRST2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apbrst2 {
        /// ADC[2:2]
        pub const adc_POS: u32 = 2;
        pub const adc_MASK: u32 = 0x4;
        /// VC[4:4]
        pub const vc_POS: u32 = 4;
        pub const vc_MASK: u32 = 0x10;
        /// ATIM[7:7]
        pub const atim_POS: u32 = 7;
        pub const atim_MASK: u32 = 0x80;
        /// SPI[8:8]
        pub const spi_POS: u32 = 8;
        pub const spi_MASK: u32 = 0x100;
        /// UART1[9:9]
        pub const uart1_POS: u32 = 9;
        pub const uart1_MASK: u32 = 0x200;
        /// BTIM[12:12]
        pub const btim_POS: u32 = 12;
        pub const btim_MASK: u32 = 0x1000;
        /// AWT[13:13]
        pub const awt_POS: u32 = 13;
        pub const awt_MASK: u32 = 0x2000;
    }
    /// APBRST1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apbrst1 {
        /// GTIM[1:1]
        pub const gtim_POS: u32 = 1;
        pub const gtim_MASK: u32 = 0x2;
        /// WWDT[4:4]
        pub const wwdt_POS: u32 = 4;
        pub const wwdt_MASK: u32 = 0x10;
        /// IWDT[5:5]
        pub const iwdt_POS: u32 = 5;
        pub const iwdt_MASK: u32 = 0x20;
        /// UART2[7:7]
        pub const uart2_POS: u32 = 7;
        pub const uart2_MASK: u32 = 0x80;
        /// I2C[11:11]
        pub const i2c_POS: u32 = 11;
        pub const i2c_MASK: u32 = 0x800;
    }
    /// RESETFLAG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod resetflag {
        /// POR[0:0]
        pub const por_POS: u32 = 0;
        pub const por_MASK: u32 = 0x1;
        /// LVD[3:3]
        pub const lvd_POS: u32 = 3;
        pub const lvd_MASK: u32 = 0x8;
        /// IWDT[4:4]
        pub const iwdt_POS: u32 = 4;
        pub const iwdt_MASK: u32 = 0x10;
        /// WWDT[5:5]
        pub const wwdt_POS: u32 = 5;
        pub const wwdt_MASK: u32 = 0x20;
        /// RSTB[6:6]
        pub const rstb_POS: u32 = 6;
        pub const rstb_MASK: u32 = 0x40;
        /// LOCKUP[8:8]
        pub const lockup_POS: u32 = 8;
        pub const lockup_MASK: u32 = 0x100;
        /// SYSRESETREQ[9:9]
        pub const sysresetreq_POS: u32 = 9;
        pub const sysresetreq_MASK: u32 = 0x200;
    }
    /// GTIMCAP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod gtimcap {
        /// CH1[2:0]
        pub const ch1_POS: u32 = 0;
        pub const ch1_MASK: u32 = 0x7;
        /// CH2[6:4]
        pub const ch2_POS: u32 = 4;
        pub const ch2_MASK: u32 = 0x70;
        /// CH3[10:8]
        pub const ch3_POS: u32 = 8;
        pub const ch3_MASK: u32 = 0x700;
        /// CH4[14:12]
        pub const ch4_POS: u32 = 12;
        pub const ch4_MASK: u32 = 0x7000;
    }
    /// ATIMETR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod atimetr {
        /// ATIMETR[2:0]
        pub const atimetr_POS: u32 = 0;
        pub const atimetr_MASK: u32 = 0x7;
    }
    /// GTIMETR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod gtimetr {
        /// GTIMETR[2:0]
        pub const gtimetr_POS: u32 = 0;
        pub const gtimetr_MASK: u32 = 0x7;
    }
    /// TIMITR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod timitr {
        /// ATIMITR[2:0]
        pub const atimitr_POS: u32 = 0;
        pub const atimitr_MASK: u32 = 0x7;
        /// GTIMITR[5:3]
        pub const gtimitr_POS: u32 = 3;
        pub const gtimitr_MASK: u32 = 0x38;
        /// BTIM1ITR[17:15]
        pub const btim1itr_POS: u32 = 15;
        pub const btim1itr_MASK: u32 = 0x38000;
        /// BTIM2ITR[20:18]
        pub const btim2itr_POS: u32 = 18;
        pub const btim2itr_MASK: u32 = 0x1c0000;
        /// BTIM3ITR[23:21]
        pub const btim3itr_POS: u32 = 21;
        pub const btim3itr_MASK: u32 = 0xe00000;
    }
    /// MCO 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mco {
        /// SOURCE[3:0]
        pub const source_POS: u32 = 0;
        pub const source_MASK: u32 = 0xf;
        /// DIV[6:4]
        pub const div_POS: u32 = 4;
        pub const div_MASK: u32 = 0x70;
    }
    /// IRMOD 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod irmod {
        /// MOD[3:0]
        pub const mod_POS: u32 = 0;
        pub const mod_MASK: u32 = 0xf;
        /// IRSW[4:4]
        pub const irsw_POS: u32 = 4;
        pub const irsw_MASK: u32 = 0x10;
    }
}
/// `UART` 位段常量
pub mod uart {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// TXEN[0:0]
        pub const txen_POS: u32 = 0;
        pub const txen_MASK: u32 = 0x1;
        /// RXEN[1:1]
        pub const rxen_POS: u32 = 1;
        pub const rxen_MASK: u32 = 0x2;
        /// PARITY[3:2]
        pub const parity_POS: u32 = 2;
        pub const parity_MASK: u32 = 0xc;
        /// STOP[5:4]
        pub const stop_POS: u32 = 4;
        pub const stop_MASK: u32 = 0x30;
        /// SYNC[6:6]
        pub const sync_POS: u32 = 6;
        pub const sync_MASK: u32 = 0x40;
        /// START[8:8]
        pub const start_POS: u32 = 8;
        pub const start_MASK: u32 = 0x100;
        /// OVER[10:9]
        pub const over_POS: u32 = 9;
        pub const over_MASK: u32 = 0x600;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// ADDREN[0:0]
        pub const addren_POS: u32 = 0;
        pub const addren_MASK: u32 = 0x1;
        /// SIGNAL[1:1]
        pub const signal_POS: u32 = 1;
        pub const signal_MASK: u32 = 0x2;
        /// CTSEN[2:2]
        pub const ctsen_POS: u32 = 2;
        pub const ctsen_MASK: u32 = 0x4;
        /// RTSEN[3:3]
        pub const rtsen_POS: u32 = 3;
        pub const rtsen_MASK: u32 = 0x8;
        /// RXINV[4:4]
        pub const rxinv_POS: u32 = 4;
        pub const rxinv_MASK: u32 = 0x10;
        /// TXINV[5:5]
        pub const txinv_POS: u32 = 5;
        pub const txinv_MASK: u32 = 0x20;
        /// SOURCE[9:8]
        pub const source_POS: u32 = 8;
        pub const source_MASK: u32 = 0x300;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// TC[1:1]
        pub const tc_POS: u32 = 1;
        pub const tc_MASK: u32 = 0x2;
        /// RC[2:2]
        pub const rc_POS: u32 = 2;
        pub const rc_MASK: u32 = 0x4;
        /// FE[3:3]
        pub const fe_POS: u32 = 3;
        pub const fe_MASK: u32 = 0x8;
        /// PE[4:4]
        pub const pe_POS: u32 = 4;
        pub const pe_MASK: u32 = 0x10;
        /// CTS[6:6]
        pub const cts_POS: u32 = 6;
        pub const cts_MASK: u32 = 0x40;
    }
    /// BRRI 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod brri {
        /// BRRI[15:0]
        pub const brri_POS: u32 = 0;
        pub const brri_MASK: u32 = 0xffff;
    }
    /// BRRF 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod brrf {
        /// BRRF[3:0]
        pub const brrf_POS: u32 = 0;
        pub const brrf_MASK: u32 = 0xf;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// TC[1:1]
        pub const tc_POS: u32 = 1;
        pub const tc_MASK: u32 = 0x2;
        /// RC[2:2]
        pub const rc_POS: u32 = 2;
        pub const rc_MASK: u32 = 0x4;
        /// FE[3:3]
        pub const fe_POS: u32 = 3;
        pub const fe_MASK: u32 = 0x8;
        /// PE[4:4]
        pub const pe_POS: u32 = 4;
        pub const pe_MASK: u32 = 0x10;
        /// MATCH[5:5]
        pub const r#match_POS: u32 = 5;
        pub const r#match_MASK: u32 = 0x20;
        /// CTS[6:6]
        pub const cts_POS: u32 = 6;
        pub const cts_MASK: u32 = 0x40;
        /// CTSLV[7:7]
        pub const ctslv_POS: u32 = 7;
        pub const ctslv_MASK: u32 = 0x80;
        /// TXBUSY[8:8]
        pub const txbusy_POS: u32 = 8;
        pub const txbusy_MASK: u32 = 0x100;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// TC[1:1]
        pub const tc_POS: u32 = 1;
        pub const tc_MASK: u32 = 0x2;
        /// RC[2:2]
        pub const rc_POS: u32 = 2;
        pub const rc_MASK: u32 = 0x4;
        /// FE[3:3]
        pub const fe_POS: u32 = 3;
        pub const fe_MASK: u32 = 0x8;
        /// PE[4:4]
        pub const pe_POS: u32 = 4;
        pub const pe_MASK: u32 = 0x10;
        /// CTS[6:6]
        pub const cts_POS: u32 = 6;
        pub const cts_MASK: u32 = 0x40;
    }
    /// RDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod rdr {
        /// RDR[8:0]
        pub const rdr_POS: u32 = 0;
        pub const rdr_MASK: u32 = 0x1ff;
    }
    /// TDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tdr {
        /// TDR[8:0]
        pub const tdr_POS: u32 = 0;
        pub const tdr_MASK: u32 = 0x1ff;
    }
    /// ADDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod addr {
        /// ADDR[7:0]
        pub const addr_POS: u32 = 0;
        pub const addr_MASK: u32 = 0xff;
    }
    /// MASK 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mask {
        /// MASK[7:0]
        pub const mask_POS: u32 = 0;
        pub const mask_MASK: u32 = 0xff;
    }
}
/// `VC` 位段常量
pub mod vc {
    /// DIV 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod div {
        /// DIV[5:0]
        pub const div_POS: u32 = 0;
        pub const div_MASK: u32 = 0x3f;
        /// EN[6:6]
        pub const en_POS: u32 = 6;
        pub const en_MASK: u32 = 0x40;
        /// VIN[7:7]
        pub const vin_POS: u32 = 7;
        pub const vin_MASK: u32 = 0x80;
    }
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// RESP[2:1]
        pub const resp_POS: u32 = 1;
        pub const resp_MASK: u32 = 0x6;
        /// HYS[4:3]
        pub const hys_POS: u32 = 3;
        pub const hys_MASK: u32 = 0x18;
        /// IE[5:5]
        pub const ie_POS: u32 = 5;
        pub const ie_MASK: u32 = 0x20;
        /// POL[6:6]
        pub const pol_POS: u32 = 6;
        pub const pol_MASK: u32 = 0x40;
        /// WINDOW[7:7]
        pub const window_POS: u32 = 7;
        pub const window_MASK: u32 = 0x80;
        /// INP[11:8]
        pub const inp_POS: u32 = 8;
        pub const inp_MASK: u32 = 0xf00;
        /// INN[15:12]
        pub const inn_POS: u32 = 12;
        pub const inn_MASK: u32 = 0xf000;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// FLTEN[0:0]
        pub const flten_POS: u32 = 0;
        pub const flten_MASK: u32 = 0x1;
        /// FLTTIME[3:1]
        pub const flttime_POS: u32 = 1;
        pub const flttime_MASK: u32 = 0xe;
        /// FLTCLK[4:4]
        pub const fltclk_POS: u32 = 4;
        pub const fltclk_MASK: u32 = 0x10;
        /// FALLIE[5:5]
        pub const fallie_POS: u32 = 5;
        pub const fallie_MASK: u32 = 0x20;
        /// RISEIE[6:6]
        pub const riseie_POS: u32 = 6;
        pub const riseie_MASK: u32 = 0x40;
        /// HIGHIE[7:7]
        pub const highie_POS: u32 = 7;
        pub const highie_MASK: u32 = 0x80;
        /// ATIMCLR[8:8]
        pub const atimclr_POS: u32 = 8;
        pub const atimclr_MASK: u32 = 0x100;
        /// ATIMBK[9:9]
        pub const atimbk_POS: u32 = 9;
        pub const atimbk_MASK: u32 = 0x200;
        /// BLANKCH1B[10:10]
        pub const blankch1b_POS: u32 = 10;
        pub const blankch1b_MASK: u32 = 0x400;
        /// BLANKCH2B[11:11]
        pub const blankch2b_POS: u32 = 11;
        pub const blankch2b_MASK: u32 = 0x800;
        /// BLANKCH3B[12:12]
        pub const blankch3b_POS: u32 = 12;
        pub const blankch3b_MASK: u32 = 0x1000;
        /// BLANKFLT[15:13]
        pub const blankflt_POS: u32 = 13;
        pub const blankflt_MASK: u32 = 0xe000;
    }
    /// SR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sr {
        /// INTF[0:0]
        pub const intf_POS: u32 = 0;
        pub const intf_MASK: u32 = 0x1;
        /// FLTV[1:1]
        pub const fltv_POS: u32 = 1;
        pub const fltv_MASK: u32 = 0x2;
        /// READY[2:2]
        pub const ready_POS: u32 = 2;
        pub const ready_MASK: u32 = 0x4;
    }
}
/// `WWDT` 位段常量
pub mod wwdt {
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// WCNT[6:0]
        pub const wcnt_POS: u32 = 0;
        pub const wcnt_MASK: u32 = 0x7f;
        /// EN[7:7]
        pub const en_POS: u32 = 7;
        pub const en_MASK: u32 = 0x80;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// WINR[6:0]
        pub const winr_POS: u32 = 0;
        pub const winr_MASK: u32 = 0x7f;
        /// PRS[9:7]
        pub const prs_POS: u32 = 7;
        pub const prs_MASK: u32 = 0x380;
        /// IE[10:10]
        pub const ie_POS: u32 = 10;
        pub const ie_MASK: u32 = 0x400;
    }
    /// SR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sr {
        /// POV[0:0]
        pub const pov_POS: u32 = 0;
        pub const pov_MASK: u32 = 0x1;
    }
}

// ================= 中断号 =================
/// 外部中断号(IRQ0..28,厂商 IRQn 枚举)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    /// IRQ0:WDT
    Wdt = 0,
    /// IRQ1:LVD
    Lvd = 1,
    /// IRQ2:RESERVED2
    Reserved2 = 2,
    /// IRQ3:FLASHRAM
    Flashram = 3,
    /// IRQ4:RCC
    Rcc = 4,
    /// IRQ5:GPIOA
    Gpioa = 5,
    /// IRQ6:GPIOB
    Gpiob = 6,
    /// IRQ7:GPIOC
    Gpioc = 7,
    /// IRQ8:RESERVED8
    Reserved8 = 8,
    /// IRQ9:RESERVED9
    Reserved9 = 9,
    /// IRQ10:RESERVED10
    Reserved10 = 10,
    /// IRQ11:RESERVED11
    Reserved11 = 11,
    /// IRQ12:ADC
    Adc = 12,
    /// IRQ13:ATIM
    Atim = 13,
    /// IRQ14:VC1
    Vc1 = 14,
    /// IRQ15:VC2
    Vc2 = 15,
    /// IRQ16:GTIM
    Gtim = 16,
    /// IRQ17:RESERVED17
    Reserved17 = 17,
    /// IRQ18:RESERVED18
    Reserved18 = 18,
    /// IRQ19:RESERVED19
    Reserved19 = 19,
    /// IRQ20:BTIM1
    Btim1 = 20,
    /// IRQ21:BTIM2
    Btim2 = 21,
    /// IRQ22:BTIM3
    Btim3 = 22,
    /// IRQ23:I2C
    I2C = 23,
    /// IRQ24:RESERVED24
    Reserved24 = 24,
    /// IRQ25:SPI
    Spi = 25,
    /// IRQ26:RESERVED26
    Reserved26 = 26,
    /// IRQ27:UART1
    Uart1 = 27,
    /// IRQ28:UART2
    Uart2 = 28,
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
        /// IRQ0:向量符号 `WDT`
        WDT = 0;
        /// IRQ1:向量符号 `LVD`
        LVD = 1;
        /// IRQ2:向量符号 `RESERVED2`
        RESERVED2 = 2;
        /// IRQ3:向量符号 `FLASHRAM`
        FLASHRAM = 3;
        /// IRQ4:向量符号 `RCC`
        RCC = 4;
        /// IRQ5:向量符号 `GPIOA`
        GPIOA = 5;
        /// IRQ6:向量符号 `GPIOB`
        GPIOB = 6;
        /// IRQ7:向量符号 `GPIOC`
        GPIOC = 7;
        /// IRQ8:向量符号 `RESERVED8`
        RESERVED8 = 8;
        /// IRQ9:向量符号 `RESERVED9`
        RESERVED9 = 9;
        /// IRQ10:向量符号 `RESERVED10`
        RESERVED10 = 10;
        /// IRQ11:向量符号 `RESERVED11`
        RESERVED11 = 11;
        /// IRQ12:向量符号 `ADC`
        ADC = 12;
        /// IRQ13:向量符号 `ATIM`
        ATIM = 13;
        /// IRQ14:向量符号 `VC1`
        VC1 = 14;
        /// IRQ15:向量符号 `VC2`
        VC2 = 15;
        /// IRQ16:向量符号 `GTIM`
        GTIM = 16;
        /// IRQ17:向量符号 `RESERVED17`
        RESERVED17 = 17;
        /// IRQ18:向量符号 `RESERVED18`
        RESERVED18 = 18;
        /// IRQ19:向量符号 `RESERVED19`
        RESERVED19 = 19;
        /// IRQ20:向量符号 `BTIM1`
        BTIM1 = 20;
        /// IRQ21:向量符号 `BTIM2`
        BTIM2 = 21;
        /// IRQ22:向量符号 `BTIM3`
        BTIM3 = 22;
        /// IRQ23:向量符号 `I2C`
        I2C = 23;
        /// IRQ24:向量符号 `RESERVED24`
        RESERVED24 = 24;
        /// IRQ25:向量符号 `SPI`
        SPI = 25;
        /// IRQ26:向量符号 `RESERVED26`
        RESERVED26 = 26;
        /// IRQ27:向量符号 `UART1`
        UART1 = 27;
        /// IRQ28:向量符号 `UART2`
        UART2 = 28;
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
    /// ATIM
    pub atim: &'static ATIM,
    /// AWT
    pub awt: &'static AWT,
    /// BTIM1
    pub btim1: &'static BTIM,
    /// BTIM2
    pub btim2: &'static BTIM,
    /// BTIM3
    pub btim3: &'static BTIM,
    /// CRC
    pub crc: &'static CRC,
    /// FLASH
    pub flash: &'static FLASH,
    /// GPIOA
    pub gpioa: &'static GPIO,
    /// GPIOB
    pub gpiob: &'static GPIO,
    /// GPIOC
    pub gpioc: &'static GPIOC,
    /// GTIM
    pub gtim: &'static GTIM,
    /// I2C
    pub i2_c: &'static I2C,
    /// IWDT
    pub iwdt: &'static IWDT,
    /// LVD
    pub lvd: &'static LVD,
    /// RAM
    pub ram: &'static RAM,
    /// SPI
    pub spi: &'static SPI,
    /// SYSCTRL
    pub sysctrl: &'static SYSCTRL,
    /// UART1
    pub uart1: &'static UART,
    /// UART2
    pub uart2: &'static UART,
    /// VC1
    pub vc1: &'static VC,
    /// VC2
    pub vc2: &'static VC,
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
            atim: unsafe { &*(ATIM_BASE as *const ATIM) },
            awt: unsafe { &*(AWT_BASE as *const AWT) },
            btim1: unsafe { &*(BTIM1_BASE as *const BTIM) },
            btim2: unsafe { &*(BTIM2_BASE as *const BTIM) },
            btim3: unsafe { &*(BTIM3_BASE as *const BTIM) },
            crc: unsafe { &*(CRC_BASE as *const CRC) },
            flash: unsafe { &*(FLASH_BASE as *const FLASH) },
            gpioa: unsafe { &*(GPIOA_BASE as *const GPIO) },
            gpiob: unsafe { &*(GPIOB_BASE as *const GPIO) },
            gpioc: unsafe { &*(GPIOC_BASE as *const GPIOC) },
            gtim: unsafe { &*(GTIM_BASE as *const GTIM) },
            i2_c: unsafe { &*(I2C_BASE as *const I2C) },
            iwdt: unsafe { &*(IWDT_BASE as *const IWDT) },
            lvd: unsafe { &*(LVD_BASE as *const LVD) },
            ram: unsafe { &*(RAM_BASE as *const RAM) },
            spi: unsafe { &*(SPI_BASE as *const SPI) },
            sysctrl: unsafe { &*(SYSCTRL_BASE as *const SYSCTRL) },
            uart1: unsafe { &*(UART1_BASE as *const UART) },
            uart2: unsafe { &*(UART2_BASE as *const UART) },
            vc1: unsafe { &*(VC1_BASE as *const VC) },
            vc2: unsafe { &*(VC2_BASE as *const VC) },
            wwdt: unsafe { &*(WWDT_BASE as *const WWDT) },
        }
    }
}

// ================= 布局静态断言(厂商头 Size 注释) =================
const _: () = {
    assert!(core::mem::size_of::<ADC>() == 140,
        "ADC 布局错误:厂商头文件 Size = 140");
    assert!(core::mem::size_of::<ATIM>() == 92,
        "ATIM 布局错误:厂商头文件 Size = 92");
    assert!(core::mem::size_of::<AWT>() == 32,
        "AWT 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<BTIM>() == 32,
        "BTIM 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<CRC>() == 16,
        "CRC 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<FLASH>() == 44,
        "FLASH 布局错误:厂商头文件 Size = 44");
    assert!(core::mem::size_of::<GPIO>() == 100,
        "GPIO 布局错误:厂商头文件 Size = 100");
    assert!(core::mem::size_of::<GPIOC>() == 100,
        "GPIOC 布局错误:厂商头文件 Size = 100");
    assert!(core::mem::size_of::<GTIM>() == 820,
        "GTIM 布局错误:厂商头文件 Size = 820");
    assert!(core::mem::size_of::<I2C>() == 44,
        "I2C 布局错误:厂商头文件 Size = 44");
    assert!(core::mem::size_of::<IWDT>() == 40,
        "IWDT 布局错误:厂商头文件 Size = 40");
    assert!(core::mem::size_of::<LVD>() == 12,
        "LVD 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<RAM>() == 16,
        "RAM 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<SPI>() == 28,
        "SPI 布局错误:厂商头文件 Size = 28");
    assert!(core::mem::size_of::<SYSCTRL>() == 120,
        "SYSCTRL 布局错误:厂商头文件 Size = 120");
    assert!(core::mem::size_of::<UART>() == 56,
        "UART 布局错误:厂商头文件 Size = 56");
    assert!(core::mem::size_of::<VC>() == 16,
        "VC 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<WWDT>() == 12,
        "WWDT 布局错误:厂商头文件 Size = 12");
};