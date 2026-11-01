//! # cw32l010-pac
//!
//! CW32L010 (Cortex-M0+) 的寄存器访问层,由 cw32-pac-gen 从厂商
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
pub const ADC_BASE: usize = 0x40000000;
/// ATIM(ATIM)基地址
pub const ATIM_BASE: usize = 0x40001400;
/// BTIM1(BTIM)基地址
pub const BTIM1_BASE: usize = 0x40004800;
/// BTIM2(BTIM)基地址
pub const BTIM2_BASE: usize = 0x40004840;
/// BTIM3(BTIM)基地址
pub const BTIM3_BASE: usize = 0x40004880;
/// CRC(CRC)基地址
pub const CRC_BASE: usize = 0x40023000;
/// FLASH(FLASH)基地址
pub const FLASH_BASE: usize = 0x40022000;
/// GPIOA(GPIO)基地址
pub const GPIOA_BASE: usize = 0x48000000;
/// GPIOB(GPIO)基地址
pub const GPIOB_BASE: usize = 0x48000100;
/// GTIM1(GTIM)基地址
pub const GTIM1_BASE: usize = 0x40001800;
/// I2C1(I2C)基地址
pub const I2C1_BASE: usize = 0x40005400;
/// IRMOD(IRMOD)基地址
pub const IRMOD_BASE: usize = 0x40004080;
/// IWDT(IWDT)基地址
pub const IWDT_BASE: usize = 0x40005000;
/// LPTIM(LPTIM)基地址
pub const LPTIM_BASE: usize = 0x40006000;
/// RAM(RAM)基地址
pub const RAM_BASE: usize = 0x40022400;
/// UART1(UART)基地址
pub const UART1_BASE: usize = 0x40000c00;
/// UART2(UART)基地址
pub const UART2_BASE: usize = 0x40001000;
/// LVD(LVD)基地址
pub const LVD_BASE: usize = 0x400000d0;
/// RTC(RTC)基地址
pub const RTC_BASE: usize = 0x40004400;
/// SPI(SPI)基地址
pub const SPI_BASE: usize = 0x40000800;
/// SYSCTRL(SYSCTRL)基地址
pub const SYSCTRL_BASE: usize = 0x40004000;
/// VC1(VC)基地址
pub const VC1_BASE: usize = 0x400000a4;
/// VC2(VC)基地址
pub const VC2_BASE: usize = 0x400000b4;
/// VCREF(VCREF)基地址
pub const VCREF_BASE: usize = 0x400000a0;

// ================= 寄存器块 =================

/// `ADC` 寄存器组(生成自厂商头文件)
///
/// - `ADC` 基地址 `0X40000000`
#[allow(dead_code)]
#[repr(C)]
pub struct ADC {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    _pad0: [u32; 1], // 0x04..0x08
    /// 0x08 START(RW)
    pub start: Rw<u32>,
    _pad1: [u32; 1], // 0x0c..0x10
    /// 0x10 AWDTR(RW)
    pub awdtr: Rw<u32>,
    _pad2: [u32; 1], // 0x14..0x18
    /// 0x18 TRIGGER(RW)
    pub trigger: Rw<u32>,
    _pad3: [u32; 1], // 0x1c..0x20
    /// 0x20 AWDCR(RW)
    pub awdcr: Rw<u32>,
    _pad4: [u32; 1], // 0x24..0x28
    /// 0x28 SAMPLE(RW)
    pub sample: Rw<u32>,
    /// 0x2C SQRCFR(RW)
    pub sqrcfr: Rw<u32>,
    _pad5: [u32; 4], // 0x30..0x40
    /// 0x40 RESULT0(RO)
    pub result0: Ro<u32>,
    /// 0x44 RESULT1(RO)
    pub result1: Ro<u32>,
    /// 0x48 RESULT2(RO)
    pub result2: Ro<u32>,
    /// 0x4C RESULT3(RO)
    pub result3: Ro<u32>,
    /// 0x50 RESULT4(RO)
    pub result4: Ro<u32>,
    /// 0x54 RESULT5(RO)
    pub result5: Ro<u32>,
    /// 0x58 RESULT6(RO)
    pub result6: Ro<u32>,
    /// 0x5C RESULT7(RO)
    pub result7: Ro<u32>,
    _pad6: [u32; 5], // 0x60..0x74
    /// 0x74 IER(RW)
    pub ier: Rw<u32>,
    /// 0x78 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x7C ISR(RO)
    pub isr: Ro<u32>,
}

/// `ATIM` 寄存器组(生成自厂商头文件)
///
/// - `ATIM` 基地址 `0X40001400`
#[allow(dead_code)]
#[repr(C)]
pub struct ATIM {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 SMCR(RW)
    pub smcr: Rw<u32>,
    /// 0x0C IER(RW)
    pub ier: Rw<u32>,
    /// 0x10 ISR(RW)
    pub isr: Rw<u32>,
    /// 0x14 EGR(RW)
    pub egr: Rw<u32>,
    /// 0x18 CCMR1CAP(RW)
    pub ccmr1cap: Rw<u32>,
    /// 0x1C CCMR2CAP(RW)
    pub ccmr2cap: Rw<u32>,
    /// 0x20 CCER(RW)
    pub ccer: Rw<u32>,
    /// 0x24 CNT(RW)
    pub cnt: Rw<u32>,
    /// 0x28 PSC(RW)
    pub psc: Rw<u32>,
    /// 0x2C ARR(RW)
    pub arr: Rw<u32>,
    /// 0x30 RCR(RW)
    pub rcr: Rw<u32>,
    /// 0x34 CCR1(RW)
    pub ccr1: Rw<u32>,
    /// 0x38 CCR2(RW)
    pub ccr2: Rw<u32>,
    /// 0x3C CCR3(RW)
    pub ccr3: Rw<u32>,
    /// 0x40 CCR4(RW)
    pub ccr4: Rw<u32>,
    /// 0x44 BDTR(RW)
    pub bdtr: Rw<u32>,
    /// 0x48 CCR5(RW)
    pub ccr5: Rw<u32>,
    /// 0x4C CCR6(RW)
    pub ccr6: Rw<u32>,
    /// 0x50 CCMR3CAP(RW)
    pub ccmr3cap: Rw<u32>,
    /// 0x54 DTR2(RW)
    pub dtr2: Rw<u32>,
    /// 0x58 ECR(RW)
    pub ecr: Rw<u32>,
    /// 0x5C TISEL1(RW)
    pub tisel1: Rw<u32>,
    /// 0x60 AF1(RW)
    pub af1: Rw<u32>,
    /// 0x64 AF2(RW)
    pub af2: Rw<u32>,
    _pad0: [u32; 1], // 0x68..0x6c
    /// 0x6C TISEL2(RW)
    pub tisel2: Rw<u32>,
    /// 0x70 ICR(RW)
    pub icr: Rw<u32>,
}

/// `BTIM` 寄存器组(生成自厂商头文件)
///
/// - `BTIM1` 基地址 `0X40004800`
///
/// - `BTIM2` 基地址 `0X40004840`
///
/// - `BTIM3` 基地址 `0X40004880`
#[allow(dead_code)]
#[repr(C)]
pub struct BTIM {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 SMCR(RW)
    pub smcr: Rw<u32>,
    /// 0x0C IER(RW)
    pub ier: Rw<u32>,
    /// 0x10 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x14 EGR(WO)
    pub egr: Wo<u32>,
    /// 0x18 ICR(RW)
    pub icr: Rw<u32>,
    _pad0: [u32; 2], // 0x1c..0x24
    /// 0x24 CNT(RW)
    pub cnt: Rw<u32>,
    /// 0x28 PSC(RW)
    pub psc: Rw<u32>,
    /// 0x2C ARR(RW)
    pub arr: Rw<u32>,
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
    _pad1: [u32; 17], // 0x2c..0x70
    /// 0x70 SDKCFR(RO)
    pub sdkcfr: Ro<u32>,
}

/// `GPIO` 寄存器组(生成自厂商头文件)
///
/// - `GPIOA` 基地址 `0X48000000`
///
/// - `GPIOB` 基地址 `0X48000100`
#[allow(dead_code)]
#[repr(C)]
pub struct GPIO {
    /// 0x00 DIR(RW)
    pub dir: Rw<u32>,
    /// 0x04 OPENDRAIN(RW)
    pub opendrain: Rw<u32>,
    _pad0: [u32; 2], // 0x08..0x10
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
    _pad2: [u32; 2], // 0x2c..0x34
    /// 0x34 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
    _pad3: [u32; 1], // 0x3c..0x40
    /// 0x40 FILTER(RW)
    pub filter: Rw<u32>,
    _pad4: [u32; 3], // 0x44..0x50
    /// 0x50 IDR(RW)
    pub idr: Rw<u32>,
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
/// - `GTIM1` 基地址 `0X40001800`
#[allow(dead_code)]
#[repr(C)]
pub struct GTIM {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 SMCR(RW)
    pub smcr: Rw<u32>,
    /// 0x0C IER(RW)
    pub ier: Rw<u32>,
    /// 0x10 ISR(RW)
    pub isr: Rw<u32>,
    /// 0x14 EGR(RW)
    pub egr: Rw<u32>,
    /// 0x18 CCMR1CAP(RW)
    pub ccmr1cap: Rw<u32>,
    /// 0x1C CCMR2CAP(RW)
    pub ccmr2cap: Rw<u32>,
    /// 0x20 CCER(RW)
    pub ccer: Rw<u32>,
    /// 0x24 CNT(RW)
    pub cnt: Rw<u32>,
    /// 0x28 PSC(RW)
    pub psc: Rw<u32>,
    /// 0x2C ARR(RW)
    pub arr: Rw<u32>,
    _pad0: [u32; 1], // 0x30..0x34
    /// 0x34 CCR1(RW)
    pub ccr1: Rw<u32>,
    /// 0x38 CCR2(RW)
    pub ccr2: Rw<u32>,
    /// 0x3C CCR3(RW)
    pub ccr3: Rw<u32>,
    /// 0x40 CCR4(RW)
    pub ccr4: Rw<u32>,
    _pad1: [u32; 5], // 0x44..0x58
    /// 0x58 ECR(RW)
    pub ecr: Rw<u32>,
    /// 0x5C TISEL(RW)
    pub tisel: Rw<u32>,
    /// 0x60 AF1(RW)
    pub af1: Rw<u32>,
    /// 0x64 AF2(RW)
    pub af2: Rw<u32>,
    _pad2: [u32; 2], // 0x68..0x70
    /// 0x70 ICR(RW)
    pub icr: Rw<u32>,
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

/// `IRMOD` 寄存器组(生成自厂商头文件)
///
/// - `IRMOD` 基地址 `0X40004080`
#[allow(dead_code)]
#[repr(C)]
pub struct IRMOD {
    _pad0: [u32; 2], // 0x00..0x08
    /// 0x08 CR(RW)
    pub cr: Rw<u32>,
}

/// `IWDT` 寄存器组(生成自厂商头文件)
///
/// - `IWDT` 基地址 `0X40005000`
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

/// `LPTIM` 寄存器组(生成自厂商头文件)
///
/// - `LPTIM` 基地址 `0X40006000`
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

/// `UART` 寄存器组(生成自厂商头文件)
///
/// - `UART1` 基地址 `0X40000C00`
///
/// - `UART2` 基地址 `0X40001000`
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
    /// 0x14 TIMARR(RW)
    pub timarr: Rw<u32>,
    /// 0x18 TIMCNT(RO)
    pub timcnt: Ro<u32>,
    /// 0x1C ISR(RO)
    pub isr: Ro<u32>,
    /// 0x20 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x24 RDR(RO)
    pub rdr: Ro<u32>,
    /// 0x28 TDR(WO)
    pub tdr: Wo<u32>,
    _pad0: [u32; 1], // 0x2c..0x30
    /// 0x30 ADDR(RW)
    pub addr: Rw<u32>,
    /// 0x34 MASK(RW)
    pub mask: Rw<u32>,
    /// 0x38 CR3(RW)
    pub cr3: Rw<u32>,
    /// 0x3C RXMATCH(RW)
    pub rxmatch: Rw<u32>,
}

/// `LVD` 寄存器组(生成自厂商头文件)
///
/// - `LVD` 基地址 `0X400000D0`
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

/// `RTC` 寄存器组(生成自厂商头文件)
///
/// - `RTC` 基地址 `0X40004400`
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
    /// 0x10 COMPCFR1(RW)
    pub compcfr1: Rw<u32>,
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
    /// 0x3C AWTCNT(RO)
    pub awtcnt: Ro<u32>,
    /// 0x40 PSC(RW)
    pub psc: Rw<u32>,
    /// 0x44 SSCNT(RO)
    pub sscnt: Ro<u32>,
    _pad0: [u32; 3], // 0x48..0x54
    /// 0x54 COMPCFR2(RW)
    pub compcfr2: Rw<u32>,
    /// 0x58 COMPCFR3(RW)
    pub compcfr3: Rw<u32>,
}

/// `SPI` 寄存器组(生成自厂商头文件)
///
/// - `SPI` 基地址 `0X40000800`
#[allow(dead_code)]
#[repr(C)]
pub struct SPI {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 CR3(RW)
    pub cr3: Rw<u32>,
    /// 0x0C SSI(RW)
    pub ssi: Rw<u32>,
    /// 0x10 IER(RW)
    pub ier: Rw<u32>,
    /// 0x14 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x18 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x1C DR(RW)
    pub dr: Rw<u32>,
}

/// `SYSCTRL` 寄存器组(生成自厂商头文件)
///
/// - `SYSCTRL` 基地址 `0X40004000`
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
    _pad0: [u32; 1], // 0x28..0x2c
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
    _pad2: [u32; 8], // 0x50..0x70
    /// 0x70 MCO(RW)
    pub mco: Rw<u32>,
}

/// `VC` 寄存器组(生成自厂商头文件)
///
/// - `VC1` 基地址 `0X400000A4`
///
/// - `VC2` 基地址 `0X400000B4`
#[allow(dead_code)]
#[repr(C)]
pub struct VC {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 SR(RW)
    pub sr: Rw<u32>,
}

/// `VCREF` 寄存器组(生成自厂商头文件)
///
/// - `VCREF` 基地址 `0X400000A0`
#[allow(dead_code)]
#[repr(C)]
pub struct VCREF {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
}

// ================= 位段常量 =================
/// `ADC` 位段常量
pub mod adc {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// BGREN[1:1]
        pub const bgren_POS: u32 = 1;
        pub const bgren_MASK: u32 = 0x2;
        /// TSEN[2:2]
        pub const tsen_POS: u32 = 2;
        pub const tsen_MASK: u32 = 0x4;
        /// CONT[3:3]
        pub const cont_POS: u32 = 3;
        pub const cont_MASK: u32 = 0x8;
        /// CLK[5:4]
        pub const clk_POS: u32 = 4;
        pub const clk_MASK: u32 = 0x30;
        /// ENS[8:6]
        pub const ens_POS: u32 = 6;
        pub const ens_MASK: u32 = 0x1c0;
    }
    /// START 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod start {
        /// START[0:0]
        pub const start_POS: u32 = 0;
        pub const start_MASK: u32 = 0x1;
    }
    /// AWDTR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod awdtr {
        /// VTL[11:0]
        pub const vtl_POS: u32 = 0;
        pub const vtl_MASK: u32 = 0xfff;
        /// VTH[27:16]
        pub const vth_POS: u32 = 16;
        pub const vth_MASK: u32 = 0xfff0000;
    }
    /// TRIGGER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trigger {
        /// ATIMTRGO[0:0]
        pub const atimtrgo_POS: u32 = 0;
        pub const atimtrgo_MASK: u32 = 0x1;
        /// ATIMTRGO2[1:1]
        pub const atimtrgo2_POS: u32 = 1;
        pub const atimtrgo2_MASK: u32 = 0x2;
        /// ATIMCC1[2:2]
        pub const atimcc1_POS: u32 = 2;
        pub const atimcc1_MASK: u32 = 0x4;
        /// ATIMCC2[3:3]
        pub const atimcc2_POS: u32 = 3;
        pub const atimcc2_MASK: u32 = 0x8;
        /// ATIMCC3[4:4]
        pub const atimcc3_POS: u32 = 4;
        pub const atimcc3_MASK: u32 = 0x10;
        /// ATIMCC4[5:5]
        pub const atimcc4_POS: u32 = 5;
        pub const atimcc4_MASK: u32 = 0x20;
        /// ATIMCC5[6:6]
        pub const atimcc5_POS: u32 = 6;
        pub const atimcc5_MASK: u32 = 0x40;
        /// ATIMCC6[7:7]
        pub const atimcc6_POS: u32 = 7;
        pub const atimcc6_MASK: u32 = 0x80;
        /// GTIM1TRGO[8:8]
        pub const gtim1trgo_POS: u32 = 8;
        pub const gtim1trgo_MASK: u32 = 0x100;
        /// GTIM1CC1[9:9]
        pub const gtim1cc1_POS: u32 = 9;
        pub const gtim1cc1_MASK: u32 = 0x200;
        /// GTIM1CC2[10:10]
        pub const gtim1cc2_POS: u32 = 10;
        pub const gtim1cc2_MASK: u32 = 0x400;
        /// GTIM1CC3[11:11]
        pub const gtim1cc3_POS: u32 = 11;
        pub const gtim1cc3_MASK: u32 = 0x800;
        /// GTIM1CC4[12:12]
        pub const gtim1cc4_POS: u32 = 12;
        pub const gtim1cc4_MASK: u32 = 0x1000;
        /// BTIM1TRGO[13:13]
        pub const btim1trgo_POS: u32 = 13;
        pub const btim1trgo_MASK: u32 = 0x2000;
        /// BTIM2TRGO[14:14]
        pub const btim2trgo_POS: u32 = 14;
        pub const btim2trgo_MASK: u32 = 0x4000;
        /// BTIM3TRGO[15:15]
        pub const btim3trgo_POS: u32 = 15;
        pub const btim3trgo_MASK: u32 = 0x8000;
        /// SPI1[16:16]
        pub const spi1_POS: u32 = 16;
        pub const spi1_MASK: u32 = 0x10000;
        /// UART1[17:17]
        pub const uart1_POS: u32 = 17;
        pub const uart1_MASK: u32 = 0x20000;
        /// UART2[18:18]
        pub const uart2_POS: u32 = 18;
        pub const uart2_MASK: u32 = 0x40000;
    }
    /// AWDCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod awdcr {
        /// IN0[0:0]
        pub const in0_POS: u32 = 0;
        pub const in0_MASK: u32 = 0x1;
        /// IN1[1:1]
        pub const in1_POS: u32 = 1;
        pub const in1_MASK: u32 = 0x2;
        /// IN2[2:2]
        pub const in2_POS: u32 = 2;
        pub const in2_MASK: u32 = 0x4;
        /// IN3[3:3]
        pub const in3_POS: u32 = 3;
        pub const in3_MASK: u32 = 0x8;
        /// IN4[4:4]
        pub const in4_POS: u32 = 4;
        pub const in4_MASK: u32 = 0x10;
        /// IN5[5:5]
        pub const in5_POS: u32 = 5;
        pub const in5_MASK: u32 = 0x20;
        /// IN6[6:6]
        pub const in6_POS: u32 = 6;
        pub const in6_MASK: u32 = 0x40;
        /// IN7[7:7]
        pub const in7_POS: u32 = 7;
        pub const in7_MASK: u32 = 0x80;
        /// IN8[8:8]
        pub const in8_POS: u32 = 8;
        pub const in8_MASK: u32 = 0x100;
        /// IN9[9:9]
        pub const in9_POS: u32 = 9;
        pub const in9_MASK: u32 = 0x200;
        /// IN10[10:10]
        pub const in10_POS: u32 = 10;
        pub const in10_MASK: u32 = 0x400;
        /// IN11[11:11]
        pub const in11_POS: u32 = 11;
        pub const in11_MASK: u32 = 0x800;
        /// IN12[12:12]
        pub const in12_POS: u32 = 12;
        pub const in12_MASK: u32 = 0x1000;
        /// IN13[13:13]
        pub const in13_POS: u32 = 13;
        pub const in13_MASK: u32 = 0x2000;
        /// IN14[14:14]
        pub const in14_POS: u32 = 14;
        pub const in14_MASK: u32 = 0x4000;
        /// IN15[15:15]
        pub const in15_POS: u32 = 15;
        pub const in15_MASK: u32 = 0x8000;
    }
    /// SAMPLE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sample {
        /// SQRCH0[3:0]
        pub const sqrch0_POS: u32 = 0;
        pub const sqrch0_MASK: u32 = 0xf;
        /// SQRCH1[7:4]
        pub const sqrch1_POS: u32 = 4;
        pub const sqrch1_MASK: u32 = 0xf0;
        /// SQRCH2[11:8]
        pub const sqrch2_POS: u32 = 8;
        pub const sqrch2_MASK: u32 = 0xf00;
        /// SQRCH3[15:12]
        pub const sqrch3_POS: u32 = 12;
        pub const sqrch3_MASK: u32 = 0xf000;
        /// SQRCH4[19:16]
        pub const sqrch4_POS: u32 = 16;
        pub const sqrch4_MASK: u32 = 0xf0000;
        /// SQRCH5[23:20]
        pub const sqrch5_POS: u32 = 20;
        pub const sqrch5_MASK: u32 = 0xf00000;
        /// SQRCH6[27:24]
        pub const sqrch6_POS: u32 = 24;
        pub const sqrch6_MASK: u32 = 0xf000000;
        /// SQRCH7[31:28]
        pub const sqrch7_POS: u32 = 28;
        pub const sqrch7_MASK: u32 = 0xf0000000;
    }
    /// SQRCFR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sqrcfr {
        /// SQRCH0[3:0]
        pub const sqrch0_POS: u32 = 0;
        pub const sqrch0_MASK: u32 = 0xf;
        /// SQRCH1[7:4]
        pub const sqrch1_POS: u32 = 4;
        pub const sqrch1_MASK: u32 = 0xf0;
        /// SQRCH2[11:8]
        pub const sqrch2_POS: u32 = 8;
        pub const sqrch2_MASK: u32 = 0xf00;
        /// SQRCH3[15:12]
        pub const sqrch3_POS: u32 = 12;
        pub const sqrch3_MASK: u32 = 0xf000;
        /// SQRCH4[19:16]
        pub const sqrch4_POS: u32 = 16;
        pub const sqrch4_MASK: u32 = 0xf0000;
        /// SQRCH5[23:20]
        pub const sqrch5_POS: u32 = 20;
        pub const sqrch5_MASK: u32 = 0xf00000;
        /// SQRCH6[27:24]
        pub const sqrch6_POS: u32 = 24;
        pub const sqrch6_MASK: u32 = 0xf000000;
        /// SQRCH7[31:28]
        pub const sqrch7_POS: u32 = 28;
        pub const sqrch7_MASK: u32 = 0xf0000000;
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
    /// RESULT4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result4 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT5 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result5 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT6 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result6 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
    }
    /// RESULT7 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod result7 {
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
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
        /// AWDL[2:2]
        pub const awdl_POS: u32 = 2;
        pub const awdl_MASK: u32 = 0x4;
        /// AWDH[3:3]
        pub const awdh_POS: u32 = 3;
        pub const awdh_MASK: u32 = 0x8;
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
        /// AWDL[2:2]
        pub const awdl_POS: u32 = 2;
        pub const awdl_MASK: u32 = 0x4;
        /// AWDH[3:3]
        pub const awdh_POS: u32 = 3;
        pub const awdh_MASK: u32 = 0x8;
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
        /// AWDL[2:2]
        pub const awdl_POS: u32 = 2;
        pub const awdl_MASK: u32 = 0x4;
        /// AWDH[3:3]
        pub const awdh_POS: u32 = 3;
        pub const awdh_MASK: u32 = 0x8;
    }
}
/// `ATIM` 位段常量
pub mod atim {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// CEN[0:0]
        pub const cen_POS: u32 = 0;
        pub const cen_MASK: u32 = 0x1;
        /// UDIS[1:1]
        pub const udis_POS: u32 = 1;
        pub const udis_MASK: u32 = 0x2;
        /// URS[2:2]
        pub const urs_POS: u32 = 2;
        pub const urs_MASK: u32 = 0x4;
        /// OPM[3:3]
        pub const opm_POS: u32 = 3;
        pub const opm_MASK: u32 = 0x8;
        /// DIR[4:4]
        pub const dir_POS: u32 = 4;
        pub const dir_MASK: u32 = 0x10;
        /// CMS[6:5]
        pub const cms_POS: u32 = 5;
        pub const cms_MASK: u32 = 0x60;
        /// ARPE[7:7]
        pub const arpe_POS: u32 = 7;
        pub const arpe_MASK: u32 = 0x80;
        /// CKD[9:8]
        pub const ckd_POS: u32 = 8;
        pub const ckd_MASK: u32 = 0x300;
        /// UIFREMAP[11:11]
        pub const uifremap_POS: u32 = 11;
        pub const uifremap_MASK: u32 = 0x800;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// CCPC[0:0]
        pub const ccpc_POS: u32 = 0;
        pub const ccpc_MASK: u32 = 0x1;
        /// CCUS[2:2]
        pub const ccus_POS: u32 = 2;
        pub const ccus_MASK: u32 = 0x4;
        /// MMS[6:4]
        pub const mms_POS: u32 = 4;
        pub const mms_MASK: u32 = 0x70;
        /// TI1S[7:7]
        pub const ti1s_POS: u32 = 7;
        pub const ti1s_MASK: u32 = 0x80;
        /// OIS1[8:8]
        pub const ois1_POS: u32 = 8;
        pub const ois1_MASK: u32 = 0x100;
        /// OIS1N[9:9]
        pub const ois1n_POS: u32 = 9;
        pub const ois1n_MASK: u32 = 0x200;
        /// OIS2[10:10]
        pub const ois2_POS: u32 = 10;
        pub const ois2_MASK: u32 = 0x400;
        /// OIS2N[11:11]
        pub const ois2n_POS: u32 = 11;
        pub const ois2n_MASK: u32 = 0x800;
        /// OIS3[12:12]
        pub const ois3_POS: u32 = 12;
        pub const ois3_MASK: u32 = 0x1000;
        /// OIS3N[13:13]
        pub const ois3n_POS: u32 = 13;
        pub const ois3n_MASK: u32 = 0x2000;
        /// OIS4[14:14]
        pub const ois4_POS: u32 = 14;
        pub const ois4_MASK: u32 = 0x4000;
        /// OIS4N[15:15]
        pub const ois4n_POS: u32 = 15;
        pub const ois4n_MASK: u32 = 0x8000;
        /// OIS5[16:16]
        pub const ois5_POS: u32 = 16;
        pub const ois5_MASK: u32 = 0x10000;
        /// OIS5N[17:17]
        pub const ois5n_POS: u32 = 17;
        pub const ois5n_MASK: u32 = 0x20000;
        /// OIS6[18:18]
        pub const ois6_POS: u32 = 18;
        pub const ois6_MASK: u32 = 0x40000;
        /// OIS6N[19:19]
        pub const ois6n_POS: u32 = 19;
        pub const ois6n_MASK: u32 = 0x80000;
        /// MMS2[24:20]
        pub const mms2_POS: u32 = 20;
        pub const mms2_MASK: u32 = 0x1f00000;
        /// MMSH[26:25]
        pub const mmsh_POS: u32 = 25;
        pub const mmsh_MASK: u32 = 0x6000000;
    }
    /// SMCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod smcr {
        /// SMS[2:0]
        pub const sms_POS: u32 = 0;
        pub const sms_MASK: u32 = 0x7;
        /// OCCS[3:3]
        pub const occs_POS: u32 = 3;
        pub const occs_MASK: u32 = 0x8;
        /// TS[6:4]
        pub const ts_POS: u32 = 4;
        pub const ts_MASK: u32 = 0x70;
        /// MSM[7:7]
        pub const msm_POS: u32 = 7;
        pub const msm_MASK: u32 = 0x80;
        /// ETF[11:8]
        pub const etf_POS: u32 = 8;
        pub const etf_MASK: u32 = 0xf00;
        /// ETPS[13:12]
        pub const etps_POS: u32 = 12;
        pub const etps_MASK: u32 = 0x3000;
        /// ECE[14:14]
        pub const ece_POS: u32 = 14;
        pub const ece_MASK: u32 = 0x4000;
        /// ETP[15:15]
        pub const etp_POS: u32 = 15;
        pub const etp_MASK: u32 = 0x8000;
        /// SMSH[16:16]
        pub const smsh_POS: u32 = 16;
        pub const smsh_MASK: u32 = 0x10000;
        /// TSH[21:20]
        pub const tsh_POS: u32 = 20;
        pub const tsh_MASK: u32 = 0x300000;
        /// SMSPE[24:24]
        pub const smspe_POS: u32 = 24;
        pub const smspe_MASK: u32 = 0x1000000;
        /// SMSPS[25:25]
        pub const smsps_POS: u32 = 25;
        pub const smsps_MASK: u32 = 0x2000000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// UIE[0:0]
        pub const uie_POS: u32 = 0;
        pub const uie_MASK: u32 = 0x1;
        /// CC1IE[1:1]
        pub const cc1ie_POS: u32 = 1;
        pub const cc1ie_MASK: u32 = 0x2;
        /// CC2IE[2:2]
        pub const cc2ie_POS: u32 = 2;
        pub const cc2ie_MASK: u32 = 0x4;
        /// CC3IE[3:3]
        pub const cc3ie_POS: u32 = 3;
        pub const cc3ie_MASK: u32 = 0x8;
        /// CC4IE[4:4]
        pub const cc4ie_POS: u32 = 4;
        pub const cc4ie_MASK: u32 = 0x10;
        /// COMIE[5:5]
        pub const comie_POS: u32 = 5;
        pub const comie_MASK: u32 = 0x20;
        /// TIE[6:6]
        pub const tie_POS: u32 = 6;
        pub const tie_MASK: u32 = 0x40;
        /// BIE[7:7]
        pub const bie_POS: u32 = 7;
        pub const bie_MASK: u32 = 0x80;
        /// IDXIE[20:20]
        pub const idxie_POS: u32 = 20;
        pub const idxie_MASK: u32 = 0x100000;
        /// DIRIE[21:21]
        pub const dirie_POS: u32 = 21;
        pub const dirie_MASK: u32 = 0x200000;
        /// IERRIE[22:22]
        pub const ierrie_POS: u32 = 22;
        pub const ierrie_MASK: u32 = 0x400000;
        /// TERRIE[23:23]
        pub const terrie_POS: u32 = 23;
        pub const terrie_MASK: u32 = 0x800000;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// CC1IF[1:1]
        pub const cc1if_POS: u32 = 1;
        pub const cc1if_MASK: u32 = 0x2;
        /// CC2IF[2:2]
        pub const cc2if_POS: u32 = 2;
        pub const cc2if_MASK: u32 = 0x4;
        /// CC3IF[3:3]
        pub const cc3if_POS: u32 = 3;
        pub const cc3if_MASK: u32 = 0x8;
        /// CC4IF[4:4]
        pub const cc4if_POS: u32 = 4;
        pub const cc4if_MASK: u32 = 0x10;
        /// COMIF[5:5]
        pub const comif_POS: u32 = 5;
        pub const comif_MASK: u32 = 0x20;
        /// TIF[6:6]
        pub const tif_POS: u32 = 6;
        pub const tif_MASK: u32 = 0x40;
        /// BIF[7:7]
        pub const bif_POS: u32 = 7;
        pub const bif_MASK: u32 = 0x80;
        /// B2IF[8:8]
        pub const b2if_POS: u32 = 8;
        pub const b2if_MASK: u32 = 0x100;
        /// CC1OF[9:9]
        pub const cc1of_POS: u32 = 9;
        pub const cc1of_MASK: u32 = 0x200;
        /// CC2OF[10:10]
        pub const cc2of_POS: u32 = 10;
        pub const cc2of_MASK: u32 = 0x400;
        /// CC3OF[11:11]
        pub const cc3of_POS: u32 = 11;
        pub const cc3of_MASK: u32 = 0x800;
        /// CC4OF[12:12]
        pub const cc4of_POS: u32 = 12;
        pub const cc4of_MASK: u32 = 0x1000;
        /// SBIF[13:13]
        pub const sbif_POS: u32 = 13;
        pub const sbif_MASK: u32 = 0x2000;
        /// CC5IF[16:16]
        pub const cc5if_POS: u32 = 16;
        pub const cc5if_MASK: u32 = 0x10000;
        /// CC6IF[17:17]
        pub const cc6if_POS: u32 = 17;
        pub const cc6if_MASK: u32 = 0x20000;
        /// CC5OF[18:18]
        pub const cc5of_POS: u32 = 18;
        pub const cc5of_MASK: u32 = 0x40000;
        /// CC6OF[19:19]
        pub const cc6of_POS: u32 = 19;
        pub const cc6of_MASK: u32 = 0x80000;
        /// IDXF[20:20]
        pub const idxf_POS: u32 = 20;
        pub const idxf_MASK: u32 = 0x100000;
        /// DIRF[21:21]
        pub const dirf_POS: u32 = 21;
        pub const dirf_MASK: u32 = 0x200000;
        /// IERRF[22:22]
        pub const ierrf_POS: u32 = 22;
        pub const ierrf_MASK: u32 = 0x400000;
        /// TERRF[23:23]
        pub const terrf_POS: u32 = 23;
        pub const terrf_MASK: u32 = 0x800000;
    }
    /// EGR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod egr {
        /// UG[0:0]
        pub const ug_POS: u32 = 0;
        pub const ug_MASK: u32 = 0x1;
        /// CC1G[1:1]
        pub const cc1g_POS: u32 = 1;
        pub const cc1g_MASK: u32 = 0x2;
        /// CC2G[2:2]
        pub const cc2g_POS: u32 = 2;
        pub const cc2g_MASK: u32 = 0x4;
        /// CC3G[3:3]
        pub const cc3g_POS: u32 = 3;
        pub const cc3g_MASK: u32 = 0x8;
        /// CC4G[4:4]
        pub const cc4g_POS: u32 = 4;
        pub const cc4g_MASK: u32 = 0x10;
        /// COMG[5:5]
        pub const comg_POS: u32 = 5;
        pub const comg_MASK: u32 = 0x20;
        /// TG[6:6]
        pub const tg_POS: u32 = 6;
        pub const tg_MASK: u32 = 0x40;
        /// BG[7:7]
        pub const bg_POS: u32 = 7;
        pub const bg_MASK: u32 = 0x80;
        /// B2G[8:8]
        pub const b2g_POS: u32 = 8;
        pub const b2g_MASK: u32 = 0x100;
        /// CC5G[16:16]
        pub const cc5g_POS: u32 = 16;
        pub const cc5g_MASK: u32 = 0x10000;
        /// CC6G[17:17]
        pub const cc6g_POS: u32 = 17;
        pub const cc6g_MASK: u32 = 0x20000;
    }
    /// CCMR1CAP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr1cap {
        /// CC1S[1:0]
        pub const cc1s_POS: u32 = 0;
        pub const cc1s_MASK: u32 = 0x3;
        /// IC1PSC[3:2]
        pub const ic1psc_POS: u32 = 2;
        pub const ic1psc_MASK: u32 = 0xc;
        /// IC1F[7:4]
        pub const ic1f_POS: u32 = 4;
        pub const ic1f_MASK: u32 = 0xf0;
        /// CC2S[9:8]
        pub const cc2s_POS: u32 = 8;
        pub const cc2s_MASK: u32 = 0x300;
        /// IC2PSC[11:10]
        pub const ic2psc_POS: u32 = 10;
        pub const ic2psc_MASK: u32 = 0xc00;
        /// IC2F[15:12]
        pub const ic2f_POS: u32 = 12;
        pub const ic2f_MASK: u32 = 0xf000;
    }
    /// CCMR1CMP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr1cmp {
        /// CC1S[1:0]
        pub const cc1s_POS: u32 = 0;
        pub const cc1s_MASK: u32 = 0x3;
        /// OC1FE[2:2]
        pub const oc1fe_POS: u32 = 2;
        pub const oc1fe_MASK: u32 = 0x4;
        /// OC1PE[3:3]
        pub const oc1pe_POS: u32 = 3;
        pub const oc1pe_MASK: u32 = 0x8;
        /// OC1M[6:4]
        pub const oc1m_POS: u32 = 4;
        pub const oc1m_MASK: u32 = 0x70;
        /// OC1CE[7:7]
        pub const oc1ce_POS: u32 = 7;
        pub const oc1ce_MASK: u32 = 0x80;
        /// CC2S[9:8]
        pub const cc2s_POS: u32 = 8;
        pub const cc2s_MASK: u32 = 0x300;
        /// OC2FE[10:10]
        pub const oc2fe_POS: u32 = 10;
        pub const oc2fe_MASK: u32 = 0x400;
        /// OC2PE[11:11]
        pub const oc2pe_POS: u32 = 11;
        pub const oc2pe_MASK: u32 = 0x800;
        /// OC2M[14:12]
        pub const oc2m_POS: u32 = 12;
        pub const oc2m_MASK: u32 = 0x7000;
        /// OC2CE[15:15]
        pub const oc2ce_POS: u32 = 15;
        pub const oc2ce_MASK: u32 = 0x8000;
        /// OC1MH[16:16]
        pub const oc1mh_POS: u32 = 16;
        pub const oc1mh_MASK: u32 = 0x10000;
        /// OC2MH[24:24]
        pub const oc2mh_POS: u32 = 24;
        pub const oc2mh_MASK: u32 = 0x1000000;
    }
    /// CCMR2CAP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr2cap {
        /// CC3S[1:0]
        pub const cc3s_POS: u32 = 0;
        pub const cc3s_MASK: u32 = 0x3;
        /// IC3PSC[3:2]
        pub const ic3psc_POS: u32 = 2;
        pub const ic3psc_MASK: u32 = 0xc;
        /// IC3F[7:4]
        pub const ic3f_POS: u32 = 4;
        pub const ic3f_MASK: u32 = 0xf0;
        /// CC4S[9:8]
        pub const cc4s_POS: u32 = 8;
        pub const cc4s_MASK: u32 = 0x300;
        /// IC4PSC[11:10]
        pub const ic4psc_POS: u32 = 10;
        pub const ic4psc_MASK: u32 = 0xc00;
        /// IC4F[15:12]
        pub const ic4f_POS: u32 = 12;
        pub const ic4f_MASK: u32 = 0xf000;
    }
    /// CCMR2CMP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr2cmp {
        /// CC3S[1:0]
        pub const cc3s_POS: u32 = 0;
        pub const cc3s_MASK: u32 = 0x3;
        /// OC3FE[2:2]
        pub const oc3fe_POS: u32 = 2;
        pub const oc3fe_MASK: u32 = 0x4;
        /// OC3PE[3:3]
        pub const oc3pe_POS: u32 = 3;
        pub const oc3pe_MASK: u32 = 0x8;
        /// OC3M[6:4]
        pub const oc3m_POS: u32 = 4;
        pub const oc3m_MASK: u32 = 0x70;
        /// OC3CE[7:7]
        pub const oc3ce_POS: u32 = 7;
        pub const oc3ce_MASK: u32 = 0x80;
        /// CC4S[9:8]
        pub const cc4s_POS: u32 = 8;
        pub const cc4s_MASK: u32 = 0x300;
        /// OC4FE[10:10]
        pub const oc4fe_POS: u32 = 10;
        pub const oc4fe_MASK: u32 = 0x400;
        /// OC4PE[11:11]
        pub const oc4pe_POS: u32 = 11;
        pub const oc4pe_MASK: u32 = 0x800;
        /// OC4M[14:12]
        pub const oc4m_POS: u32 = 12;
        pub const oc4m_MASK: u32 = 0x7000;
        /// OC4CE[15:15]
        pub const oc4ce_POS: u32 = 15;
        pub const oc4ce_MASK: u32 = 0x8000;
        /// OC3MH[16:16]
        pub const oc3mh_POS: u32 = 16;
        pub const oc3mh_MASK: u32 = 0x10000;
        /// OC4MH[24:24]
        pub const oc4mh_POS: u32 = 24;
        pub const oc4mh_MASK: u32 = 0x1000000;
    }
    /// CCER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccer {
        /// CC1E[0:0]
        pub const cc1e_POS: u32 = 0;
        pub const cc1e_MASK: u32 = 0x1;
        /// CC1P[1:1]
        pub const cc1p_POS: u32 = 1;
        pub const cc1p_MASK: u32 = 0x2;
        /// CC1NE[2:2]
        pub const cc1ne_POS: u32 = 2;
        pub const cc1ne_MASK: u32 = 0x4;
        /// CC1NP[3:3]
        pub const cc1np_POS: u32 = 3;
        pub const cc1np_MASK: u32 = 0x8;
        /// CC2E[4:4]
        pub const cc2e_POS: u32 = 4;
        pub const cc2e_MASK: u32 = 0x10;
        /// CC2P[5:5]
        pub const cc2p_POS: u32 = 5;
        pub const cc2p_MASK: u32 = 0x20;
        /// CC2NE[6:6]
        pub const cc2ne_POS: u32 = 6;
        pub const cc2ne_MASK: u32 = 0x40;
        /// CC2NP[7:7]
        pub const cc2np_POS: u32 = 7;
        pub const cc2np_MASK: u32 = 0x80;
        /// CC3E[8:8]
        pub const cc3e_POS: u32 = 8;
        pub const cc3e_MASK: u32 = 0x100;
        /// CC3P[9:9]
        pub const cc3p_POS: u32 = 9;
        pub const cc3p_MASK: u32 = 0x200;
        /// CC3NE[10:10]
        pub const cc3ne_POS: u32 = 10;
        pub const cc3ne_MASK: u32 = 0x400;
        /// CC3NP[11:11]
        pub const cc3np_POS: u32 = 11;
        pub const cc3np_MASK: u32 = 0x800;
        /// CC4E[12:12]
        pub const cc4e_POS: u32 = 12;
        pub const cc4e_MASK: u32 = 0x1000;
        /// CC4P[13:13]
        pub const cc4p_POS: u32 = 13;
        pub const cc4p_MASK: u32 = 0x2000;
        /// CC4NE[14:14]
        pub const cc4ne_POS: u32 = 14;
        pub const cc4ne_MASK: u32 = 0x4000;
        /// CC4NP[15:15]
        pub const cc4np_POS: u32 = 15;
        pub const cc4np_MASK: u32 = 0x8000;
        /// CC5E[16:16]
        pub const cc5e_POS: u32 = 16;
        pub const cc5e_MASK: u32 = 0x10000;
        /// CC5P[17:17]
        pub const cc5p_POS: u32 = 17;
        pub const cc5p_MASK: u32 = 0x20000;
        /// CC5NE[18:18]
        pub const cc5ne_POS: u32 = 18;
        pub const cc5ne_MASK: u32 = 0x40000;
        /// CC5NP[19:19]
        pub const cc5np_POS: u32 = 19;
        pub const cc5np_MASK: u32 = 0x80000;
        /// CC6E[20:20]
        pub const cc6e_POS: u32 = 20;
        pub const cc6e_MASK: u32 = 0x100000;
        /// CC6P[21:21]
        pub const cc6p_POS: u32 = 21;
        pub const cc6p_MASK: u32 = 0x200000;
        /// CC6NE[22:22]
        pub const cc6ne_POS: u32 = 22;
        pub const cc6ne_MASK: u32 = 0x400000;
        /// CC6NP[23:23]
        pub const cc6np_POS: u32 = 23;
        pub const cc6np_MASK: u32 = 0x800000;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// UIFCPY[31:31]
        pub const uifcpy_POS: u32 = 31;
        pub const uifcpy_MASK: u32 = 0x80000000;
    }
    /// PSC 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod psc {
        /// PSC[15:0]
        pub const psc_POS: u32 = 0;
        pub const psc_MASK: u32 = 0xffff;
    }
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// RCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod rcr {
        /// REP[15:0]
        pub const rep_POS: u32 = 0;
        pub const rep_MASK: u32 = 0xffff;
    }
    /// CCR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr1 {
        /// CCR1[15:0]
        pub const ccr1_POS: u32 = 0;
        pub const ccr1_MASK: u32 = 0xffff;
    }
    /// CCR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr2 {
        /// CCR2[15:0]
        pub const ccr2_POS: u32 = 0;
        pub const ccr2_MASK: u32 = 0xffff;
    }
    /// CCR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr3 {
        /// CCR3[15:0]
        pub const ccr3_POS: u32 = 0;
        pub const ccr3_MASK: u32 = 0xffff;
    }
    /// CCR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr4 {
        /// CCR4[15:0]
        pub const ccr4_POS: u32 = 0;
        pub const ccr4_MASK: u32 = 0xffff;
    }
    /// BDTR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod bdtr {
        /// DTG[7:0]
        pub const dtg_POS: u32 = 0;
        pub const dtg_MASK: u32 = 0xff;
        /// LOCK[9:8]
        pub const lock_POS: u32 = 8;
        pub const lock_MASK: u32 = 0x300;
        /// OSSI[10:10]
        pub const ossi_POS: u32 = 10;
        pub const ossi_MASK: u32 = 0x400;
        /// OSSR[11:11]
        pub const ossr_POS: u32 = 11;
        pub const ossr_MASK: u32 = 0x800;
        /// BKE[12:12]
        pub const bke_POS: u32 = 12;
        pub const bke_MASK: u32 = 0x1000;
        /// BKP[13:13]
        pub const bkp_POS: u32 = 13;
        pub const bkp_MASK: u32 = 0x2000;
        /// AOE[14:14]
        pub const aoe_POS: u32 = 14;
        pub const aoe_MASK: u32 = 0x4000;
        /// MOE[15:15]
        pub const moe_POS: u32 = 15;
        pub const moe_MASK: u32 = 0x8000;
        /// BKF[19:16]
        pub const bkf_POS: u32 = 16;
        pub const bkf_MASK: u32 = 0xf0000;
        /// BK2F[23:20]
        pub const bk2f_POS: u32 = 20;
        pub const bk2f_MASK: u32 = 0xf00000;
        /// BK2E[24:24]
        pub const bk2e_POS: u32 = 24;
        pub const bk2e_MASK: u32 = 0x1000000;
        /// BK2P[25:25]
        pub const bk2p_POS: u32 = 25;
        pub const bk2p_MASK: u32 = 0x2000000;
    }
    /// CCR5 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr5 {
        /// CCR5[15:0]
        pub const ccr5_POS: u32 = 0;
        pub const ccr5_MASK: u32 = 0xffff;
        /// GC5C1[26:26]
        pub const gc5c1_POS: u32 = 26;
        pub const gc5c1_MASK: u32 = 0x4000000;
        /// GC5C2[27:27]
        pub const gc5c2_POS: u32 = 27;
        pub const gc5c2_MASK: u32 = 0x8000000;
        /// GC5C3[28:28]
        pub const gc5c3_POS: u32 = 28;
        pub const gc5c3_MASK: u32 = 0x10000000;
        /// GC5C4[29:29]
        pub const gc5c4_POS: u32 = 29;
        pub const gc5c4_MASK: u32 = 0x20000000;
        /// GC5C5[30:30]
        pub const gc5c5_POS: u32 = 30;
        pub const gc5c5_MASK: u32 = 0x40000000;
        /// GC5C6[31:31]
        pub const gc5c6_POS: u32 = 31;
        pub const gc5c6_MASK: u32 = 0x80000000;
    }
    /// CCR6 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr6 {
        /// CCR6[15:0]
        pub const ccr6_POS: u32 = 0;
        pub const ccr6_MASK: u32 = 0xffff;
        /// GC6C1[26:26]
        pub const gc6c1_POS: u32 = 26;
        pub const gc6c1_MASK: u32 = 0x4000000;
        /// GC6C2[27:27]
        pub const gc6c2_POS: u32 = 27;
        pub const gc6c2_MASK: u32 = 0x8000000;
        /// GC6C3[28:28]
        pub const gc6c3_POS: u32 = 28;
        pub const gc6c3_MASK: u32 = 0x10000000;
        /// GC6C4[29:29]
        pub const gc6c4_POS: u32 = 29;
        pub const gc6c4_MASK: u32 = 0x20000000;
        /// GC6C5[30:30]
        pub const gc6c5_POS: u32 = 30;
        pub const gc6c5_MASK: u32 = 0x40000000;
        /// GC6C6[31:31]
        pub const gc6c6_POS: u32 = 31;
        pub const gc6c6_MASK: u32 = 0x80000000;
    }
    /// CCMR3CAP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr3cap {
        /// CC5S[1:0]
        pub const cc5s_POS: u32 = 0;
        pub const cc5s_MASK: u32 = 0x3;
        /// IC5PSC[3:2]
        pub const ic5psc_POS: u32 = 2;
        pub const ic5psc_MASK: u32 = 0xc;
        /// IC5F[7:4]
        pub const ic5f_POS: u32 = 4;
        pub const ic5f_MASK: u32 = 0xf0;
        /// CC6S[9:8]
        pub const cc6s_POS: u32 = 8;
        pub const cc6s_MASK: u32 = 0x300;
        /// IC6PSC[11:10]
        pub const ic6psc_POS: u32 = 10;
        pub const ic6psc_MASK: u32 = 0xc00;
        /// IC6F[15:12]
        pub const ic6f_POS: u32 = 12;
        pub const ic6f_MASK: u32 = 0xf000;
    }
    /// CCMR3CMP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr3cmp {
        /// CC5S[1:0]
        pub const cc5s_POS: u32 = 0;
        pub const cc5s_MASK: u32 = 0x3;
        /// OC5FE[2:2]
        pub const oc5fe_POS: u32 = 2;
        pub const oc5fe_MASK: u32 = 0x4;
        /// OC5PE[3:3]
        pub const oc5pe_POS: u32 = 3;
        pub const oc5pe_MASK: u32 = 0x8;
        /// OC5M[6:4]
        pub const oc5m_POS: u32 = 4;
        pub const oc5m_MASK: u32 = 0x70;
        /// OC5CE[7:7]
        pub const oc5ce_POS: u32 = 7;
        pub const oc5ce_MASK: u32 = 0x80;
        /// CC6S[9:8]
        pub const cc6s_POS: u32 = 8;
        pub const cc6s_MASK: u32 = 0x300;
        /// OC6FE[10:10]
        pub const oc6fe_POS: u32 = 10;
        pub const oc6fe_MASK: u32 = 0x400;
        /// OC6PE[11:11]
        pub const oc6pe_POS: u32 = 11;
        pub const oc6pe_MASK: u32 = 0x800;
        /// OC6M[14:12]
        pub const oc6m_POS: u32 = 12;
        pub const oc6m_MASK: u32 = 0x7000;
        /// OC6CE[15:15]
        pub const oc6ce_POS: u32 = 15;
        pub const oc6ce_MASK: u32 = 0x8000;
        /// OC5MH[16:16]
        pub const oc5mh_POS: u32 = 16;
        pub const oc5mh_MASK: u32 = 0x10000;
        /// OC6MH[24:24]
        pub const oc6mh_POS: u32 = 24;
        pub const oc6mh_MASK: u32 = 0x1000000;
    }
    /// DTR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dtr2 {
        /// DTGF[7:0]
        pub const dtgf_POS: u32 = 0;
        pub const dtgf_MASK: u32 = 0xff;
        /// DTAE[16:16]
        pub const dtae_POS: u32 = 16;
        pub const dtae_MASK: u32 = 0x10000;
        /// DTPE[17:17]
        pub const dtpe_POS: u32 = 17;
        pub const dtpe_MASK: u32 = 0x20000;
    }
    /// ECR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ecr {
        /// IE[0:0]
        pub const ie_POS: u32 = 0;
        pub const ie_MASK: u32 = 0x1;
        /// IDIR[2:1]
        pub const idir_POS: u32 = 1;
        pub const idir_MASK: u32 = 0x6;
        /// FIDX[5:5]
        pub const fidx_POS: u32 = 5;
        pub const fidx_MASK: u32 = 0x20;
        /// IPOS[7:6]
        pub const ipos_POS: u32 = 6;
        pub const ipos_MASK: u32 = 0xc0;
    }
    /// TISEL1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tisel1 {
        /// TI1SEL[3:0]
        pub const ti1sel_POS: u32 = 0;
        pub const ti1sel_MASK: u32 = 0xf;
        /// TI2SEL[11:8]
        pub const ti2sel_POS: u32 = 8;
        pub const ti2sel_MASK: u32 = 0xf00;
        /// TI3SEL[19:16]
        pub const ti3sel_POS: u32 = 16;
        pub const ti3sel_MASK: u32 = 0xf0000;
        /// TI4SEL[27:24]
        pub const ti4sel_POS: u32 = 24;
        pub const ti4sel_MASK: u32 = 0xf000000;
    }
    /// AF1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod af1 {
        /// BKINE[0:0]
        pub const bkine_POS: u32 = 0;
        pub const bkine_MASK: u32 = 0x1;
        /// BKVC1E[1:1]
        pub const bkvc1e_POS: u32 = 1;
        pub const bkvc1e_MASK: u32 = 0x2;
        /// BKVC2E[2:2]
        pub const bkvc2e_POS: u32 = 2;
        pub const bkvc2e_MASK: u32 = 0x4;
        /// BKINP[9:9]
        pub const bkinp_POS: u32 = 9;
        pub const bkinp_MASK: u32 = 0x200;
        /// BKVC1P[10:10]
        pub const bkvc1p_POS: u32 = 10;
        pub const bkvc1p_MASK: u32 = 0x400;
        /// BKVC2P[11:11]
        pub const bkvc2p_POS: u32 = 11;
        pub const bkvc2p_MASK: u32 = 0x800;
        /// ETRSEL[17:14]
        pub const etrsel_POS: u32 = 14;
        pub const etrsel_MASK: u32 = 0x3c000;
    }
    /// AF2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod af2 {
        /// BK2INE[0:0]
        pub const bk2ine_POS: u32 = 0;
        pub const bk2ine_MASK: u32 = 0x1;
        /// BK2VC1E[1:1]
        pub const bk2vc1e_POS: u32 = 1;
        pub const bk2vc1e_MASK: u32 = 0x2;
        /// BK2VC2E[2:2]
        pub const bk2vc2e_POS: u32 = 2;
        pub const bk2vc2e_MASK: u32 = 0x4;
        /// BK2INP[9:9]
        pub const bk2inp_POS: u32 = 9;
        pub const bk2inp_MASK: u32 = 0x200;
        /// BK2VC1P[10:10]
        pub const bk2vc1p_POS: u32 = 10;
        pub const bk2vc1p_MASK: u32 = 0x400;
        /// BK2VC2P[11:11]
        pub const bk2vc2p_POS: u32 = 11;
        pub const bk2vc2p_MASK: u32 = 0x800;
        /// OCRSEL[18:16]
        pub const ocrsel_POS: u32 = 16;
        pub const ocrsel_MASK: u32 = 0x70000;
    }
    /// TISEL2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tisel2 {
        /// TI5SEL[3:0]
        pub const ti5sel_POS: u32 = 0;
        pub const ti5sel_MASK: u32 = 0xf;
        /// TI6SEL[11:8]
        pub const ti6sel_POS: u32 = 8;
        pub const ti6sel_MASK: u32 = 0xf00;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// CC1IF[1:1]
        pub const cc1if_POS: u32 = 1;
        pub const cc1if_MASK: u32 = 0x2;
        /// CC2IF[2:2]
        pub const cc2if_POS: u32 = 2;
        pub const cc2if_MASK: u32 = 0x4;
        /// CC3IF[3:3]
        pub const cc3if_POS: u32 = 3;
        pub const cc3if_MASK: u32 = 0x8;
        /// CC4IF[4:4]
        pub const cc4if_POS: u32 = 4;
        pub const cc4if_MASK: u32 = 0x10;
        /// COMIF[5:5]
        pub const comif_POS: u32 = 5;
        pub const comif_MASK: u32 = 0x20;
        /// TIF[6:6]
        pub const tif_POS: u32 = 6;
        pub const tif_MASK: u32 = 0x40;
        /// BIF[7:7]
        pub const bif_POS: u32 = 7;
        pub const bif_MASK: u32 = 0x80;
        /// B2IF[8:8]
        pub const b2if_POS: u32 = 8;
        pub const b2if_MASK: u32 = 0x100;
        /// CC1OF[9:9]
        pub const cc1of_POS: u32 = 9;
        pub const cc1of_MASK: u32 = 0x200;
        /// CC2OF[10:10]
        pub const cc2of_POS: u32 = 10;
        pub const cc2of_MASK: u32 = 0x400;
        /// CC3OF[11:11]
        pub const cc3of_POS: u32 = 11;
        pub const cc3of_MASK: u32 = 0x800;
        /// CC4OF[12:12]
        pub const cc4of_POS: u32 = 12;
        pub const cc4of_MASK: u32 = 0x1000;
        /// SBIF[13:13]
        pub const sbif_POS: u32 = 13;
        pub const sbif_MASK: u32 = 0x2000;
        /// CC5IF[16:16]
        pub const cc5if_POS: u32 = 16;
        pub const cc5if_MASK: u32 = 0x10000;
        /// CC6IF[17:17]
        pub const cc6if_POS: u32 = 17;
        pub const cc6if_MASK: u32 = 0x20000;
        /// CC5OF[18:18]
        pub const cc5of_POS: u32 = 18;
        pub const cc5of_MASK: u32 = 0x40000;
        /// CC6OF[19:19]
        pub const cc6of_POS: u32 = 19;
        pub const cc6of_MASK: u32 = 0x80000;
        /// IDXF[20:20]
        pub const idxf_POS: u32 = 20;
        pub const idxf_MASK: u32 = 0x100000;
        /// DIRF[21:21]
        pub const dirf_POS: u32 = 21;
        pub const dirf_MASK: u32 = 0x200000;
        /// IERRF[22:22]
        pub const ierrf_POS: u32 = 22;
        pub const ierrf_MASK: u32 = 0x400000;
        /// TERRF[23:23]
        pub const terrf_POS: u32 = 23;
        pub const terrf_MASK: u32 = 0x800000;
    }
}
/// `BTIM` 位段常量
pub mod btim {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// UDIS[1:1]
        pub const udis_POS: u32 = 1;
        pub const udis_MASK: u32 = 0x2;
        /// URS[2:2]
        pub const urs_POS: u32 = 2;
        pub const urs_MASK: u32 = 0x4;
        /// ONESHOT[3:3]
        pub const oneshot_POS: u32 = 3;
        pub const oneshot_MASK: u32 = 0x8;
        /// UIFREMAP[11:11]
        pub const uifremap_POS: u32 = 11;
        pub const uifremap_MASK: u32 = 0x800;
        /// TOGEN[15:15]
        pub const togen_POS: u32 = 15;
        pub const togen_MASK: u32 = 0x8000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// MMS[6:4]
        pub const mms_POS: u32 = 4;
        pub const mms_MASK: u32 = 0x70;
    }
    /// SMCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod smcr {
        /// SMS[2:0]
        pub const sms_POS: u32 = 0;
        pub const sms_MASK: u32 = 0x7;
        /// RSTISRC[6:3]
        pub const rstisrc_POS: u32 = 3;
        pub const rstisrc_MASK: u32 = 0x78;
        /// TRGISRC[10:7]
        pub const trgisrc_POS: u32 = 7;
        pub const trgisrc_MASK: u32 = 0x780;
        /// MSM[11:11]
        pub const msm_POS: u32 = 11;
        pub const msm_MASK: u32 = 0x800;
        /// TRGIFLT[14:12]
        pub const trgiflt_POS: u32 = 12;
        pub const trgiflt_MASK: u32 = 0x7000;
        /// RSTIPOL[16:16]
        pub const rstipol_POS: u32 = 16;
        pub const rstipol_MASK: u32 = 0x10000;
        /// TRGIPOL[17:17]
        pub const trgipol_POS: u32 = 17;
        pub const trgipol_MASK: u32 = 0x20000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// UIE[0:0]
        pub const uie_POS: u32 = 0;
        pub const uie_MASK: u32 = 0x1;
        /// TIE[6:6]
        pub const tie_POS: u32 = 6;
        pub const tie_MASK: u32 = 0x40;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// TIF[6:6]
        pub const tif_POS: u32 = 6;
        pub const tif_MASK: u32 = 0x40;
    }
    /// EGR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod egr {
        /// UG[0:0]
        pub const ug_POS: u32 = 0;
        pub const ug_MASK: u32 = 0x1;
        /// TG[6:6]
        pub const tg_POS: u32 = 6;
        pub const tg_MASK: u32 = 0x40;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// TIF[6:6]
        pub const tif_POS: u32 = 6;
        pub const tif_MASK: u32 = 0x40;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// UIFCPY[31:31]
        pub const uifcpy_POS: u32 = 31;
        pub const uifcpy_MASK: u32 = 0x80000000;
    }
    /// PSC 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod psc {
        /// PSC[15:0]
        pub const psc_POS: u32 = 0;
        pub const psc_MASK: u32 = 0xffff;
    }
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
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
        /// RESULT[15:0]
        pub const result_POS: u32 = 0;
        pub const result_MASK: u32 = 0xffff;
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
        /// SECURITY[6:5]
        pub const security_POS: u32 = 5;
        pub const security_MASK: u32 = 0x60;
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
        /// LOCK10[10:10]
        pub const lock10_POS: u32 = 10;
        pub const lock10_MASK: u32 = 0x400;
        /// LOCK11[11:11]
        pub const lock11_POS: u32 = 11;
        pub const lock11_MASK: u32 = 0x800;
        /// LOCK12[12:12]
        pub const lock12_POS: u32 = 12;
        pub const lock12_MASK: u32 = 0x1000;
        /// LOCK13[13:13]
        pub const lock13_POS: u32 = 13;
        pub const lock13_MASK: u32 = 0x2000;
        /// LOCK14[14:14]
        pub const lock14_POS: u32 = 14;
        pub const lock14_MASK: u32 = 0x4000;
        /// LOCK15[15:15]
        pub const lock15_POS: u32 = 15;
        pub const lock15_MASK: u32 = 0x8000;
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
        /// BUSY[5:5]
        pub const busy_POS: u32 = 5;
        pub const busy_MASK: u32 = 0x20;
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
    /// SDKCFR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sdkcfr {
        /// START[6:0]
        pub const start_POS: u32 = 0;
        pub const start_MASK: u32 = 0x7f;
        /// END[14:8]
        pub const end_POS: u32 = 8;
        pub const end_MASK: u32 = 0x7f00;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
    }
    /// AFRH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod afrh {
        /// AFR8[2:0]
        pub const afr8_POS: u32 = 0;
        pub const afr8_MASK: u32 = 0x7;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
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
        /// BRR8[8:8]
        pub const brr8_POS: u32 = 8;
        pub const brr8_MASK: u32 = 0x100;
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
        /// BSS8[8:8]
        pub const bss8_POS: u32 = 8;
        pub const bss8_MASK: u32 = 0x100;
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
        /// BRR8[24:24]
        pub const brr8_POS: u32 = 24;
        pub const brr8_MASK: u32 = 0x1000000;
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
        /// PIN8[8:8]
        pub const pin8_POS: u32 = 8;
        pub const pin8_MASK: u32 = 0x100;
    }
}
/// `GTIM` 位段常量
pub mod gtim {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// CEN[0:0]
        pub const cen_POS: u32 = 0;
        pub const cen_MASK: u32 = 0x1;
        /// UDIS[1:1]
        pub const udis_POS: u32 = 1;
        pub const udis_MASK: u32 = 0x2;
        /// URS[2:2]
        pub const urs_POS: u32 = 2;
        pub const urs_MASK: u32 = 0x4;
        /// OPM[3:3]
        pub const opm_POS: u32 = 3;
        pub const opm_MASK: u32 = 0x8;
        /// DIR[4:4]
        pub const dir_POS: u32 = 4;
        pub const dir_MASK: u32 = 0x10;
        /// CMS[6:5]
        pub const cms_POS: u32 = 5;
        pub const cms_MASK: u32 = 0x60;
        /// ARPE[7:7]
        pub const arpe_POS: u32 = 7;
        pub const arpe_MASK: u32 = 0x80;
        /// CKD[9:8]
        pub const ckd_POS: u32 = 8;
        pub const ckd_MASK: u32 = 0x300;
        /// UIFREMAP[11:11]
        pub const uifremap_POS: u32 = 11;
        pub const uifremap_MASK: u32 = 0x800;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// MMS[6:4]
        pub const mms_POS: u32 = 4;
        pub const mms_MASK: u32 = 0x70;
        /// TI1S[7:7]
        pub const ti1s_POS: u32 = 7;
        pub const ti1s_MASK: u32 = 0x80;
        /// MMSH[26:25]
        pub const mmsh_POS: u32 = 25;
        pub const mmsh_MASK: u32 = 0x6000000;
    }
    /// SMCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod smcr {
        /// SMS[2:0]
        pub const sms_POS: u32 = 0;
        pub const sms_MASK: u32 = 0x7;
        /// OCCS[3:3]
        pub const occs_POS: u32 = 3;
        pub const occs_MASK: u32 = 0x8;
        /// TS[6:4]
        pub const ts_POS: u32 = 4;
        pub const ts_MASK: u32 = 0x70;
        /// MSM[7:7]
        pub const msm_POS: u32 = 7;
        pub const msm_MASK: u32 = 0x80;
        /// ETF[11:8]
        pub const etf_POS: u32 = 8;
        pub const etf_MASK: u32 = 0xf00;
        /// ETPS[13:12]
        pub const etps_POS: u32 = 12;
        pub const etps_MASK: u32 = 0x3000;
        /// ECE[14:14]
        pub const ece_POS: u32 = 14;
        pub const ece_MASK: u32 = 0x4000;
        /// ETP[15:15]
        pub const etp_POS: u32 = 15;
        pub const etp_MASK: u32 = 0x8000;
        /// SMSH[16:16]
        pub const smsh_POS: u32 = 16;
        pub const smsh_MASK: u32 = 0x10000;
        /// TSH[21:20]
        pub const tsh_POS: u32 = 20;
        pub const tsh_MASK: u32 = 0x300000;
        /// SMSPE[24:24]
        pub const smspe_POS: u32 = 24;
        pub const smspe_MASK: u32 = 0x1000000;
        /// SMSPS[25:25]
        pub const smsps_POS: u32 = 25;
        pub const smsps_MASK: u32 = 0x2000000;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// UIE[0:0]
        pub const uie_POS: u32 = 0;
        pub const uie_MASK: u32 = 0x1;
        /// CC1IE[1:1]
        pub const cc1ie_POS: u32 = 1;
        pub const cc1ie_MASK: u32 = 0x2;
        /// CC2IE[2:2]
        pub const cc2ie_POS: u32 = 2;
        pub const cc2ie_MASK: u32 = 0x4;
        /// CC3IE[3:3]
        pub const cc3ie_POS: u32 = 3;
        pub const cc3ie_MASK: u32 = 0x8;
        /// CC4IE[4:4]
        pub const cc4ie_POS: u32 = 4;
        pub const cc4ie_MASK: u32 = 0x10;
        /// TIE[6:6]
        pub const tie_POS: u32 = 6;
        pub const tie_MASK: u32 = 0x40;
        /// IDXIE[20:20]
        pub const idxie_POS: u32 = 20;
        pub const idxie_MASK: u32 = 0x100000;
        /// DIRIE[21:21]
        pub const dirie_POS: u32 = 21;
        pub const dirie_MASK: u32 = 0x200000;
        /// IERRIE[22:22]
        pub const ierrie_POS: u32 = 22;
        pub const ierrie_MASK: u32 = 0x400000;
        /// TERRIE[23:23]
        pub const terrie_POS: u32 = 23;
        pub const terrie_MASK: u32 = 0x800000;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// CC1IF[1:1]
        pub const cc1if_POS: u32 = 1;
        pub const cc1if_MASK: u32 = 0x2;
        /// CC2IF[2:2]
        pub const cc2if_POS: u32 = 2;
        pub const cc2if_MASK: u32 = 0x4;
        /// CC3IF[3:3]
        pub const cc3if_POS: u32 = 3;
        pub const cc3if_MASK: u32 = 0x8;
        /// CC4IF[4:4]
        pub const cc4if_POS: u32 = 4;
        pub const cc4if_MASK: u32 = 0x10;
        /// TIF[6:6]
        pub const tif_POS: u32 = 6;
        pub const tif_MASK: u32 = 0x40;
        /// CC1OF[9:9]
        pub const cc1of_POS: u32 = 9;
        pub const cc1of_MASK: u32 = 0x200;
        /// CC2OF[10:10]
        pub const cc2of_POS: u32 = 10;
        pub const cc2of_MASK: u32 = 0x400;
        /// CC3OF[11:11]
        pub const cc3of_POS: u32 = 11;
        pub const cc3of_MASK: u32 = 0x800;
        /// CC4OF[12:12]
        pub const cc4of_POS: u32 = 12;
        pub const cc4of_MASK: u32 = 0x1000;
        /// IDXF[20:20]
        pub const idxf_POS: u32 = 20;
        pub const idxf_MASK: u32 = 0x100000;
        /// DIRF[21:21]
        pub const dirf_POS: u32 = 21;
        pub const dirf_MASK: u32 = 0x200000;
        /// IERRF[22:22]
        pub const ierrf_POS: u32 = 22;
        pub const ierrf_MASK: u32 = 0x400000;
        /// TERRF[23:23]
        pub const terrf_POS: u32 = 23;
        pub const terrf_MASK: u32 = 0x800000;
    }
    /// EGR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod egr {
        /// UG[0:0]
        pub const ug_POS: u32 = 0;
        pub const ug_MASK: u32 = 0x1;
        /// CC1G[1:1]
        pub const cc1g_POS: u32 = 1;
        pub const cc1g_MASK: u32 = 0x2;
        /// CC2G[2:2]
        pub const cc2g_POS: u32 = 2;
        pub const cc2g_MASK: u32 = 0x4;
        /// CC3G[3:3]
        pub const cc3g_POS: u32 = 3;
        pub const cc3g_MASK: u32 = 0x8;
        /// CC4G[4:4]
        pub const cc4g_POS: u32 = 4;
        pub const cc4g_MASK: u32 = 0x10;
        /// TG[6:6]
        pub const tg_POS: u32 = 6;
        pub const tg_MASK: u32 = 0x40;
    }
    /// CCMR1CAP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr1cap {
        /// CC1S[1:0]
        pub const cc1s_POS: u32 = 0;
        pub const cc1s_MASK: u32 = 0x3;
        /// IC1PSC[3:2]
        pub const ic1psc_POS: u32 = 2;
        pub const ic1psc_MASK: u32 = 0xc;
        /// IC1F[7:4]
        pub const ic1f_POS: u32 = 4;
        pub const ic1f_MASK: u32 = 0xf0;
        /// CC2S[9:8]
        pub const cc2s_POS: u32 = 8;
        pub const cc2s_MASK: u32 = 0x300;
        /// IC2PSC[11:10]
        pub const ic2psc_POS: u32 = 10;
        pub const ic2psc_MASK: u32 = 0xc00;
        /// IC2F[15:12]
        pub const ic2f_POS: u32 = 12;
        pub const ic2f_MASK: u32 = 0xf000;
    }
    /// CCMR1CMP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr1cmp {
        /// CC1S[1:0]
        pub const cc1s_POS: u32 = 0;
        pub const cc1s_MASK: u32 = 0x3;
        /// OC1FE[2:2]
        pub const oc1fe_POS: u32 = 2;
        pub const oc1fe_MASK: u32 = 0x4;
        /// OC1PE[3:3]
        pub const oc1pe_POS: u32 = 3;
        pub const oc1pe_MASK: u32 = 0x8;
        /// OC1M[6:4]
        pub const oc1m_POS: u32 = 4;
        pub const oc1m_MASK: u32 = 0x70;
        /// OC1CE[7:7]
        pub const oc1ce_POS: u32 = 7;
        pub const oc1ce_MASK: u32 = 0x80;
        /// CC2S[9:8]
        pub const cc2s_POS: u32 = 8;
        pub const cc2s_MASK: u32 = 0x300;
        /// OC2FE[10:10]
        pub const oc2fe_POS: u32 = 10;
        pub const oc2fe_MASK: u32 = 0x400;
        /// OC2PE[11:11]
        pub const oc2pe_POS: u32 = 11;
        pub const oc2pe_MASK: u32 = 0x800;
        /// OC2M[14:12]
        pub const oc2m_POS: u32 = 12;
        pub const oc2m_MASK: u32 = 0x7000;
        /// OC2CE[15:15]
        pub const oc2ce_POS: u32 = 15;
        pub const oc2ce_MASK: u32 = 0x8000;
        /// OC1MH[16:16]
        pub const oc1mh_POS: u32 = 16;
        pub const oc1mh_MASK: u32 = 0x10000;
        /// OC2MH[24:24]
        pub const oc2mh_POS: u32 = 24;
        pub const oc2mh_MASK: u32 = 0x1000000;
    }
    /// CCMR2CAP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr2cap {
        /// CC3S[1:0]
        pub const cc3s_POS: u32 = 0;
        pub const cc3s_MASK: u32 = 0x3;
        /// IC3PSC[3:2]
        pub const ic3psc_POS: u32 = 2;
        pub const ic3psc_MASK: u32 = 0xc;
        /// IC3F[7:4]
        pub const ic3f_POS: u32 = 4;
        pub const ic3f_MASK: u32 = 0xf0;
        /// CC4S[9:8]
        pub const cc4s_POS: u32 = 8;
        pub const cc4s_MASK: u32 = 0x300;
        /// IC4PSC[11:10]
        pub const ic4psc_POS: u32 = 10;
        pub const ic4psc_MASK: u32 = 0xc00;
        /// IC4F[15:12]
        pub const ic4f_POS: u32 = 12;
        pub const ic4f_MASK: u32 = 0xf000;
    }
    /// CCMR2CMP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccmr2cmp {
        /// CC3S[1:0]
        pub const cc3s_POS: u32 = 0;
        pub const cc3s_MASK: u32 = 0x3;
        /// OC3FE[2:2]
        pub const oc3fe_POS: u32 = 2;
        pub const oc3fe_MASK: u32 = 0x4;
        /// OC3PE[3:3]
        pub const oc3pe_POS: u32 = 3;
        pub const oc3pe_MASK: u32 = 0x8;
        /// OC3M[6:4]
        pub const oc3m_POS: u32 = 4;
        pub const oc3m_MASK: u32 = 0x70;
        /// OC3CE[7:7]
        pub const oc3ce_POS: u32 = 7;
        pub const oc3ce_MASK: u32 = 0x80;
        /// CC4S[9:8]
        pub const cc4s_POS: u32 = 8;
        pub const cc4s_MASK: u32 = 0x300;
        /// OC4FE[10:10]
        pub const oc4fe_POS: u32 = 10;
        pub const oc4fe_MASK: u32 = 0x400;
        /// OC4PE[11:11]
        pub const oc4pe_POS: u32 = 11;
        pub const oc4pe_MASK: u32 = 0x800;
        /// OC4M[14:12]
        pub const oc4m_POS: u32 = 12;
        pub const oc4m_MASK: u32 = 0x7000;
        /// OC4CE[15:15]
        pub const oc4ce_POS: u32 = 15;
        pub const oc4ce_MASK: u32 = 0x8000;
        /// OC3MH[16:16]
        pub const oc3mh_POS: u32 = 16;
        pub const oc3mh_MASK: u32 = 0x10000;
        /// OC4MH[24:24]
        pub const oc4mh_POS: u32 = 24;
        pub const oc4mh_MASK: u32 = 0x1000000;
    }
    /// CCER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccer {
        /// CC1E[0:0]
        pub const cc1e_POS: u32 = 0;
        pub const cc1e_MASK: u32 = 0x1;
        /// CC1P[1:1]
        pub const cc1p_POS: u32 = 1;
        pub const cc1p_MASK: u32 = 0x2;
        /// CC1NP[3:3]
        pub const cc1np_POS: u32 = 3;
        pub const cc1np_MASK: u32 = 0x8;
        /// CC2E[4:4]
        pub const cc2e_POS: u32 = 4;
        pub const cc2e_MASK: u32 = 0x10;
        /// CC2P[5:5]
        pub const cc2p_POS: u32 = 5;
        pub const cc2p_MASK: u32 = 0x20;
        /// CC2NP[7:7]
        pub const cc2np_POS: u32 = 7;
        pub const cc2np_MASK: u32 = 0x80;
        /// CC3E[8:8]
        pub const cc3e_POS: u32 = 8;
        pub const cc3e_MASK: u32 = 0x100;
        /// CC3P[9:9]
        pub const cc3p_POS: u32 = 9;
        pub const cc3p_MASK: u32 = 0x200;
        /// CC3NP[11:11]
        pub const cc3np_POS: u32 = 11;
        pub const cc3np_MASK: u32 = 0x800;
        /// CC4E[12:12]
        pub const cc4e_POS: u32 = 12;
        pub const cc4e_MASK: u32 = 0x1000;
        /// CC4P[13:13]
        pub const cc4p_POS: u32 = 13;
        pub const cc4p_MASK: u32 = 0x2000;
        /// CC4NP[15:15]
        pub const cc4np_POS: u32 = 15;
        pub const cc4np_MASK: u32 = 0x8000;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// UIFCPY[31:31]
        pub const uifcpy_POS: u32 = 31;
        pub const uifcpy_MASK: u32 = 0x80000000;
    }
    /// PSC 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod psc {
        /// PSC[15:0]
        pub const psc_POS: u32 = 0;
        pub const psc_MASK: u32 = 0xffff;
    }
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// CCR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr1 {
        /// CCR1[15:0]
        pub const ccr1_POS: u32 = 0;
        pub const ccr1_MASK: u32 = 0xffff;
    }
    /// CCR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr2 {
        /// CCR2[15:0]
        pub const ccr2_POS: u32 = 0;
        pub const ccr2_MASK: u32 = 0xffff;
    }
    /// CCR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr3 {
        /// CCR3[15:0]
        pub const ccr3_POS: u32 = 0;
        pub const ccr3_MASK: u32 = 0xffff;
    }
    /// CCR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr4 {
        /// CCR4[15:0]
        pub const ccr4_POS: u32 = 0;
        pub const ccr4_MASK: u32 = 0xffff;
    }
    /// ECR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ecr {
        /// IE[0:0]
        pub const ie_POS: u32 = 0;
        pub const ie_MASK: u32 = 0x1;
        /// IDIR[2:1]
        pub const idir_POS: u32 = 1;
        pub const idir_MASK: u32 = 0x6;
        /// FIDX[5:5]
        pub const fidx_POS: u32 = 5;
        pub const fidx_MASK: u32 = 0x20;
        /// IPOS[7:6]
        pub const ipos_POS: u32 = 6;
        pub const ipos_MASK: u32 = 0xc0;
    }
    /// TISEL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tisel {
        /// TI1SEL[3:0]
        pub const ti1sel_POS: u32 = 0;
        pub const ti1sel_MASK: u32 = 0xf;
        /// TI2SEL[11:8]
        pub const ti2sel_POS: u32 = 8;
        pub const ti2sel_MASK: u32 = 0xf00;
        /// TI3SEL[19:16]
        pub const ti3sel_POS: u32 = 16;
        pub const ti3sel_MASK: u32 = 0xf0000;
        /// TI4SEL[27:24]
        pub const ti4sel_POS: u32 = 24;
        pub const ti4sel_MASK: u32 = 0xf000000;
    }
    /// AF1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod af1 {
        /// ETRSEL[17:14]
        pub const etrsel_POS: u32 = 14;
        pub const etrsel_MASK: u32 = 0x3c000;
    }
    /// AF2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod af2 {
        /// OCRSEL[18:16]
        pub const ocrsel_POS: u32 = 16;
        pub const ocrsel_MASK: u32 = 0x70000;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// UIF[0:0]
        pub const uif_POS: u32 = 0;
        pub const uif_MASK: u32 = 0x1;
        /// CC1IF[1:1]
        pub const cc1if_POS: u32 = 1;
        pub const cc1if_MASK: u32 = 0x2;
        /// CC2IF[2:2]
        pub const cc2if_POS: u32 = 2;
        pub const cc2if_MASK: u32 = 0x4;
        /// CC3IF[3:3]
        pub const cc3if_POS: u32 = 3;
        pub const cc3if_MASK: u32 = 0x8;
        /// CC4IF[4:4]
        pub const cc4if_POS: u32 = 4;
        pub const cc4if_MASK: u32 = 0x10;
        /// TIF[6:6]
        pub const tif_POS: u32 = 6;
        pub const tif_MASK: u32 = 0x40;
        /// CC1OF[9:9]
        pub const cc1of_POS: u32 = 9;
        pub const cc1of_MASK: u32 = 0x200;
        /// CC2OF[10:10]
        pub const cc2of_POS: u32 = 10;
        pub const cc2of_MASK: u32 = 0x400;
        /// CC3OF[11:11]
        pub const cc3of_POS: u32 = 11;
        pub const cc3of_MASK: u32 = 0x800;
        /// CC4OF[12:12]
        pub const cc4of_POS: u32 = 12;
        pub const cc4of_MASK: u32 = 0x1000;
        /// IDXF[20:20]
        pub const idxf_POS: u32 = 20;
        pub const idxf_MASK: u32 = 0x100000;
        /// DIRF[21:21]
        pub const dirf_POS: u32 = 21;
        pub const dirf_MASK: u32 = 0x200000;
        /// IERRF[22:22]
        pub const ierrf_POS: u32 = 22;
        pub const ierrf_MASK: u32 = 0x400000;
        /// TERRF[23:23]
        pub const terrf_POS: u32 = 23;
        pub const terrf_MASK: u32 = 0x800000;
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
        /// SCLINSRC[10:8]
        pub const sclinsrc_POS: u32 = 8;
        pub const sclinsrc_MASK: u32 = 0x700;
        /// SDAINSRC[13:11]
        pub const sdainsrc_POS: u32 = 11;
        pub const sdainsrc_MASK: u32 = 0x3800;
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
/// `IRMOD` 位段常量
pub mod irmod {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// MOD[3:0]
        pub const mod_POS: u32 = 0;
        pub const mod_MASK: u32 = 0xf;
        /// IRSW[4:4]
        pub const irsw_POS: u32 = 4;
        pub const irsw_MASK: u32 = 0x10;
        /// INV[5:5]
        pub const inv_POS: u32 = 5;
        pub const inv_MASK: u32 = 0x20;
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
/// `LPTIM` 位段常量
pub mod lptim {
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// CMPM[0:0]
        pub const cmpm_POS: u32 = 0;
        pub const cmpm_MASK: u32 = 0x1;
        /// ARRM[1:1]
        pub const arrm_POS: u32 = 1;
        pub const arrm_MASK: u32 = 0x2;
        /// EXTTRIG[2:2]
        pub const exttrig_POS: u32 = 2;
        pub const exttrig_MASK: u32 = 0x4;
        /// CMPOK[3:3]
        pub const cmpok_POS: u32 = 3;
        pub const cmpok_MASK: u32 = 0x8;
        /// ARROK[4:4]
        pub const arrok_POS: u32 = 4;
        pub const arrok_MASK: u32 = 0x10;
        /// UP[5:5]
        pub const up_POS: u32 = 5;
        pub const up_MASK: u32 = 0x20;
        /// DOWN[6:6]
        pub const down_POS: u32 = 6;
        pub const down_MASK: u32 = 0x40;
        /// DIR[8:7]
        pub const dir_POS: u32 = 7;
        pub const dir_MASK: u32 = 0x180;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// CMPM[0:0]
        pub const cmpm_POS: u32 = 0;
        pub const cmpm_MASK: u32 = 0x1;
        /// ARRM[1:1]
        pub const arrm_POS: u32 = 1;
        pub const arrm_MASK: u32 = 0x2;
        /// EXTTRIG[2:2]
        pub const exttrig_POS: u32 = 2;
        pub const exttrig_MASK: u32 = 0x4;
        /// CMPOK[3:3]
        pub const cmpok_POS: u32 = 3;
        pub const cmpok_MASK: u32 = 0x8;
        /// ARROK[4:4]
        pub const arrok_POS: u32 = 4;
        pub const arrok_MASK: u32 = 0x10;
        /// UP[5:5]
        pub const up_POS: u32 = 5;
        pub const up_MASK: u32 = 0x20;
        /// DOWN[6:6]
        pub const down_POS: u32 = 6;
        pub const down_MASK: u32 = 0x40;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// CMPM[0:0]
        pub const cmpm_POS: u32 = 0;
        pub const cmpm_MASK: u32 = 0x1;
        /// ARRM[1:1]
        pub const arrm_POS: u32 = 1;
        pub const arrm_MASK: u32 = 0x2;
        /// EXTTRIG[2:2]
        pub const exttrig_POS: u32 = 2;
        pub const exttrig_MASK: u32 = 0x4;
        /// CMPOK[3:3]
        pub const cmpok_POS: u32 = 3;
        pub const cmpok_MASK: u32 = 0x8;
        /// ARROK[4:4]
        pub const arrok_POS: u32 = 4;
        pub const arrok_MASK: u32 = 0x10;
        /// UP[5:5]
        pub const up_POS: u32 = 5;
        pub const up_MASK: u32 = 0x20;
        /// DOWN[6:6]
        pub const down_POS: u32 = 6;
        pub const down_MASK: u32 = 0x40;
    }
    /// CFGR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cfgr {
        /// CKSEL[0:0]
        pub const cksel_POS: u32 = 0;
        pub const cksel_MASK: u32 = 0x1;
        /// ENCMD_CKPOL[2:1]
        pub const encmd_ckpol_POS: u32 = 1;
        pub const encmd_ckpol_MASK: u32 = 0x6;
        /// CHFLT[4:3]
        pub const chflt_POS: u32 = 3;
        pub const chflt_MASK: u32 = 0x18;
        /// TRIGFLT[7:6]
        pub const trigflt_POS: u32 = 6;
        pub const trigflt_MASK: u32 = 0xc0;
        /// PRS[11:9]
        pub const prs_POS: u32 = 9;
        pub const prs_MASK: u32 = 0xe00;
        /// TRIGSEL[15:13]
        pub const trigsel_POS: u32 = 13;
        pub const trigsel_MASK: u32 = 0xe000;
        /// TRIGEN[18:17]
        pub const trigen_POS: u32 = 17;
        pub const trigen_MASK: u32 = 0x60000;
        /// TIMOUT[19:19]
        pub const timout_POS: u32 = 19;
        pub const timout_MASK: u32 = 0x80000;
        /// WAVE[20:20]
        pub const wave_POS: u32 = 20;
        pub const wave_MASK: u32 = 0x100000;
        /// WAVPOL[21:21]
        pub const wavpol_POS: u32 = 21;
        pub const wavpol_MASK: u32 = 0x200000;
        /// PRELOAD[22:22]
        pub const preload_POS: u32 = 22;
        pub const preload_MASK: u32 = 0x400000;
        /// COUNTMD[23:23]
        pub const countmd_POS: u32 = 23;
        pub const countmd_MASK: u32 = 0x800000;
        /// ENC[24:24]
        pub const enc_POS: u32 = 24;
        pub const enc_MASK: u32 = 0x1000000;
        /// ICLKSRC[26:25]
        pub const iclksrc_POS: u32 = 25;
        pub const iclksrc_MASK: u32 = 0x6000000;
    }
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// SNGSTART[1:1]
        pub const sngstart_POS: u32 = 1;
        pub const sngstart_MASK: u32 = 0x2;
        /// CNTSTART[2:2]
        pub const cntstart_POS: u32 = 2;
        pub const cntstart_MASK: u32 = 0x4;
        /// SRST[3:3]
        pub const srst_POS: u32 = 3;
        pub const srst_MASK: u32 = 0x8;
        /// ARST[4:4]
        pub const arst_POS: u32 = 4;
        pub const arst_MASK: u32 = 0x10;
    }
    /// CMP 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cmp {
        /// CMP[15:0]
        pub const cmp_POS: u32 = 0;
        pub const cmp_MASK: u32 = 0xffff;
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
}
/// `RAM` 位段常量
pub mod ram {
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// PARITY[0:0]
        pub const parity_POS: u32 = 0;
        pub const parity_MASK: u32 = 0x1;
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
        /// PARITY[3:3]
        pub const parity_POS: u32 = 3;
        pub const parity_MASK: u32 = 0x8;
        /// PARITYEN[2:2]
        pub const parityen_POS: u32 = 2;
        pub const parityen_MASK: u32 = 0x4;
        /// STOP[5:4]
        pub const stop_POS: u32 = 4;
        pub const stop_MASK: u32 = 0x30;
        /// CHLEN[6:6]
        pub const chlen_POS: u32 = 6;
        pub const chlen_MASK: u32 = 0x40;
        /// MSBF[7:7]
        pub const msbf_POS: u32 = 7;
        pub const msbf_MASK: u32 = 0x80;
        /// START[8:8]
        pub const start_POS: u32 = 8;
        pub const start_MASK: u32 = 0x100;
        /// OVER[10:9]
        pub const over_POS: u32 = 9;
        pub const over_MASK: u32 = 0x600;
        /// SIGNAL[11:11]
        pub const signal_POS: u32 = 11;
        pub const signal_MASK: u32 = 0x800;
        /// SOURCE[13:12]
        pub const source_POS: u32 = 12;
        pub const source_MASK: u32 = 0x3000;
        /// SYNC[14:14]
        pub const sync_POS: u32 = 14;
        pub const sync_MASK: u32 = 0x4000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// ADDREN[0:0]
        pub const addren_POS: u32 = 0;
        pub const addren_MASK: u32 = 0x1;
        /// RXMATCHEN[1:1]
        pub const rxmatchen_POS: u32 = 1;
        pub const rxmatchen_MASK: u32 = 0x2;
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
        /// TIMCR[10:8]
        pub const timcr_POS: u32 = 8;
        pub const timcr_MASK: u32 = 0x700;
        /// SWAP[11:11]
        pub const swap_POS: u32 = 11;
        pub const swap_MASK: u32 = 0x800;
        /// ADCRX[12:12]
        pub const adcrx_POS: u32 = 12;
        pub const adcrx_MASK: u32 = 0x1000;
        /// ADCTX[13:13]
        pub const adctx_POS: u32 = 13;
        pub const adctx_MASK: u32 = 0x2000;
        /// LOOP[14:14]
        pub const r#loop_POS: u32 = 14;
        pub const r#loop_MASK: u32 = 0x4000;
        /// RXSRC[17:15]
        pub const rxsrc_POS: u32 = 15;
        pub const rxsrc_MASK: u32 = 0x38000;
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
        /// RXIDLE[3:3]
        pub const rxidle_POS: u32 = 3;
        pub const rxidle_MASK: u32 = 0x8;
        /// RXBRK[4:4]
        pub const rxbrk_POS: u32 = 4;
        pub const rxbrk_MASK: u32 = 0x10;
        /// BAUD[5:5]
        pub const baud_POS: u32 = 5;
        pub const baud_MASK: u32 = 0x20;
        /// TIMOV[6:6]
        pub const timov_POS: u32 = 6;
        pub const timov_MASK: u32 = 0x40;
        /// CTS[7:7]
        pub const cts_POS: u32 = 7;
        pub const cts_MASK: u32 = 0x80;
        /// FE[8:8]
        pub const fe_POS: u32 = 8;
        pub const fe_MASK: u32 = 0x100;
        /// PE[9:9]
        pub const pe_POS: u32 = 9;
        pub const pe_MASK: u32 = 0x200;
        /// NE[10:10]
        pub const ne_POS: u32 = 10;
        pub const ne_MASK: u32 = 0x400;
        /// ORE[11:11]
        pub const ore_POS: u32 = 11;
        pub const ore_MASK: u32 = 0x800;
        /// RXMATCH[12:12]
        pub const rxmatch_POS: u32 = 12;
        pub const rxmatch_MASK: u32 = 0x1000;
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
    /// TIMARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod timarr {
        /// TIMARR[23:0]
        pub const timarr_POS: u32 = 0;
        pub const timarr_MASK: u32 = 0xffffff;
    }
    /// TIMCNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod timcnt {
        /// TIMCNT[23:0]
        pub const timcnt_POS: u32 = 0;
        pub const timcnt_MASK: u32 = 0xffffff;
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
        /// RXIDLE[3:3]
        pub const rxidle_POS: u32 = 3;
        pub const rxidle_MASK: u32 = 0x8;
        /// RXBRK[4:4]
        pub const rxbrk_POS: u32 = 4;
        pub const rxbrk_MASK: u32 = 0x10;
        /// BAUD[5:5]
        pub const baud_POS: u32 = 5;
        pub const baud_MASK: u32 = 0x20;
        /// TIMOV[6:6]
        pub const timov_POS: u32 = 6;
        pub const timov_MASK: u32 = 0x40;
        /// CTS[7:7]
        pub const cts_POS: u32 = 7;
        pub const cts_MASK: u32 = 0x80;
        /// FE[8:8]
        pub const fe_POS: u32 = 8;
        pub const fe_MASK: u32 = 0x100;
        /// PE[9:9]
        pub const pe_POS: u32 = 9;
        pub const pe_MASK: u32 = 0x200;
        /// NE[10:10]
        pub const ne_POS: u32 = 10;
        pub const ne_MASK: u32 = 0x400;
        /// ORE[11:11]
        pub const ore_POS: u32 = 11;
        pub const ore_MASK: u32 = 0x800;
        /// RXMATCH[12:12]
        pub const rxmatch_POS: u32 = 12;
        pub const rxmatch_MASK: u32 = 0x1000;
        /// SLVMATCH[13:13]
        pub const slvmatch_POS: u32 = 13;
        pub const slvmatch_MASK: u32 = 0x2000;
        /// TXBUSY[14:14]
        pub const txbusy_POS: u32 = 14;
        pub const txbusy_MASK: u32 = 0x4000;
        /// CTSLV[15:15]
        pub const ctslv_POS: u32 = 15;
        pub const ctslv_MASK: u32 = 0x8000;
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
        /// RXIDLE[3:3]
        pub const rxidle_POS: u32 = 3;
        pub const rxidle_MASK: u32 = 0x8;
        /// RXBRK[4:4]
        pub const rxbrk_POS: u32 = 4;
        pub const rxbrk_MASK: u32 = 0x10;
        /// BAUD[5:5]
        pub const baud_POS: u32 = 5;
        pub const baud_MASK: u32 = 0x20;
        /// TIMOV[6:6]
        pub const timov_POS: u32 = 6;
        pub const timov_MASK: u32 = 0x40;
        /// CTS[7:7]
        pub const cts_POS: u32 = 7;
        pub const cts_MASK: u32 = 0x80;
        /// FE[8:8]
        pub const fe_POS: u32 = 8;
        pub const fe_MASK: u32 = 0x100;
        /// PE[9:9]
        pub const pe_POS: u32 = 9;
        pub const pe_MASK: u32 = 0x200;
        /// NE[10:10]
        pub const ne_POS: u32 = 10;
        pub const ne_MASK: u32 = 0x400;
        /// ORE[11:11]
        pub const ore_POS: u32 = 11;
        pub const ore_MASK: u32 = 0x800;
        /// RXMATCH[12:12]
        pub const rxmatch_POS: u32 = 12;
        pub const rxmatch_MASK: u32 = 0x1000;
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
        /// IDLE[9:9]
        pub const idle_POS: u32 = 9;
        pub const idle_MASK: u32 = 0x200;
        /// BREAK[10:10]
        pub const r#break_POS: u32 = 10;
        pub const r#break_MASK: u32 = 0x400;
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
    /// CR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr3 {
        /// DEM[0:0]
        pub const dem_POS: u32 = 0;
        pub const dem_MASK: u32 = 0x1;
        /// DEP[1:1]
        pub const dep_POS: u32 = 1;
        pub const dep_MASK: u32 = 0x2;
        /// DETIME[6:2]
        pub const detime_POS: u32 = 2;
        pub const detime_MASK: u32 = 0x7c;
        /// LIN[7:7]
        pub const lin_POS: u32 = 7;
        pub const lin_MASK: u32 = 0x80;
        /// BRKL[8:8]
        pub const brkl_POS: u32 = 8;
        pub const brkl_MASK: u32 = 0x100;
    }
    /// RXMATCH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod rxmatch {
        /// RXMATCH[8:0]
        pub const rxmatch_POS: u32 = 0;
        pub const rxmatch_MASK: u32 = 0x1ff;
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
        /// SOURCE[2:2]
        pub const source_POS: u32 = 2;
        pub const source_MASK: u32 = 0x4;
        /// VTH[6:4]
        pub const vth_POS: u32 = 4;
        pub const vth_MASK: u32 = 0x70;
        /// FLTCLK[8:8]
        pub const fltclk_POS: u32 = 8;
        pub const fltclk_MASK: u32 = 0x100;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// IE[0:0]
        pub const ie_POS: u32 = 0;
        pub const ie_MASK: u32 = 0x1;
        /// LEVEL[1:1]
        pub const level_POS: u32 = 1;
        pub const level_MASK: u32 = 0x2;
        /// FALL[2:2]
        pub const fall_POS: u32 = 2;
        pub const fall_MASK: u32 = 0x4;
        /// RISE[3:3]
        pub const rise_POS: u32 = 3;
        pub const rise_MASK: u32 = 0x8;
        /// FLTTIME[7:4]
        pub const flttime_POS: u32 = 4;
        pub const flttime_MASK: u32 = 0xf0;
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
/// `RTC` 位段常量
pub mod rtc {
    /// KEY 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod key {
        /// KEY[7:0]
        pub const key_POS: u32 = 0;
        pub const key_MASK: u32 = 0xff;
    }
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// INTERVAL[2:0]
        pub const interval_POS: u32 = 0;
        pub const interval_MASK: u32 = 0x7;
        /// H24[3:3]
        pub const h24_POS: u32 = 3;
        pub const h24_MASK: u32 = 0x8;
        /// RTC1HZ[6:5]
        pub const rtc1hz_POS: u32 = 5;
        pub const rtc1hz_MASK: u32 = 0x60;
        /// START[7:7]
        pub const start_POS: u32 = 7;
        pub const start_MASK: u32 = 0x80;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// ACCESS[0:0]
        pub const access_POS: u32 = 0;
        pub const access_MASK: u32 = 0x1;
        /// WINDOW[1:1]
        pub const window_POS: u32 = 1;
        pub const window_MASK: u32 = 0x2;
        /// WAIT[2:2]
        pub const wait_POS: u32 = 2;
        pub const wait_MASK: u32 = 0x4;
        /// SOURCE[10:8]
        pub const source_POS: u32 = 8;
        pub const source_MASK: u32 = 0x700;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// AWTPRS[1:0]
        pub const awtprs_POS: u32 = 0;
        pub const awtprs_MASK: u32 = 0x3;
        /// AWTSRC[2:2]
        pub const awtsrc_POS: u32 = 2;
        pub const awtsrc_MASK: u32 = 0x4;
        /// TAMPEDGE[3:3]
        pub const tampedge_POS: u32 = 3;
        pub const tampedge_MASK: u32 = 0x8;
        /// RTCOUT[5:4]
        pub const rtcout_POS: u32 = 4;
        pub const rtcout_MASK: u32 = 0x30;
        /// TAMPEN[6:6]
        pub const tampen_POS: u32 = 6;
        pub const tampen_MASK: u32 = 0x40;
        /// AWTEN[7:7]
        pub const awten_POS: u32 = 7;
        pub const awten_MASK: u32 = 0x80;
        /// ALARMAEN[9:9]
        pub const alarmaen_POS: u32 = 9;
        pub const alarmaen_MASK: u32 = 0x200;
        /// ALARMBEN[10:10]
        pub const alarmben_POS: u32 = 10;
        pub const alarmben_MASK: u32 = 0x400;
    }
    /// COMPCFR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod compcfr1 {
        /// COMP[11:0]
        pub const comp_POS: u32 = 0;
        pub const comp_MASK: u32 = 0xfff;
        /// PERIOD[13:12]
        pub const period_POS: u32 = 12;
        pub const period_MASK: u32 = 0x3000;
        /// SIGN[14:14]
        pub const sign_POS: u32 = 14;
        pub const sign_MASK: u32 = 0x4000;
        /// EN[15:15]
        pub const en_POS: u32 = 15;
        pub const en_MASK: u32 = 0x8000;
    }
    /// DATE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod date {
        /// DAY[7:0]
        pub const day_POS: u32 = 0;
        pub const day_MASK: u32 = 0xff;
        /// MONTH[15:8]
        pub const month_POS: u32 = 8;
        pub const month_MASK: u32 = 0xff00;
        /// YEAR[23:16]
        pub const year_POS: u32 = 16;
        pub const year_MASK: u32 = 0xff0000;
        /// WEEK[26:24]
        pub const week_POS: u32 = 24;
        pub const week_MASK: u32 = 0x7000000;
    }
    /// TIME 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod time {
        /// SECOND[6:0]
        pub const second_POS: u32 = 0;
        pub const second_MASK: u32 = 0x7f;
        /// MINUTE[14:8]
        pub const minute_POS: u32 = 8;
        pub const minute_MASK: u32 = 0x7f00;
        /// HOUR[21:16]
        pub const hour_POS: u32 = 16;
        pub const hour_MASK: u32 = 0x3f0000;
    }
    /// ALARMA 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod alarma {
        /// SECOND[6:0]
        pub const second_POS: u32 = 0;
        pub const second_MASK: u32 = 0x7f;
        /// SECONDEN[7:7]
        pub const seconden_POS: u32 = 7;
        pub const seconden_MASK: u32 = 0x80;
        /// MINUTE[14:8]
        pub const minute_POS: u32 = 8;
        pub const minute_MASK: u32 = 0x7f00;
        /// MINUTEEN[15:15]
        pub const minuteen_POS: u32 = 15;
        pub const minuteen_MASK: u32 = 0x8000;
        /// HOUR[21:16]
        pub const hour_POS: u32 = 16;
        pub const hour_MASK: u32 = 0x3f0000;
        /// HOUREN[23:23]
        pub const houren_POS: u32 = 23;
        pub const houren_MASK: u32 = 0x800000;
        /// WEEKMASK[30:24]
        pub const weekmask_POS: u32 = 24;
        pub const weekmask_MASK: u32 = 0x7f000000;
    }
    /// ALARMB 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod alarmb {
        /// SECOND[6:0]
        pub const second_POS: u32 = 0;
        pub const second_MASK: u32 = 0x7f;
        /// SECONDEN[7:7]
        pub const seconden_POS: u32 = 7;
        pub const seconden_MASK: u32 = 0x80;
        /// MINUTE[14:8]
        pub const minute_POS: u32 = 8;
        pub const minute_MASK: u32 = 0x7f00;
        /// MINUTEEN[15:15]
        pub const minuteen_POS: u32 = 15;
        pub const minuteen_MASK: u32 = 0x8000;
        /// HOUR[21:16]
        pub const hour_POS: u32 = 16;
        pub const hour_MASK: u32 = 0x3f0000;
        /// HOUREN[23:23]
        pub const houren_POS: u32 = 23;
        pub const houren_MASK: u32 = 0x800000;
        /// WEEKMASK[30:24]
        pub const weekmask_POS: u32 = 24;
        pub const weekmask_MASK: u32 = 0x7f000000;
    }
    /// TAMPDATE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tampdate {
        /// DAY[5:0]
        pub const day_POS: u32 = 0;
        pub const day_MASK: u32 = 0x3f;
        /// MONTH[12:8]
        pub const month_POS: u32 = 8;
        pub const month_MASK: u32 = 0x1f00;
        /// WEEK[15:13]
        pub const week_POS: u32 = 13;
        pub const week_MASK: u32 = 0xe000;
    }
    /// TAMPTIME 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod tamptime {
        /// SECOND[6:0]
        pub const second_POS: u32 = 0;
        pub const second_MASK: u32 = 0x7f;
        /// MINUTE[14:8]
        pub const minute_POS: u32 = 8;
        pub const minute_MASK: u32 = 0x7f00;
        /// HOUR[21:16]
        pub const hour_POS: u32 = 16;
        pub const hour_MASK: u32 = 0x3f0000;
    }
    /// AWTARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod awtarr {
        /// ARR[15:0]
        pub const arr_POS: u32 = 0;
        pub const arr_MASK: u32 = 0xffff;
    }
    /// IER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ier {
        /// ALARMA[0:0]
        pub const alarma_POS: u32 = 0;
        pub const alarma_MASK: u32 = 0x1;
        /// ALARMB[1:1]
        pub const alarmb_POS: u32 = 1;
        pub const alarmb_MASK: u32 = 0x2;
        /// AWTIMER[2:2]
        pub const awtimer_POS: u32 = 2;
        pub const awtimer_MASK: u32 = 0x4;
        /// TAMP[3:3]
        pub const tamp_POS: u32 = 3;
        pub const tamp_MASK: u32 = 0x8;
        /// TAMPOV[4:4]
        pub const tampov_POS: u32 = 4;
        pub const tampov_MASK: u32 = 0x10;
        /// INTERVAL[6:6]
        pub const interval_POS: u32 = 6;
        pub const interval_MASK: u32 = 0x40;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// ALARMA[0:0]
        pub const alarma_POS: u32 = 0;
        pub const alarma_MASK: u32 = 0x1;
        /// ALARMB[1:1]
        pub const alarmb_POS: u32 = 1;
        pub const alarmb_MASK: u32 = 0x2;
        /// AWTIMER[2:2]
        pub const awtimer_POS: u32 = 2;
        pub const awtimer_MASK: u32 = 0x4;
        /// TAMP[3:3]
        pub const tamp_POS: u32 = 3;
        pub const tamp_MASK: u32 = 0x8;
        /// TAMPOV[4:4]
        pub const tampov_POS: u32 = 4;
        pub const tampov_MASK: u32 = 0x10;
        /// INTERVAL[6:6]
        pub const interval_POS: u32 = 6;
        pub const interval_MASK: u32 = 0x40;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// ALARMA[0:0]
        pub const alarma_POS: u32 = 0;
        pub const alarma_MASK: u32 = 0x1;
        /// ALARMB[1:1]
        pub const alarmb_POS: u32 = 1;
        pub const alarmb_MASK: u32 = 0x2;
        /// AWTIMER[2:2]
        pub const awtimer_POS: u32 = 2;
        pub const awtimer_MASK: u32 = 0x4;
        /// TAMP[3:3]
        pub const tamp_POS: u32 = 3;
        pub const tamp_MASK: u32 = 0x8;
        /// TAMPOV[4:4]
        pub const tampov_POS: u32 = 4;
        pub const tampov_MASK: u32 = 0x10;
        /// INTERVAL[6:6]
        pub const interval_POS: u32 = 6;
        pub const interval_MASK: u32 = 0x40;
    }
    /// AWTCNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod awtcnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
    }
    /// PSC 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod psc {
        /// PSC2[19:0]
        pub const psc2_POS: u32 = 0;
        pub const psc2_MASK: u32 = 0xfffff;
        /// PSC1[27:20]
        pub const psc1_POS: u32 = 20;
        pub const psc1_MASK: u32 = 0xff00000;
    }
    /// SSCNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sscnt {
        /// SSCNT0[19:0]
        pub const sscnt0_POS: u32 = 0;
        pub const sscnt0_MASK: u32 = 0xfffff;
        /// SSCNT1[20:20]
        pub const sscnt1_POS: u32 = 20;
        pub const sscnt1_MASK: u32 = 0x100000;
    }
    /// COMPCFR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod compcfr2 {
        /// PCLKCNT[10:0]
        pub const pclkcnt_POS: u32 = 0;
        pub const pclkcnt_MASK: u32 = 0x7ff;
    }
    /// COMPCFR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod compcfr3 {
        /// PCLKCNT[10:0]
        pub const pclkcnt_POS: u32 = 0;
        pub const pclkcnt_MASK: u32 = 0x7ff;
    }
}
/// `SPI` 位段常量
pub mod spi {
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// MSTR[0:0]
        pub const mstr_POS: u32 = 0;
        pub const mstr_MASK: u32 = 0x1;
        /// SSM[1:1]
        pub const ssm_POS: u32 = 1;
        pub const ssm_MASK: u32 = 0x2;
        /// CPHA[2:2]
        pub const cpha_POS: u32 = 2;
        pub const cpha_MASK: u32 = 0x4;
        /// CPOL[3:3]
        pub const cpol_POS: u32 = 3;
        pub const cpol_MASK: u32 = 0x8;
        /// BR[6:4]
        pub const br_POS: u32 = 4;
        pub const br_MASK: u32 = 0x70;
        /// LSBF[7:7]
        pub const lsbf_POS: u32 = 7;
        pub const lsbf_MASK: u32 = 0x80;
        /// WIDTH[11:8]
        pub const width_POS: u32 = 8;
        pub const width_MASK: u32 = 0xf00;
        /// GAP[15:12]
        pub const gap_POS: u32 = 12;
        pub const gap_MASK: u32 = 0xf000;
        /// MODE[17:16]
        pub const mode_POS: u32 = 16;
        pub const mode_MASK: u32 = 0x30000;
        /// FLTEN[18:18]
        pub const flten_POS: u32 = 18;
        pub const flten_MASK: u32 = 0x40000;
        /// MISOHD[19:19]
        pub const misohd_POS: u32 = 19;
        pub const misohd_MASK: u32 = 0x80000;
        /// SMP[20:20]
        pub const smp_POS: u32 = 20;
        pub const smp_MASK: u32 = 0x100000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// ADCRX[3:3]
        pub const adcrx_POS: u32 = 3;
        pub const adcrx_MASK: u32 = 0x8;
        /// ADCTX[4:4]
        pub const adctx_POS: u32 = 4;
        pub const adctx_MASK: u32 = 0x10;
    }
    /// CR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr3 {
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
        /// HSEEN[1:1]
        pub const hseen_POS: u32 = 1;
        pub const hseen_MASK: u32 = 0x2;
        /// LSIEN[3:3]
        pub const lsien_POS: u32 = 3;
        pub const lsien_MASK: u32 = 0x8;
        /// LSEEN[4:4]
        pub const lseen_POS: u32 = 4;
        pub const lseen_MASK: u32 = 0x10;
        /// LSELOCK[5:5]
        pub const lselock_POS: u32 = 5;
        pub const lselock_MASK: u32 = 0x20;
        /// LSECCS[6:6]
        pub const lseccs_POS: u32 = 6;
        pub const lseccs_MASK: u32 = 0x40;
        /// HSECCS[7:7]
        pub const hseccs_POS: u32 = 7;
        pub const hseccs_MASK: u32 = 0x80;
        /// CLKCCS[8:8]
        pub const clkccs_POS: u32 = 8;
        pub const clkccs_MASK: u32 = 0x100;
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
        /// FLASHWAIT[6:4]
        pub const flashwait_POS: u32 = 4;
        pub const flashwait_MASK: u32 = 0x70;
        /// RTCLPM[7:7]
        pub const rtclpm_POS: u32 = 7;
        pub const rtclpm_MASK: u32 = 0x80;
        /// HSEBKEN[10:10]
        pub const hsebken_POS: u32 = 10;
        pub const hsebken_MASK: u32 = 0x400;
        /// LSEBKEN[11:11]
        pub const lsebken_POS: u32 = 11;
        pub const lsebken_MASK: u32 = 0x800;
        /// CLLBKEN[12:12]
        pub const cllbken_POS: u32 = 12;
        pub const cllbken_MASK: u32 = 0x1000;
        /// DSBKEN[13:13]
        pub const dsbken_POS: u32 = 13;
        pub const dsbken_MASK: u32 = 0x2000;
        /// LVDBKEN[14:14]
        pub const lvdbken_POS: u32 = 14;
        pub const lvdbken_MASK: u32 = 0x4000;
        /// RAMBKEN[15:15]
        pub const rambken_POS: u32 = 15;
        pub const rambken_MASK: u32 = 0x8000;
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
        /// HSERDY[1:1]
        pub const hserdy_POS: u32 = 1;
        pub const hserdy_MASK: u32 = 0x2;
        /// LSIRDY[3:3]
        pub const lsirdy_POS: u32 = 3;
        pub const lsirdy_MASK: u32 = 0x8;
        /// LSERDY[4:4]
        pub const lserdy_POS: u32 = 4;
        pub const lserdy_MASK: u32 = 0x10;
        /// LSEFAIL[5:5]
        pub const lsefail_POS: u32 = 5;
        pub const lsefail_MASK: u32 = 0x20;
        /// HSEFAIL[6:6]
        pub const hsefail_POS: u32 = 6;
        pub const hsefail_MASK: u32 = 0x40;
        /// LSEFAULT[7:7]
        pub const lsefault_POS: u32 = 7;
        pub const lsefault_MASK: u32 = 0x80;
        /// HSEFAULT[8:8]
        pub const hsefault_POS: u32 = 8;
        pub const hsefault_MASK: u32 = 0x100;
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
        /// HSERDY[1:1]
        pub const hserdy_POS: u32 = 1;
        pub const hserdy_MASK: u32 = 0x2;
        /// LSIRDY[3:3]
        pub const lsirdy_POS: u32 = 3;
        pub const lsirdy_MASK: u32 = 0x8;
        /// LSERDY[4:4]
        pub const lserdy_POS: u32 = 4;
        pub const lserdy_MASK: u32 = 0x10;
        /// LSEFAIL[5:5]
        pub const lsefail_POS: u32 = 5;
        pub const lsefail_MASK: u32 = 0x20;
        /// HSEFAIL[6:6]
        pub const hsefail_POS: u32 = 6;
        pub const hsefail_MASK: u32 = 0x40;
        /// LSEFAULT[7:7]
        pub const lsefault_POS: u32 = 7;
        pub const lsefault_MASK: u32 = 0x80;
        /// HSEFAULT[8:8]
        pub const hsefault_POS: u32 = 8;
        pub const hsefault_MASK: u32 = 0x100;
        /// HSISTABLE[11:11]
        pub const hsistable_POS: u32 = 11;
        pub const hsistable_MASK: u32 = 0x800;
        /// HSESTABLE[12:12]
        pub const hsestable_POS: u32 = 12;
        pub const hsestable_MASK: u32 = 0x1000;
        /// LSISTABLE[14:14]
        pub const lsistable_POS: u32 = 14;
        pub const lsistable_MASK: u32 = 0x4000;
        /// LSESTABLE[15:15]
        pub const lsestable_POS: u32 = 15;
        pub const lsestable_MASK: u32 = 0x8000;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// HSIRDY[0:0]
        pub const hsirdy_POS: u32 = 0;
        pub const hsirdy_MASK: u32 = 0x1;
        /// HSERDY[1:1]
        pub const hserdy_POS: u32 = 1;
        pub const hserdy_MASK: u32 = 0x2;
        /// LSIRDY[3:3]
        pub const lsirdy_POS: u32 = 3;
        pub const lsirdy_MASK: u32 = 0x8;
        /// LSERDY[4:4]
        pub const lserdy_POS: u32 = 4;
        pub const lserdy_MASK: u32 = 0x10;
        /// LSEFAIL[5:5]
        pub const lsefail_POS: u32 = 5;
        pub const lsefail_MASK: u32 = 0x20;
        /// HSEFAIL[6:6]
        pub const hsefail_POS: u32 = 6;
        pub const hsefail_MASK: u32 = 0x40;
        /// LSEFAULT[7:7]
        pub const lsefault_POS: u32 = 7;
        pub const lsefault_MASK: u32 = 0x80;
        /// HSEFAULT[8:8]
        pub const hsefault_POS: u32 = 8;
        pub const hsefault_MASK: u32 = 0x100;
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
    /// HSE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod hse {
        /// DRIVER[3:0]
        pub const driver_POS: u32 = 0;
        pub const driver_MASK: u32 = 0xf;
        /// WAITCYCLE[5:4]
        pub const waitcycle_POS: u32 = 4;
        pub const waitcycle_MASK: u32 = 0x30;
        /// MODE[6:6]
        pub const mode_POS: u32 = 6;
        pub const mode_MASK: u32 = 0x40;
        /// HEXENPOL[7:7]
        pub const hexenpol_POS: u32 = 7;
        pub const hexenpol_MASK: u32 = 0x80;
        /// DETCNT[18:8]
        pub const detcnt_POS: u32 = 8;
        pub const detcnt_MASK: u32 = 0x7ff00;
        /// STABLE[19:19]
        pub const stable_POS: u32 = 19;
        pub const stable_MASK: u32 = 0x80000;
        /// PDRIVER[23:20]
        pub const pdriver_POS: u32 = 20;
        pub const pdriver_MASK: u32 = 0xf00000;
        /// DIGFLT[24:24]
        pub const digflt_POS: u32 = 24;
        pub const digflt_MASK: u32 = 0x1000000;
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
    /// LSE 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod lse {
        /// DRIVER[3:0]
        pub const driver_POS: u32 = 0;
        pub const driver_MASK: u32 = 0xf;
        /// WAITCYCLE[5:4]
        pub const waitcycle_POS: u32 = 4;
        pub const waitcycle_MASK: u32 = 0x30;
        /// MODE[6:6]
        pub const mode_POS: u32 = 6;
        pub const mode_MASK: u32 = 0x40;
        /// ANAFLT[7:7]
        pub const anaflt_POS: u32 = 7;
        pub const anaflt_MASK: u32 = 0x80;
        /// PDRIVER[11:8]
        pub const pdriver_POS: u32 = 8;
        pub const pdriver_MASK: u32 = 0xf00;
        /// COMP[12:12]
        pub const comp_POS: u32 = 12;
        pub const comp_MASK: u32 = 0x1000;
        /// CPEN[13:13]
        pub const cpen_POS: u32 = 13;
        pub const cpen_MASK: u32 = 0x2000;
        /// RESTRIM[15:14]
        pub const restrim_POS: u32 = 14;
        pub const restrim_MASK: u32 = 0xc000;
        /// WP[16:16]
        pub const wp_POS: u32 = 16;
        pub const wp_MASK: u32 = 0x10000;
        /// PINLOCK[17:17]
        pub const pinlock_POS: u32 = 17;
        pub const pinlock_MASK: u32 = 0x20000;
        /// STABLE[18:18]
        pub const stable_POS: u32 = 18;
        pub const stable_MASK: u32 = 0x40000;
    }
    /// DEBUG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod debug {
        /// ATIM[0:0]
        pub const atim_POS: u32 = 0;
        pub const atim_MASK: u32 = 0x1;
        /// GTIM1[1:1]
        pub const gtim1_POS: u32 = 1;
        pub const gtim1_MASK: u32 = 0x2;
        /// BTIM123[5:5]
        pub const btim123_POS: u32 = 5;
        pub const btim123_MASK: u32 = 0x20;
        /// LPTIM[6:6]
        pub const lptim_POS: u32 = 6;
        pub const lptim_MASK: u32 = 0x40;
        /// RTC[8:8]
        pub const rtc_POS: u32 = 8;
        pub const rtc_MASK: u32 = 0x100;
        /// IWDT[9:9]
        pub const iwdt_POS: u32 = 9;
        pub const iwdt_MASK: u32 = 0x200;
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
    }
    /// APBEN2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apben2 {
        /// RTC[1:1]
        pub const rtc_POS: u32 = 1;
        pub const rtc_MASK: u32 = 0x2;
        /// BTIM123[2:2]
        pub const btim123_POS: u32 = 2;
        pub const btim123_MASK: u32 = 0x4;
        /// IWDT[4:4]
        pub const iwdt_POS: u32 = 4;
        pub const iwdt_MASK: u32 = 0x10;
        /// I2C1[6:6]
        pub const i2c1_POS: u32 = 6;
        pub const i2c1_MASK: u32 = 0x40;
        /// LPTIM[7:7]
        pub const lptim_POS: u32 = 7;
        pub const lptim_MASK: u32 = 0x80;
    }
    /// APBEN1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apben1 {
        /// ADC[0:0]
        pub const adc_POS: u32 = 0;
        pub const adc_MASK: u32 = 0x1;
        /// VC[1:1]
        pub const vc_POS: u32 = 1;
        pub const vc_MASK: u32 = 0x2;
        /// SPI1[2:2]
        pub const spi1_POS: u32 = 2;
        pub const spi1_MASK: u32 = 0x4;
        /// UART1[3:3]
        pub const uart1_POS: u32 = 3;
        pub const uart1_MASK: u32 = 0x8;
        /// UART2[4:4]
        pub const uart2_POS: u32 = 4;
        pub const uart2_MASK: u32 = 0x10;
        /// ATIM[5:5]
        pub const atim_POS: u32 = 5;
        pub const atim_MASK: u32 = 0x20;
        /// GTIM1[6:6]
        pub const gtim1_POS: u32 = 6;
        pub const gtim1_MASK: u32 = 0x40;
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
    }
    /// APBRST2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apbrst2 {
        /// RTC[1:1]
        pub const rtc_POS: u32 = 1;
        pub const rtc_MASK: u32 = 0x2;
        /// BTIM123[2:2]
        pub const btim123_POS: u32 = 2;
        pub const btim123_MASK: u32 = 0x4;
        /// IWDT[4:4]
        pub const iwdt_POS: u32 = 4;
        pub const iwdt_MASK: u32 = 0x10;
        /// I2C1[6:6]
        pub const i2c1_POS: u32 = 6;
        pub const i2c1_MASK: u32 = 0x40;
        /// LPTIM[7:7]
        pub const lptim_POS: u32 = 7;
        pub const lptim_MASK: u32 = 0x80;
    }
    /// APBRST1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod apbrst1 {
        /// ADC[0:0]
        pub const adc_POS: u32 = 0;
        pub const adc_MASK: u32 = 0x1;
        /// VC[1:1]
        pub const vc_POS: u32 = 1;
        pub const vc_MASK: u32 = 0x2;
        /// SPI1[2:2]
        pub const spi1_POS: u32 = 2;
        pub const spi1_MASK: u32 = 0x4;
        /// UART1[3:3]
        pub const uart1_POS: u32 = 3;
        pub const uart1_MASK: u32 = 0x8;
        /// UART2[4:4]
        pub const uart2_POS: u32 = 4;
        pub const uart2_MASK: u32 = 0x10;
        /// ATIM[5:5]
        pub const atim_POS: u32 = 5;
        pub const atim_MASK: u32 = 0x20;
        /// GTIM1[6:6]
        pub const gtim1_POS: u32 = 6;
        pub const gtim1_MASK: u32 = 0x40;
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
}
/// `VC` 位段常量
pub mod vc {
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// RESP[1:1]
        pub const resp_POS: u32 = 1;
        pub const resp_MASK: u32 = 0x2;
        /// HYS[2:2]
        pub const hys_POS: u32 = 2;
        pub const hys_MASK: u32 = 0x4;
        /// IE[3:3]
        pub const ie_POS: u32 = 3;
        pub const ie_MASK: u32 = 0x8;
        /// POL[4:4]
        pub const pol_POS: u32 = 4;
        pub const pol_MASK: u32 = 0x10;
        /// WINDOW[5:5]
        pub const window_POS: u32 = 5;
        pub const window_MASK: u32 = 0x20;
        /// INP[7:6]
        pub const inp_POS: u32 = 6;
        pub const inp_MASK: u32 = 0xc0;
        /// INN[9:8]
        pub const inn_POS: u32 = 8;
        pub const inn_MASK: u32 = 0x300;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// FLTTIME[3:0]
        pub const flttime_POS: u32 = 0;
        pub const flttime_MASK: u32 = 0xf;
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
        /// BLANKATCH1[8:8]
        pub const blankatch1_POS: u32 = 8;
        pub const blankatch1_MASK: u32 = 0x100;
        /// BLANKATCH2[9:9]
        pub const blankatch2_POS: u32 = 9;
        pub const blankatch2_MASK: u32 = 0x200;
        /// BLANKATCH3[10:10]
        pub const blankatch3_POS: u32 = 10;
        pub const blankatch3_MASK: u32 = 0x400;
        /// BLANKATCH4[11:11]
        pub const blankatch4_POS: u32 = 11;
        pub const blankatch4_MASK: u32 = 0x800;
        /// BLANKATCH5[12:12]
        pub const blankatch5_POS: u32 = 12;
        pub const blankatch5_MASK: u32 = 0x1000;
        /// BLANKATCH6[13:13]
        pub const blankatch6_POS: u32 = 13;
        pub const blankatch6_MASK: u32 = 0x2000;
        /// BLANKTIME[16:14]
        pub const blanktime_POS: u32 = 14;
        pub const blanktime_MASK: u32 = 0x1c000;
        /// BLANKLVL[17:17]
        pub const blanklvl_POS: u32 = 17;
        pub const blanklvl_MASK: u32 = 0x20000;
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
/// `VCREF` 位段常量
pub mod vcref {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// DIV[2:0]
        pub const div_POS: u32 = 0;
        pub const div_MASK: u32 = 0x7;
        /// EN[4:4]
        pub const en_POS: u32 = 4;
        pub const en_MASK: u32 = 0x10;
        /// VIN[5:5]
        pub const vin_POS: u32 = 5;
        pub const vin_MASK: u32 = 0x20;
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
    /// IRQ2:RTC
    Rtc = 2,
    /// IRQ3:FLASHRAM
    Flashram = 3,
    /// IRQ4:SYSCTRL
    Sysctrl = 4,
    /// IRQ5:GPIOA
    Gpioa = 5,
    /// IRQ6:GPIOB
    Gpiob = 6,
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
    /// IRQ12:ADC
    Adc = 12,
    /// IRQ13:ATIM
    Atim = 13,
    /// IRQ14:VC1
    Vc1 = 14,
    /// IRQ15:VC2
    Vc2 = 15,
    /// IRQ16:GTIM1
    Gtim1 = 16,
    /// IRQ17:RESERVED17
    Reserved17 = 17,
    /// IRQ18:RESERVED18
    Reserved18 = 18,
    /// IRQ19:LPTIM
    Lptim = 19,
    /// IRQ20:BTIM1
    Btim1 = 20,
    /// IRQ21:BTIM2
    Btim2 = 21,
    /// IRQ22:BTIM3
    Btim3 = 22,
    /// IRQ23:I2C1
    I2C1 = 23,
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
        /// IRQ2:向量符号 `RTC`
        RTC = 2;
        /// IRQ3:向量符号 `FLASHRAM`
        FLASHRAM = 3;
        /// IRQ4:向量符号 `SYSCTRL`
        SYSCTRL = 4;
        /// IRQ5:向量符号 `GPIOA`
        GPIOA = 5;
        /// IRQ6:向量符号 `GPIOB`
        GPIOB = 6;
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
        /// IRQ12:向量符号 `ADC`
        ADC = 12;
        /// IRQ13:向量符号 `ATIM`
        ATIM = 13;
        /// IRQ14:向量符号 `VC1`
        VC1 = 14;
        /// IRQ15:向量符号 `VC2`
        VC2 = 15;
        /// IRQ16:向量符号 `GTIM1`
        GTIM1 = 16;
        /// IRQ17:向量符号 `RESERVED17`
        RESERVED17 = 17;
        /// IRQ18:向量符号 `RESERVED18`
        RESERVED18 = 18;
        /// IRQ19:向量符号 `LPTIM`
        LPTIM = 19;
        /// IRQ20:向量符号 `BTIM1`
        BTIM1 = 20;
        /// IRQ21:向量符号 `BTIM2`
        BTIM2 = 21;
        /// IRQ22:向量符号 `BTIM3`
        BTIM3 = 22;
        /// IRQ23:向量符号 `I2C1`
        I2C1 = 23;
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
    /// GTIM1
    pub gtim1: &'static GTIM,
    /// I2C1
    pub i2_c1: &'static I2C,
    /// IRMOD
    pub irmod: &'static IRMOD,
    /// IWDT
    pub iwdt: &'static IWDT,
    /// LPTIM
    pub lptim: &'static LPTIM,
    /// RAM
    pub ram: &'static RAM,
    /// UART1
    pub uart1: &'static UART,
    /// UART2
    pub uart2: &'static UART,
    /// LVD
    pub lvd: &'static LVD,
    /// RTC
    pub rtc: &'static RTC,
    /// SPI
    pub spi: &'static SPI,
    /// SYSCTRL
    pub sysctrl: &'static SYSCTRL,
    /// VC1
    pub vc1: &'static VC,
    /// VC2
    pub vc2: &'static VC,
    /// VCREF
    pub vcref: &'static VCREF,
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
            btim1: unsafe { &*(BTIM1_BASE as *const BTIM) },
            btim2: unsafe { &*(BTIM2_BASE as *const BTIM) },
            btim3: unsafe { &*(BTIM3_BASE as *const BTIM) },
            crc: unsafe { &*(CRC_BASE as *const CRC) },
            flash: unsafe { &*(FLASH_BASE as *const FLASH) },
            gpioa: unsafe { &*(GPIOA_BASE as *const GPIO) },
            gpiob: unsafe { &*(GPIOB_BASE as *const GPIO) },
            gtim1: unsafe { &*(GTIM1_BASE as *const GTIM) },
            i2_c1: unsafe { &*(I2C1_BASE as *const I2C) },
            irmod: unsafe { &*(IRMOD_BASE as *const IRMOD) },
            iwdt: unsafe { &*(IWDT_BASE as *const IWDT) },
            lptim: unsafe { &*(LPTIM_BASE as *const LPTIM) },
            ram: unsafe { &*(RAM_BASE as *const RAM) },
            uart1: unsafe { &*(UART1_BASE as *const UART) },
            uart2: unsafe { &*(UART2_BASE as *const UART) },
            lvd: unsafe { &*(LVD_BASE as *const LVD) },
            rtc: unsafe { &*(RTC_BASE as *const RTC) },
            spi: unsafe { &*(SPI_BASE as *const SPI) },
            sysctrl: unsafe { &*(SYSCTRL_BASE as *const SYSCTRL) },
            vc1: unsafe { &*(VC1_BASE as *const VC) },
            vc2: unsafe { &*(VC2_BASE as *const VC) },
            vcref: unsafe { &*(VCREF_BASE as *const VCREF) },
        }
    }
}

// ================= 布局静态断言(厂商头 Size 注释) =================
const _: () = {
    assert!(core::mem::size_of::<ADC>() == 128,
        "ADC 布局错误:厂商头文件 Size = 128");
    assert!(core::mem::size_of::<ATIM>() == 116,
        "ATIM 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<BTIM>() == 48,
        "BTIM 布局错误:厂商头文件 Size = 48");
    assert!(core::mem::size_of::<CRC>() == 20,
        "CRC 布局错误:厂商头文件 Size = 20");
    assert!(core::mem::size_of::<FLASH>() == 116,
        "FLASH 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<GPIO>() == 100,
        "GPIO 布局错误:厂商头文件 Size = 100");
    assert!(core::mem::size_of::<GTIM>() == 116,
        "GTIM 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<I2C>() == 44,
        "I2C 布局错误:厂商头文件 Size = 44");
    assert!(core::mem::size_of::<IRMOD>() == 12,
        "IRMOD 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<IWDT>() == 40,
        "IWDT 布局错误:厂商头文件 Size = 40");
    assert!(core::mem::size_of::<LPTIM>() == 32,
        "LPTIM 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<RAM>() == 16,
        "RAM 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<UART>() == 64,
        "UART 布局错误:厂商头文件 Size = 64");
    assert!(core::mem::size_of::<LVD>() == 12,
        "LVD 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<RTC>() == 92,
        "RTC 布局错误:厂商头文件 Size = 92");
    assert!(core::mem::size_of::<SPI>() == 32,
        "SPI 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<SYSCTRL>() == 116,
        "SYSCTRL 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<VC>() == 12,
        "VC 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<VCREF>() == 4,
        "VCREF 布局错误:厂商头文件 Size = 4");
};