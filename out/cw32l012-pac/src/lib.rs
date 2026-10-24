//! # cw32l012-pac
//!
//! CW32L012 (Cortex-M0+) 的寄存器访问层,由 cw32-pac-gen 从厂商
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

/// ADC1(ADC)基地址
pub const ADC1_BASE: usize = 0x40000000;
/// ATIM(ATIM)基地址
pub const ATIM_BASE: usize = 0x40001400;
/// BGR(BGR)基地址
pub const BGR_BASE: usize = 0x400000fc;
/// BTIM1(BTIM)基地址
pub const BTIM1_BASE: usize = 0x40004800;
/// CORDIC(CORDIC)基地址
pub const CORDIC_BASE: usize = 0x40023400;
/// CRC(CRC)基地址
pub const CRC_BASE: usize = 0x40023000;
/// DAC(DAC)基地址
pub const DAC_BASE: usize = 0x400000c0;
/// DMA(DMA)基地址
pub const DMA_BASE: usize = 0x40020000;
/// DMACHANNEL1(DMACHANNEL)基地址
pub const DMACHANNEL1_BASE: usize = 0x40020020;
/// EAU(EAU)基地址
pub const EAU_BASE: usize = 0x40023200;
/// FLASH(FLASH)基地址
pub const FLASH_BASE: usize = 0x40022000;
/// GPIOA(GPIO)基地址
pub const GPIOA_BASE: usize = 0x48000000;
/// GTIM1(GTIM)基地址
pub const GTIM1_BASE: usize = 0x40001800;
/// HALLTIM(HALLTIM)基地址
pub const HALLTIM_BASE: usize = 0x40006400;
/// I2C1(I2C)基地址
pub const I2C1_BASE: usize = 0x40005800;
/// IRMOD(IRMOD)基地址
pub const IRMOD_BASE: usize = 0x40004080;
/// IWDT(IWDT)基地址
pub const IWDT_BASE: usize = 0x40005000;
/// LPTIM(LPTIM)基地址
pub const LPTIM_BASE: usize = 0x40006000;
/// LVD(LVD)基地址
pub const LVD_BASE: usize = 0x400000a4;
/// OPA1(OPA)基地址
pub const OPA1_BASE: usize = 0x400000b0;
/// RAM(RAM)基地址
pub const RAM_BASE: usize = 0x40022400;
/// RTC(RTC)基地址
pub const RTC_BASE: usize = 0x40004400;
/// SPI1(SPI)基地址
pub const SPI1_BASE: usize = 0x40000800;
/// SYSCTRL(SYSCTRL)基地址
pub const SYSCTRL_BASE: usize = 0x40004000;
/// UART1(UART)基地址
pub const UART1_BASE: usize = 0x40000c00;
/// VC1(VC)基地址
pub const VC1_BASE: usize = 0x40000084;
/// VC12REF(VCREF)基地址
pub const VC12REF_BASE: usize = 0x40000080;

// ================= 寄存器块 =================

/// `ADC` 寄存器组(生成自厂商头文件)
///
/// - `ADC1` 基地址 `0X40000000`
#[allow(dead_code)]
#[repr(C)]
pub struct ADC {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    /// 0x04 START(RW)
    pub start: Rw<u32>,
    /// 0x08 AWDTR(RW)
    pub awdtr: Rw<u32>,
    /// 0x0C AWDCR(RW)
    pub awdcr: Rw<u32>,
    /// 0x10 TRIGGER(RW)
    pub trigger: Rw<u32>,
    _pad0: [u32; 1], // 0x14..0x18
    /// 0x18 SAMPLE(RW)
    pub sample: Rw<u32>,
    _pad1: [u32; 1], // 0x1c..0x20
    /// 0x20 SQRCFR(RW)
    pub sqrcfr: Rw<u32>,
    _pad2: [u32; 3], // 0x24..0x30
    /// 0x30 RESULT0(RO)
    pub result0: Ro<u32>,
    /// 0x34 RESULT1(RO)
    pub result1: Ro<u32>,
    /// 0x38 RESULT2(RO)
    pub result2: Ro<u32>,
    /// 0x3C RESULT3(RO)
    pub result3: Ro<u32>,
    /// 0x40 RESULT4(RO)
    pub result4: Ro<u32>,
    /// 0x44 RESULT5(RO)
    pub result5: Ro<u32>,
    /// 0x48 RESULT6(RO)
    pub result6: Ro<u32>,
    /// 0x4C RESULT7(RO)
    pub result7: Ro<u32>,
    _pad3: [u32; 8], // 0x50..0x70
    /// 0x70 IER(RW)
    pub ier: Rw<u32>,
    /// 0x74 ICR(RW)
    pub icr: Rw<u32>,
    /// 0x78 ISR(RO)
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
    /// 0x10 ISR(RO)
    pub isr: Ro<u32>,
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

/// `BGR` 寄存器组(生成自厂商头文件)
///
/// - `BGR` 基地址 `0X400000FC`
#[allow(dead_code)]
#[repr(C)]
pub struct BGR {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
}

/// `BTIM` 寄存器组(生成自厂商头文件)
///
/// - `BTIM1` 基地址 `0X40004800`
#[allow(dead_code)]
#[repr(C)]
pub struct BTIM {
    /// 0x00 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x04 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x08 SMCR(RW)
    pub smcr: Rw<u32>,
    /// 0x0C DIER(RW)
    pub dier: Rw<u32>,
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

/// `CORDIC` 寄存器组(生成自厂商头文件)
///
/// - `CORDIC` 基地址 `0X40023400`
#[allow(dead_code)]
#[repr(C)]
pub struct CORDIC {
    /// 0x00 CSR(RW)
    pub csr: Rw<u32>,
    /// 0x04 X(RW)
    pub x: Rw<u32>,
    /// 0x08 Y(RW)
    pub y: Rw<u32>,
    /// 0x0C Z(RW)
    pub z: Rw<u32>,
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

/// `DAC` 寄存器组(生成自厂商头文件)
///
/// - `DAC` 基地址 `0X400000C0`
#[allow(dead_code)]
#[repr(C)]
pub struct DAC {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 SWTRGR(RW)
    pub swtrgr: Rw<u32>,
    /// 0x08 DHR12R1(RW)
    pub dhr12r1: Rw<u32>,
    /// 0x0C DHR12L1(RW)
    pub dhr12l1: Rw<u32>,
    /// 0x10 DHR8R1(RW)
    pub dhr8r1: Rw<u32>,
    /// 0x14 DHR12R2(RW)
    pub dhr12r2: Rw<u32>,
    /// 0x18 DHR12L2(RW)
    pub dhr12l2: Rw<u32>,
    /// 0x1C DHR8R2(RW)
    pub dhr8r2: Rw<u32>,
    /// 0x20 DHR12RD(RW)
    pub dhr12rd: Rw<u32>,
    /// 0x24 DHR12LD(RW)
    pub dhr12ld: Rw<u32>,
    /// 0x28 DHR8RD(RW)
    pub dhr8rd: Rw<u32>,
    /// 0x2C DOR1(RW)
    pub dor1: Rw<u32>,
    /// 0x30 DOR2(RW)
    pub dor2: Rw<u32>,
    /// 0x34 CR1(RW)
    pub cr1: Rw<u32>,
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
    /// 0x34 CCNT1(RO)
    pub ccnt1: Ro<u32>,
    /// 0x38 CSRCADDR1(RO)
    pub csrcaddr1: Ro<u32>,
    /// 0x3C CDSTADDR1(RO)
    pub cdstaddr1: Ro<u32>,
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
    /// 0x54 CCNT2(RO)
    pub ccnt2: Ro<u32>,
    /// 0x58 CSRCADDR2(RO)
    pub csrcaddr2: Ro<u32>,
    /// 0x5C CDSTADDR2(RO)
    pub cdstaddr2: Ro<u32>,
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
    /// 0x74 CCNT3(RO)
    pub ccnt3: Ro<u32>,
    /// 0x78 CSRCADDR3(RO)
    pub csrcaddr3: Ro<u32>,
    /// 0x7C CDSTADDR3(RO)
    pub cdstaddr3: Ro<u32>,
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
    /// 0x94 CCNT4(RO)
    pub ccnt4: Ro<u32>,
    /// 0x98 CSRCADDR4(RO)
    pub csrcaddr4: Ro<u32>,
    /// 0x9C CDSTADDR4(RO)
    pub cdstaddr4: Ro<u32>,
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
    /// 0x14 CCNT(RO)
    pub ccnt: Ro<u32>,
    /// 0x18 CSRCADDR(RO)
    pub csrcaddr: Ro<u32>,
    /// 0x1C CDSTADDR(RO)
    pub cdstaddr: Ro<u32>,
}

/// `EAU` 寄存器组(生成自厂商头文件)
///
/// - `EAU` 基地址 `0X40023200`
#[allow(dead_code)]
#[repr(C)]
pub struct EAU {
    /// 0x00 CSR(RW)
    pub csr: Rw<u32>,
    /// 0x04 DIVIDEND(RW)
    pub dividend: Rw<u32>,
    /// 0x08 DIVISOR(RW)
    pub divisor: Rw<u32>,
    /// 0x0C QUOTIENT(RO)
    pub quotient: Ro<u32>,
    /// 0x10 REMAINDER(RO)
    pub remainder: Ro<u32>,
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
#[allow(dead_code)]
#[repr(C)]
pub struct GPIO {
    /// 0x00 DIR(RW)
    pub dir: Rw<u32>,
    /// 0x04 OPENDRAIN(RW)
    pub opendrain: Rw<u32>,
    /// 0x08 RESERVED(RO)
    pub reserved: Ro<u32>,
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
    _pad0: [u32; 1], // 0x20..0x24
    /// 0x24 RISEIE(RW)
    pub riseie: Rw<u32>,
    /// 0x28 FALLIE(RW)
    pub fallie: Rw<u32>,
    _pad1: [u32; 2], // 0x2c..0x34
    /// 0x34 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x38 ICR(RW)
    pub icr: Rw<u32>,
    _pad2: [u32; 1], // 0x3c..0x40
    /// 0x40 FILTER(RW)
    pub filter: Rw<u32>,
    _pad3: [u32; 3], // 0x44..0x50
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
    /// 0x10 ISR(RO)
    pub isr: Ro<u32>,
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

/// `HALLTIM` 寄存器组(生成自厂商头文件)
///
/// - `HALLTIM` 基地址 `0X40006400`
#[allow(dead_code)]
#[repr(C)]
pub struct HALLTIM {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    /// 0x04 DIER(RW)
    pub dier: Rw<u32>,
    /// 0x08 CNT(RW)
    pub cnt: Rw<u32>,
    /// 0x0C ARR(RW)
    pub arr: Rw<u32>,
    /// 0x10 WIDTH(RO)
    pub width: Ro<u32>,
    /// 0x14 CCR(RW)
    pub ccr: Rw<u32>,
    /// 0x18 ISR(RO)
    pub isr: Ro<u32>,
    /// 0x1C ICR(RW)
    pub icr: Rw<u32>,
    /// 0x20 STATE(RO)
    pub state: Ro<u32>,
}

/// `I2C` 寄存器组(生成自厂商头文件)
///
/// - `I2C1` 基地址 `0X40005800`
#[allow(dead_code)]
#[repr(C)]
pub struct I2C {
    _pad0: [u32; 2], // 0x00..0x08
    /// 0x08 INSEL(RW)
    pub insel: Rw<u32>,
    /// 0x0C MICR(RW)
    pub micr: Rw<u32>,
    /// 0x10 MCR0(RW)
    pub mcr0: Rw<u32>,
    /// 0x14 MISR(RO)
    pub misr: Ro<u32>,
    /// 0x18 MIER(RW)
    pub mier: Rw<u32>,
    /// 0x1C MDER(RW)
    pub mder: Rw<u32>,
    /// 0x20 MCR1(RW)
    pub mcr1: Rw<u32>,
    /// 0x24 MCR2(RW)
    pub mcr2: Rw<u32>,
    /// 0x28 MCR3(RW)
    pub mcr3: Rw<u32>,
    /// 0x2C MCR4(RW)
    pub mcr4: Rw<u32>,
    _pad1: [u32; 4], // 0x30..0x40
    /// 0x40 MMATCH(RW)
    pub mmatch: Rw<u32>,
    _pad2: [u32; 1], // 0x44..0x48
    /// 0x48 MCCR(RW)
    pub mccr: Rw<u32>,
    _pad3: [u32; 3], // 0x4c..0x58
    /// 0x58 MFIFOCR(RW)
    pub mfifocr: Rw<u32>,
    /// 0x5C MFIFOSR(RO)
    pub mfifosr: Ro<u32>,
    /// 0x60 MTDR(WO)
    pub mtdr: Wo<u32>,
    _pad4: [u32; 3], // 0x64..0x70
    /// 0x70 MRDR(RO)
    pub mrdr: Ro<u32>,
    _pad5: [u32; 38], // 0x74..0x10c
    /// 0x10C SICR(RW)
    pub sicr: Rw<u32>,
    /// 0x110 SCR0(RW)
    pub scr0: Rw<u32>,
    /// 0x114 SISR(RO)
    pub sisr: Ro<u32>,
    /// 0x118 SIER(RW)
    pub sier: Rw<u32>,
    /// 0x11C SDER(RW)
    pub sder: Rw<u32>,
    _pad6: [u32; 1], // 0x120..0x124
    /// 0x124 SCR1(RW)
    pub scr1: Rw<u32>,
    /// 0x128 SCR3(RW)
    pub scr3: Rw<u32>,
    _pad7: [u32; 5], // 0x12c..0x140
    /// 0x140 SAMR(RW)
    pub samr: Rw<u32>,
    _pad8: [u32; 3], // 0x144..0x150
    /// 0x150 SASR(RO)
    pub sasr: Ro<u32>,
    /// 0x154 STAR(RW)
    pub star: Rw<u32>,
    _pad9: [u32; 2], // 0x158..0x160
    /// 0x160 STDR(WO)
    pub stdr: Wo<u32>,
    _pad10: [u32; 3], // 0x164..0x170
    /// 0x170 SRDR(RO)
    pub srdr: Ro<u32>,
}

/// `IRMOD` 寄存器组(生成自厂商头文件)
///
/// - `IRMOD` 基地址 `0X40004080`
#[allow(dead_code)]
#[repr(C)]
pub struct IRMOD {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    _pad0: [u32; 2], // 尾部到 Size=12
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
    /// 0x10 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x14 CMP(RW)
    pub cmp: Rw<u32>,
    /// 0x18 ARR(RW)
    pub arr: Rw<u32>,
    /// 0x1C CNT(RO)
    pub cnt: Ro<u32>,
    /// 0x20 CR1(RW)
    pub cr1: Rw<u32>,
}

/// `LVD` 寄存器组(生成自厂商头文件)
///
/// - `LVD` 基地址 `0X400000A4`
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

/// `OPA` 寄存器组(生成自厂商头文件)
///
/// - `OPA1` 基地址 `0X400000B0`
#[allow(dead_code)]
#[repr(C)]
pub struct OPA {
    /// 0x00 CR(RW)
    pub cr: Rw<u32>,
    /// 0x04 CAL(RW)
    pub cal: Rw<u32>,
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
    _pad0: [u32; 5], // 尾部到 Size=92
}

/// `SPI` 寄存器组(生成自厂商头文件)
///
/// - `SPI1` 基地址 `0X40000800`
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

/// `UART` 寄存器组(生成自厂商头文件)
///
/// - `UART1` 基地址 `0X40000C00`
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

/// `VC` 寄存器组(生成自厂商头文件)
///
/// - `VC1` 基地址 `0X40000084`
#[allow(dead_code)]
#[repr(C)]
pub struct VC {
    /// 0x00 CR0(RW)
    pub cr0: Rw<u32>,
    /// 0x04 CR1(RW)
    pub cr1: Rw<u32>,
    /// 0x08 CR2(RW)
    pub cr2: Rw<u32>,
    /// 0x0C SR(RW)
    pub sr: Rw<u32>,
}

/// `VCREF` 寄存器组(生成自厂商头文件)
///
/// - `VC12REF` 基地址 `0X40000080`
#[allow(dead_code)]
#[repr(C)]
pub struct VCREF {
    /// 0x00 REF(RW)
    pub r#ref: Rw<u32>,
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
        /// CONT[1:1]
        pub const cont_POS: u32 = 1;
        pub const cont_MASK: u32 = 0x2;
        /// CLK[3:2]
        pub const clk_POS: u32 = 2;
        pub const clk_MASK: u32 = 0xc;
        /// ENS[6:4]
        pub const ens_POS: u32 = 4;
        pub const ens_MASK: u32 = 0x70;
        /// SLAVE[7:7]
        pub const slave_POS: u32 = 7;
        pub const slave_MASK: u32 = 0x80;
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
    /// TRIGGER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trigger {
        /// ATIMTRGO[0:0]
        pub const atimtrgo_POS: u32 = 0;
        pub const atimtrgo_MASK: u32 = 0x1;
        /// ATIMTRGO2[1:1]
        pub const atimtrgo2_POS: u32 = 1;
        pub const atimtrgo2_MASK: u32 = 0x2;
        /// ATIMOC1REFC[2:2]
        pub const atimoc1refc_POS: u32 = 2;
        pub const atimoc1refc_MASK: u32 = 0x4;
        /// ATIMOC2REFC[3:3]
        pub const atimoc2refc_POS: u32 = 3;
        pub const atimoc2refc_MASK: u32 = 0x8;
        /// ATIMOC3REFC[4:4]
        pub const atimoc3refc_POS: u32 = 4;
        pub const atimoc3refc_MASK: u32 = 0x10;
        /// ATIMOC4REFC[5:5]
        pub const atimoc4refc_POS: u32 = 5;
        pub const atimoc4refc_MASK: u32 = 0x20;
        /// ATIMOC5REFC[6:6]
        pub const atimoc5refc_POS: u32 = 6;
        pub const atimoc5refc_MASK: u32 = 0x40;
        /// ATIMOC6REFC[7:7]
        pub const atimoc6refc_POS: u32 = 7;
        pub const atimoc6refc_MASK: u32 = 0x80;
        /// GTIM1TRGO[8:8]
        pub const gtim1trgo_POS: u32 = 8;
        pub const gtim1trgo_MASK: u32 = 0x100;
        /// GTIM1OC1REFC[9:9]
        pub const gtim1oc1refc_POS: u32 = 9;
        pub const gtim1oc1refc_MASK: u32 = 0x200;
        /// GTIM1OC2REFC[10:10]
        pub const gtim1oc2refc_POS: u32 = 10;
        pub const gtim1oc2refc_MASK: u32 = 0x400;
        /// GTIM1OC3REFC[11:11]
        pub const gtim1oc3refc_POS: u32 = 11;
        pub const gtim1oc3refc_MASK: u32 = 0x800;
        /// GTIM1OC4REFC[12:12]
        pub const gtim1oc4refc_POS: u32 = 12;
        pub const gtim1oc4refc_MASK: u32 = 0x1000;
        /// GTIM2TRGO[13:13]
        pub const gtim2trgo_POS: u32 = 13;
        pub const gtim2trgo_MASK: u32 = 0x2000;
        /// GTIM2OC1REFC[14:14]
        pub const gtim2oc1refc_POS: u32 = 14;
        pub const gtim2oc1refc_MASK: u32 = 0x4000;
        /// GTIM2OC2REFC[15:15]
        pub const gtim2oc2refc_POS: u32 = 15;
        pub const gtim2oc2refc_MASK: u32 = 0x8000;
        /// GTIM2OC3REFC[16:16]
        pub const gtim2oc3refc_POS: u32 = 16;
        pub const gtim2oc3refc_MASK: u32 = 0x10000;
        /// GTIM2OC4REFC[17:17]
        pub const gtim2oc4refc_POS: u32 = 17;
        pub const gtim2oc4refc_MASK: u32 = 0x20000;
        /// GTIM3TRGO[18:18]
        pub const gtim3trgo_POS: u32 = 18;
        pub const gtim3trgo_MASK: u32 = 0x40000;
        /// GTIM3OC1REFC[19:19]
        pub const gtim3oc1refc_POS: u32 = 19;
        pub const gtim3oc1refc_MASK: u32 = 0x80000;
        /// GTIM3OC2REFC[20:20]
        pub const gtim3oc2refc_POS: u32 = 20;
        pub const gtim3oc2refc_MASK: u32 = 0x100000;
        /// GTIM3OC3REFC[21:21]
        pub const gtim3oc3refc_POS: u32 = 21;
        pub const gtim3oc3refc_MASK: u32 = 0x200000;
        /// GTIM3OC4REFC[22:22]
        pub const gtim3oc4refc_POS: u32 = 22;
        pub const gtim3oc4refc_MASK: u32 = 0x400000;
        /// GTIM4TRGO[23:23]
        pub const gtim4trgo_POS: u32 = 23;
        pub const gtim4trgo_MASK: u32 = 0x800000;
        /// GTIM4OC1REFC[24:24]
        pub const gtim4oc1refc_POS: u32 = 24;
        pub const gtim4oc1refc_MASK: u32 = 0x1000000;
        /// GTIM4OC2REFC[25:25]
        pub const gtim4oc2refc_POS: u32 = 25;
        pub const gtim4oc2refc_MASK: u32 = 0x2000000;
        /// GTIM4OC3REFC[26:26]
        pub const gtim4oc3refc_POS: u32 = 26;
        pub const gtim4oc3refc_MASK: u32 = 0x4000000;
        /// GTIM4OC4REFC[27:27]
        pub const gtim4oc4refc_POS: u32 = 27;
        pub const gtim4oc4refc_MASK: u32 = 0x8000000;
        /// BTIM1TRGO[28:28]
        pub const btim1trgo_POS: u32 = 28;
        pub const btim1trgo_MASK: u32 = 0x10000000;
        /// BTIM2TRGO[29:29]
        pub const btim2trgo_POS: u32 = 29;
        pub const btim2trgo_MASK: u32 = 0x20000000;
        /// BTIM3TRGO[30:30]
        pub const btim3trgo_POS: u32 = 30;
        pub const btim3trgo_MASK: u32 = 0x40000000;
        /// HALLTIMTRGO[31:31]
        pub const halltimtrgo_POS: u32 = 31;
        pub const halltimtrgo_MASK: u32 = 0x80000000;
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
        /// AWDL[3:3]
        pub const awdl_POS: u32 = 3;
        pub const awdl_MASK: u32 = 0x8;
        /// AWDH[4:4]
        pub const awdh_POS: u32 = 4;
        pub const awdh_MASK: u32 = 0x10;
        /// DMAEOC[5:5]
        pub const dmaeoc_POS: u32 = 5;
        pub const dmaeoc_MASK: u32 = 0x20;
        /// DMAEOS[6:6]
        pub const dmaeos_POS: u32 = 6;
        pub const dmaeos_MASK: u32 = 0x40;
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
        /// AWDL[3:3]
        pub const awdl_POS: u32 = 3;
        pub const awdl_MASK: u32 = 0x8;
        /// AWDH[4:4]
        pub const awdh_POS: u32 = 4;
        pub const awdh_MASK: u32 = 0x10;
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
        /// AWDL[3:3]
        pub const awdl_POS: u32 = 3;
        pub const awdl_MASK: u32 = 0x8;
        /// AWDH[4:4]
        pub const awdh_POS: u32 = 4;
        pub const awdh_MASK: u32 = 0x10;
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
        /// CCDS[3:3]
        pub const ccds_POS: u32 = 3;
        pub const ccds_MASK: u32 = 0x8;
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
        /// UDE[8:8]
        pub const ude_POS: u32 = 8;
        pub const ude_MASK: u32 = 0x100;
        /// CC1DE[9:9]
        pub const cc1de_POS: u32 = 9;
        pub const cc1de_MASK: u32 = 0x200;
        /// CC2DE[10:10]
        pub const cc2de_POS: u32 = 10;
        pub const cc2de_MASK: u32 = 0x400;
        /// CC3DE[11:11]
        pub const cc3de_POS: u32 = 11;
        pub const cc3de_MASK: u32 = 0x800;
        /// CC4DE[12:12]
        pub const cc4de_POS: u32 = 12;
        pub const cc4de_MASK: u32 = 0x1000;
        /// COMDE[13:13]
        pub const comde_POS: u32 = 13;
        pub const comde_MASK: u32 = 0x2000;
        /// TDE[14:14]
        pub const tde_POS: u32 = 14;
        pub const tde_MASK: u32 = 0x4000;
        /// CC5IE[16:16]
        pub const cc5ie_POS: u32 = 16;
        pub const cc5ie_MASK: u32 = 0x10000;
        /// CC6IE[17:17]
        pub const cc6ie_POS: u32 = 17;
        pub const cc6ie_MASK: u32 = 0x20000;
        /// CC5DE[18:18]
        pub const cc5de_POS: u32 = 18;
        pub const cc5de_MASK: u32 = 0x40000;
        /// CC6DE[19:19]
        pub const cc6de_POS: u32 = 19;
        pub const cc6de_MASK: u32 = 0x80000;
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
        /// BKVC3E[3:3]
        pub const bkvc3e_POS: u32 = 3;
        pub const bkvc3e_MASK: u32 = 0x8;
        /// BKVC4E[4:4]
        pub const bkvc4e_POS: u32 = 4;
        pub const bkvc4e_MASK: u32 = 0x10;
        /// BKINP[9:9]
        pub const bkinp_POS: u32 = 9;
        pub const bkinp_MASK: u32 = 0x200;
        /// BKVC1P[10:10]
        pub const bkvc1p_POS: u32 = 10;
        pub const bkvc1p_MASK: u32 = 0x400;
        /// BKVC2P[11:11]
        pub const bkvc2p_POS: u32 = 11;
        pub const bkvc2p_MASK: u32 = 0x800;
        /// BKVC3P[12:12]
        pub const bkvc3p_POS: u32 = 12;
        pub const bkvc3p_MASK: u32 = 0x1000;
        /// BKVC4P[13:13]
        pub const bkvc4p_POS: u32 = 13;
        pub const bkvc4p_MASK: u32 = 0x2000;
        /// ETRSEL[19:16]
        pub const etrsel_POS: u32 = 16;
        pub const etrsel_MASK: u32 = 0xf0000;
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
        /// BK2VC3E[3:3]
        pub const bk2vc3e_POS: u32 = 3;
        pub const bk2vc3e_MASK: u32 = 0x8;
        /// BK2VC4E[4:4]
        pub const bk2vc4e_POS: u32 = 4;
        pub const bk2vc4e_MASK: u32 = 0x10;
        /// BK2INP[9:9]
        pub const bk2inp_POS: u32 = 9;
        pub const bk2inp_MASK: u32 = 0x200;
        /// BK2VC1P[10:10]
        pub const bk2vc1p_POS: u32 = 10;
        pub const bk2vc1p_MASK: u32 = 0x400;
        /// BK2VC2P[11:11]
        pub const bk2vc2p_POS: u32 = 11;
        pub const bk2vc2p_MASK: u32 = 0x800;
        /// BK2VC3P[12:12]
        pub const bk2vc3p_POS: u32 = 12;
        pub const bk2vc3p_MASK: u32 = 0x1000;
        /// BK2VC4P[13:13]
        pub const bk2vc4p_POS: u32 = 13;
        pub const bk2vc4p_MASK: u32 = 0x2000;
        /// OCRSEL[19:16]
        pub const ocrsel_POS: u32 = 16;
        pub const ocrsel_MASK: u32 = 0xf0000;
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
/// `BGR` 位段常量
pub mod bgr {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// BGREN[0:0]
        pub const bgren_POS: u32 = 0;
        pub const bgren_MASK: u32 = 0x1;
        /// TSEN[1:1]
        pub const tsen_POS: u32 = 1;
        pub const tsen_MASK: u32 = 0x2;
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
    /// DIER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dier {
        /// UIE[0:0]
        pub const uie_POS: u32 = 0;
        pub const uie_MASK: u32 = 0x1;
        /// TIE[6:6]
        pub const tie_POS: u32 = 6;
        pub const tie_MASK: u32 = 0x40;
        /// UDE[8:8]
        pub const ude_POS: u32 = 8;
        pub const ude_MASK: u32 = 0x100;
        /// TDE[14:14]
        pub const tde_POS: u32 = 14;
        pub const tde_MASK: u32 = 0x4000;
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
/// `CORDIC` 位段常量
pub mod cordic {
    /// CSR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr {
        /// FUNC[3:0]
        pub const func_POS: u32 = 0;
        pub const func_MASK: u32 = 0xf;
        /// SCALE[6:4]
        pub const scale_POS: u32 = 4;
        pub const scale_MASK: u32 = 0x70;
        /// FORMAT[7:7]
        pub const format_POS: u32 = 7;
        pub const format_MASK: u32 = 0x80;
        /// ITER[11:8]
        pub const iter_POS: u32 = 8;
        pub const iter_MASK: u32 = 0xf00;
        /// COMP[12:12]
        pub const comp_POS: u32 = 12;
        pub const comp_MASK: u32 = 0x1000;
        /// IE[13:13]
        pub const ie_POS: u32 = 13;
        pub const ie_MASK: u32 = 0x2000;
        /// DMAEOC[14:14]
        pub const dmaeoc_POS: u32 = 14;
        pub const dmaeoc_MASK: u32 = 0x4000;
        /// DMAIDLE[15:15]
        pub const dmaidle_POS: u32 = 15;
        pub const dmaidle_MASK: u32 = 0x8000;
        /// EOC[16:16]
        pub const eoc_POS: u32 = 16;
        pub const eoc_MASK: u32 = 0x10000;
        /// BUSY[17:17]
        pub const busy_POS: u32 = 17;
        pub const busy_MASK: u32 = 0x20000;
    }
    /// X 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod x {
        /// X[31:0]
        pub const x_POS: u32 = 0;
        pub const x_MASK: u32 = 0xffffffff;
    }
    /// Y 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod y {
        /// Y[31:0]
        pub const y_POS: u32 = 0;
        pub const y_MASK: u32 = 0xffffffff;
    }
    /// Z 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod z {
        /// Z[31:0]
        pub const z_POS: u32 = 0;
        pub const z_MASK: u32 = 0xffffffff;
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
/// `DAC` 位段常量
pub mod dac {
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
        /// EN1[0:0]
        pub const en1_POS: u32 = 0;
        pub const en1_MASK: u32 = 0x1;
        /// TEN1[1:1]
        pub const ten1_POS: u32 = 1;
        pub const ten1_MASK: u32 = 0x2;
        /// TSEL1[5:2]
        pub const tsel1_POS: u32 = 2;
        pub const tsel1_MASK: u32 = 0x3c;
        /// WAVE1[7:6]
        pub const wave1_POS: u32 = 6;
        pub const wave1_MASK: u32 = 0xc0;
        /// MAMP1[11:8]
        pub const mamp1_POS: u32 = 8;
        pub const mamp1_MASK: u32 = 0xf00;
        /// DMAEN1[12:12]
        pub const dmaen1_POS: u32 = 12;
        pub const dmaen1_MASK: u32 = 0x1000;
        /// DMAUDRIE1[13:13]
        pub const dmaudrie1_POS: u32 = 13;
        pub const dmaudrie1_MASK: u32 = 0x2000;
        /// EN2[16:16]
        pub const en2_POS: u32 = 16;
        pub const en2_MASK: u32 = 0x10000;
        /// TEN2[17:17]
        pub const ten2_POS: u32 = 17;
        pub const ten2_MASK: u32 = 0x20000;
        /// TSEL2[21:18]
        pub const tsel2_POS: u32 = 18;
        pub const tsel2_MASK: u32 = 0x3c0000;
        /// WAVE2[23:22]
        pub const wave2_POS: u32 = 22;
        pub const wave2_MASK: u32 = 0xc00000;
        /// MAMP2[27:24]
        pub const mamp2_POS: u32 = 24;
        pub const mamp2_MASK: u32 = 0xf000000;
        /// DMAEN2[28:28]
        pub const dmaen2_POS: u32 = 28;
        pub const dmaen2_MASK: u32 = 0x10000000;
        /// DMAUDRIE2[29:29]
        pub const dmaudrie2_POS: u32 = 29;
        pub const dmaudrie2_MASK: u32 = 0x20000000;
    }
    /// SWTRGR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod swtrgr {
        /// SWTRIG1[0:0]
        pub const swtrig1_POS: u32 = 0;
        pub const swtrig1_MASK: u32 = 0x1;
        /// SWTRIG2[1:1]
        pub const swtrig2_POS: u32 = 1;
        pub const swtrig2_MASK: u32 = 0x2;
    }
    /// DHR12R1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr12r1 {
        /// DATA[11:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xfff;
    }
    /// DHR12L1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr12l1 {
        /// DATA[15:4]
        pub const data_POS: u32 = 4;
        pub const data_MASK: u32 = 0xfff0;
    }
    /// DHR12R2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr12r2 {
        /// DATA[11:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xfff;
    }
    /// DHR12L2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr12l2 {
        /// DATA[15:4]
        pub const data_POS: u32 = 4;
        pub const data_MASK: u32 = 0xfff0;
    }
    /// DHR12RD 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr12rd {
        /// C1DATA[11:0]
        pub const c1data_POS: u32 = 0;
        pub const c1data_MASK: u32 = 0xfff;
        /// C2DATA[27:16]
        pub const c2data_POS: u32 = 16;
        pub const c2data_MASK: u32 = 0xfff0000;
    }
    /// DHR12LD 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr12ld {
        /// C1DATA[15:4]
        pub const c1data_POS: u32 = 4;
        pub const c1data_MASK: u32 = 0xfff0;
        /// C2DATA[31:20]
        pub const c2data_POS: u32 = 20;
        pub const c2data_MASK: u32 = 0xfff00000;
    }
    /// DHR8RD 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dhr8rd {
        /// C1DATA[7:0]
        pub const c1data_POS: u32 = 0;
        pub const c1data_MASK: u32 = 0xff;
        /// C2DATA[15:8]
        pub const c2data_POS: u32 = 8;
        pub const c2data_MASK: u32 = 0xff00;
    }
    /// DOR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dor1 {
        /// DATA[12:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0x1fff;
        /// DMAUDR[15:15]
        pub const dmaudr_POS: u32 = 15;
        pub const dmaudr_MASK: u32 = 0x8000;
    }
    /// DOR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dor2 {
        /// DATA[11:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xfff;
        /// DMAUDR[15:15]
        pub const dmaudr_POS: u32 = 15;
        pub const dmaudr_MASK: u32 = 0x8000;
    }
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// C1OUT[0:0]
        pub const c1out_POS: u32 = 0;
        pub const c1out_MASK: u32 = 0x1;
        /// C2OUT[1:1]
        pub const c2out_POS: u32 = 1;
        pub const c2out_MASK: u32 = 0x2;
    }
}
/// `DMA` 位段常量
pub mod dma {
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// TC1[0:0]
        pub const tc1_POS: u32 = 0;
        pub const tc1_MASK: u32 = 0x1;
        /// TE1[1:1]
        pub const te1_POS: u32 = 1;
        pub const te1_MASK: u32 = 0x2;
        /// TC2[4:4]
        pub const tc2_POS: u32 = 4;
        pub const tc2_MASK: u32 = 0x10;
        /// TE2[5:5]
        pub const te2_POS: u32 = 5;
        pub const te2_MASK: u32 = 0x20;
        /// TC3[8:8]
        pub const tc3_POS: u32 = 8;
        pub const tc3_MASK: u32 = 0x100;
        /// TE3[9:9]
        pub const te3_POS: u32 = 9;
        pub const te3_MASK: u32 = 0x200;
        /// TC4[12:12]
        pub const tc4_POS: u32 = 12;
        pub const tc4_MASK: u32 = 0x1000;
        /// TE4[13:13]
        pub const te4_POS: u32 = 13;
        pub const te4_MASK: u32 = 0x2000;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// TC1[0:0]
        pub const tc1_POS: u32 = 0;
        pub const tc1_MASK: u32 = 0x1;
        /// TE1[1:1]
        pub const te1_POS: u32 = 1;
        pub const te1_MASK: u32 = 0x2;
        /// TC2[4:4]
        pub const tc2_POS: u32 = 4;
        pub const tc2_MASK: u32 = 0x10;
        /// TE2[5:5]
        pub const te2_POS: u32 = 5;
        pub const te2_MASK: u32 = 0x20;
        /// TC3[8:8]
        pub const tc3_POS: u32 = 8;
        pub const tc3_MASK: u32 = 0x100;
        /// TE3[9:9]
        pub const te3_POS: u32 = 9;
        pub const te3_MASK: u32 = 0x200;
        /// TC4[12:12]
        pub const tc4_POS: u32 = 12;
        pub const tc4_MASK: u32 = 0x1000;
        /// TE4[13:13]
        pub const te4_POS: u32 = 13;
        pub const te4_MASK: u32 = 0x2000;
    }
    /// CSR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr1 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// TCIE[1:1]
        pub const tcie_POS: u32 = 1;
        pub const tcie_MASK: u32 = 0x2;
        /// TEIE[2:2]
        pub const teie_POS: u32 = 2;
        pub const teie_MASK: u32 = 0x4;
        /// TRANS[3:3]
        pub const trans_POS: u32 = 3;
        pub const trans_MASK: u32 = 0x8;
        /// SRCINC[4:4]
        pub const srcinc_POS: u32 = 4;
        pub const srcinc_MASK: u32 = 0x10;
        /// DSTINC[5:5]
        pub const dstinc_POS: u32 = 5;
        pub const dstinc_MASK: u32 = 0x20;
        /// SIZE[7:6]
        pub const size_POS: u32 = 6;
        pub const size_MASK: u32 = 0xc0;
        /// STATUS[10:8]
        pub const status_POS: u32 = 8;
        pub const status_MASK: u32 = 0x700;
        /// RESTART[11:11]
        pub const restart_POS: u32 = 11;
        pub const restart_MASK: u32 = 0x800;
        /// TC[12:12]
        pub const tc_POS: u32 = 12;
        pub const tc_MASK: u32 = 0x1000;
        /// TE[13:13]
        pub const te_POS: u32 = 13;
        pub const te_MASK: u32 = 0x2000;
        /// SRCLOAD[14:14]
        pub const srcload_POS: u32 = 14;
        pub const srcload_MASK: u32 = 0x4000;
        /// DSTLOAD[15:15]
        pub const dstload_POS: u32 = 15;
        pub const dstload_MASK: u32 = 0x8000;
    }
    /// CNT1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt1 {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// REPEAT[19:16]
        pub const repeat_POS: u32 = 16;
        pub const repeat_MASK: u32 = 0xf0000;
    }
    /// SRCADDR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod srcaddr1 {
        /// SRCADDR[31:0]
        pub const srcaddr_POS: u32 = 0;
        pub const srcaddr_MASK: u32 = 0xffffffff;
    }
    /// DSTADDR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dstaddr1 {
        /// DSTADDR[31:0]
        pub const dstaddr_POS: u32 = 0;
        pub const dstaddr_MASK: u32 = 0xffffffff;
    }
    /// TRIG1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trig1 {
        /// TYPE[0:0]
        pub const r#type_POS: u32 = 0;
        pub const r#type_MASK: u32 = 0x1;
        /// SOFTSRC[1:1]
        pub const softsrc_POS: u32 = 1;
        pub const softsrc_MASK: u32 = 0x2;
        /// HARDSRC[7:2]
        pub const hardsrc_POS: u32 = 2;
        pub const hardsrc_MASK: u32 = 0xfc;
    }
    /// CCNT1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccnt1 {
        /// CCNT[15:0]
        pub const ccnt_POS: u32 = 0;
        pub const ccnt_MASK: u32 = 0xffff;
        /// CREPEAT[19:16]
        pub const crepeat_POS: u32 = 16;
        pub const crepeat_MASK: u32 = 0xf0000;
    }
    /// CSRCADDR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csrcaddr1 {
        /// CSRCADDR[31:0]
        pub const csrcaddr_POS: u32 = 0;
        pub const csrcaddr_MASK: u32 = 0xffffffff;
    }
    /// CDSTADDR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cdstaddr1 {
        /// CDSTADDR[31:0]
        pub const cdstaddr_POS: u32 = 0;
        pub const cdstaddr_MASK: u32 = 0xffffffff;
    }
    /// CSR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr2 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// TCIE[1:1]
        pub const tcie_POS: u32 = 1;
        pub const tcie_MASK: u32 = 0x2;
        /// TEIE[2:2]
        pub const teie_POS: u32 = 2;
        pub const teie_MASK: u32 = 0x4;
        /// TRANS[3:3]
        pub const trans_POS: u32 = 3;
        pub const trans_MASK: u32 = 0x8;
        /// SRCINC[4:4]
        pub const srcinc_POS: u32 = 4;
        pub const srcinc_MASK: u32 = 0x10;
        /// DSTINC[5:5]
        pub const dstinc_POS: u32 = 5;
        pub const dstinc_MASK: u32 = 0x20;
        /// SIZE[7:6]
        pub const size_POS: u32 = 6;
        pub const size_MASK: u32 = 0xc0;
        /// STATUS[10:8]
        pub const status_POS: u32 = 8;
        pub const status_MASK: u32 = 0x700;
        /// RESTART[11:11]
        pub const restart_POS: u32 = 11;
        pub const restart_MASK: u32 = 0x800;
        /// TC[12:12]
        pub const tc_POS: u32 = 12;
        pub const tc_MASK: u32 = 0x1000;
        /// TE[13:13]
        pub const te_POS: u32 = 13;
        pub const te_MASK: u32 = 0x2000;
        /// SRCLOAD[14:14]
        pub const srcload_POS: u32 = 14;
        pub const srcload_MASK: u32 = 0x4000;
        /// DSTLOAD[15:15]
        pub const dstload_POS: u32 = 15;
        pub const dstload_MASK: u32 = 0x8000;
    }
    /// CNT2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt2 {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// REPEAT[19:16]
        pub const repeat_POS: u32 = 16;
        pub const repeat_MASK: u32 = 0xf0000;
    }
    /// SRCADDR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod srcaddr2 {
        /// SRCADDR[31:0]
        pub const srcaddr_POS: u32 = 0;
        pub const srcaddr_MASK: u32 = 0xffffffff;
    }
    /// DSTADDR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dstaddr2 {
        /// DSTADDR[31:0]
        pub const dstaddr_POS: u32 = 0;
        pub const dstaddr_MASK: u32 = 0xffffffff;
    }
    /// TRIG2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trig2 {
        /// TYPE[0:0]
        pub const r#type_POS: u32 = 0;
        pub const r#type_MASK: u32 = 0x1;
        /// SOFTSRC[1:1]
        pub const softsrc_POS: u32 = 1;
        pub const softsrc_MASK: u32 = 0x2;
        /// HARDSRC[7:2]
        pub const hardsrc_POS: u32 = 2;
        pub const hardsrc_MASK: u32 = 0xfc;
    }
    /// CCNT2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccnt2 {
        /// CCNT[15:0]
        pub const ccnt_POS: u32 = 0;
        pub const ccnt_MASK: u32 = 0xffff;
        /// CREPEAT[19:16]
        pub const crepeat_POS: u32 = 16;
        pub const crepeat_MASK: u32 = 0xf0000;
    }
    /// CSRCADDR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csrcaddr2 {
        /// CSRCADDR[31:0]
        pub const csrcaddr_POS: u32 = 0;
        pub const csrcaddr_MASK: u32 = 0xffffffff;
    }
    /// CDSTADDR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cdstaddr2 {
        /// CDSTADDR[31:0]
        pub const cdstaddr_POS: u32 = 0;
        pub const cdstaddr_MASK: u32 = 0xffffffff;
    }
    /// CSR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr3 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// TCIE[1:1]
        pub const tcie_POS: u32 = 1;
        pub const tcie_MASK: u32 = 0x2;
        /// TEIE[2:2]
        pub const teie_POS: u32 = 2;
        pub const teie_MASK: u32 = 0x4;
        /// TRANS[3:3]
        pub const trans_POS: u32 = 3;
        pub const trans_MASK: u32 = 0x8;
        /// SRCINC[4:4]
        pub const srcinc_POS: u32 = 4;
        pub const srcinc_MASK: u32 = 0x10;
        /// DSTINC[5:5]
        pub const dstinc_POS: u32 = 5;
        pub const dstinc_MASK: u32 = 0x20;
        /// SIZE[7:6]
        pub const size_POS: u32 = 6;
        pub const size_MASK: u32 = 0xc0;
        /// STATUS[10:8]
        pub const status_POS: u32 = 8;
        pub const status_MASK: u32 = 0x700;
        /// RESTART[11:11]
        pub const restart_POS: u32 = 11;
        pub const restart_MASK: u32 = 0x800;
        /// TC[12:12]
        pub const tc_POS: u32 = 12;
        pub const tc_MASK: u32 = 0x1000;
        /// TE[13:13]
        pub const te_POS: u32 = 13;
        pub const te_MASK: u32 = 0x2000;
        /// SRCLOAD[14:14]
        pub const srcload_POS: u32 = 14;
        pub const srcload_MASK: u32 = 0x4000;
        /// DSTLOAD[15:15]
        pub const dstload_POS: u32 = 15;
        pub const dstload_MASK: u32 = 0x8000;
    }
    /// CNT3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt3 {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// REPEAT[19:16]
        pub const repeat_POS: u32 = 16;
        pub const repeat_MASK: u32 = 0xf0000;
    }
    /// SRCADDR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod srcaddr3 {
        /// SRCADDR[31:0]
        pub const srcaddr_POS: u32 = 0;
        pub const srcaddr_MASK: u32 = 0xffffffff;
    }
    /// DSTADDR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dstaddr3 {
        /// DSTADDR[31:0]
        pub const dstaddr_POS: u32 = 0;
        pub const dstaddr_MASK: u32 = 0xffffffff;
    }
    /// TRIG3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trig3 {
        /// TYPE[0:0]
        pub const r#type_POS: u32 = 0;
        pub const r#type_MASK: u32 = 0x1;
        /// SOFTSRC[1:1]
        pub const softsrc_POS: u32 = 1;
        pub const softsrc_MASK: u32 = 0x2;
        /// HARDSRC[7:2]
        pub const hardsrc_POS: u32 = 2;
        pub const hardsrc_MASK: u32 = 0xfc;
    }
    /// CCNT3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccnt3 {
        /// CCNT[15:0]
        pub const ccnt_POS: u32 = 0;
        pub const ccnt_MASK: u32 = 0xffff;
        /// CREPEAT[19:16]
        pub const crepeat_POS: u32 = 16;
        pub const crepeat_MASK: u32 = 0xf0000;
    }
    /// CSRCADDR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csrcaddr3 {
        /// CSRCADDR[31:0]
        pub const csrcaddr_POS: u32 = 0;
        pub const csrcaddr_MASK: u32 = 0xffffffff;
    }
    /// CDSTADDR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cdstaddr3 {
        /// CDSTADDR[31:0]
        pub const cdstaddr_POS: u32 = 0;
        pub const cdstaddr_MASK: u32 = 0xffffffff;
    }
    /// CSR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr4 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// TCIE[1:1]
        pub const tcie_POS: u32 = 1;
        pub const tcie_MASK: u32 = 0x2;
        /// TEIE[2:2]
        pub const teie_POS: u32 = 2;
        pub const teie_MASK: u32 = 0x4;
        /// TRANS[3:3]
        pub const trans_POS: u32 = 3;
        pub const trans_MASK: u32 = 0x8;
        /// SRCINC[4:4]
        pub const srcinc_POS: u32 = 4;
        pub const srcinc_MASK: u32 = 0x10;
        /// DSTINC[5:5]
        pub const dstinc_POS: u32 = 5;
        pub const dstinc_MASK: u32 = 0x20;
        /// SIZE[7:6]
        pub const size_POS: u32 = 6;
        pub const size_MASK: u32 = 0xc0;
        /// STATUS[10:8]
        pub const status_POS: u32 = 8;
        pub const status_MASK: u32 = 0x700;
        /// RESTART[11:11]
        pub const restart_POS: u32 = 11;
        pub const restart_MASK: u32 = 0x800;
        /// TC[12:12]
        pub const tc_POS: u32 = 12;
        pub const tc_MASK: u32 = 0x1000;
        /// TE[13:13]
        pub const te_POS: u32 = 13;
        pub const te_MASK: u32 = 0x2000;
        /// SRCLOAD[14:14]
        pub const srcload_POS: u32 = 14;
        pub const srcload_MASK: u32 = 0x4000;
        /// DSTLOAD[15:15]
        pub const dstload_POS: u32 = 15;
        pub const dstload_MASK: u32 = 0x8000;
    }
    /// CNT4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt4 {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// REPEAT[19:16]
        pub const repeat_POS: u32 = 16;
        pub const repeat_MASK: u32 = 0xf0000;
    }
    /// SRCADDR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod srcaddr4 {
        /// SRCADDR[31:0]
        pub const srcaddr_POS: u32 = 0;
        pub const srcaddr_MASK: u32 = 0xffffffff;
    }
    /// DSTADDR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dstaddr4 {
        /// DSTADDR[31:0]
        pub const dstaddr_POS: u32 = 0;
        pub const dstaddr_MASK: u32 = 0xffffffff;
    }
    /// TRIG4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trig4 {
        /// TYPE[0:0]
        pub const r#type_POS: u32 = 0;
        pub const r#type_MASK: u32 = 0x1;
        /// SOFTSRC[1:1]
        pub const softsrc_POS: u32 = 1;
        pub const softsrc_MASK: u32 = 0x2;
        /// HARDSRC[7:2]
        pub const hardsrc_POS: u32 = 2;
        pub const hardsrc_MASK: u32 = 0xfc;
    }
    /// CCNT4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccnt4 {
        /// CCNT[15:0]
        pub const ccnt_POS: u32 = 0;
        pub const ccnt_MASK: u32 = 0xffff;
        /// CREPEAT[19:16]
        pub const crepeat_POS: u32 = 16;
        pub const crepeat_MASK: u32 = 0xf0000;
    }
    /// CSRCADDR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csrcaddr4 {
        /// CSRCADDR[31:0]
        pub const csrcaddr_POS: u32 = 0;
        pub const csrcaddr_MASK: u32 = 0xffffffff;
    }
    /// CDSTADDR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cdstaddr4 {
        /// CDSTADDR[31:0]
        pub const cdstaddr_POS: u32 = 0;
        pub const cdstaddr_MASK: u32 = 0xffffffff;
    }
}
/// `DMACHANNEL` 位段常量
pub mod dmachannel {
    /// CSR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// TCIE[1:1]
        pub const tcie_POS: u32 = 1;
        pub const tcie_MASK: u32 = 0x2;
        /// TEIE[2:2]
        pub const teie_POS: u32 = 2;
        pub const teie_MASK: u32 = 0x4;
        /// TRANS[3:3]
        pub const trans_POS: u32 = 3;
        pub const trans_MASK: u32 = 0x8;
        /// SRCINC[4:4]
        pub const srcinc_POS: u32 = 4;
        pub const srcinc_MASK: u32 = 0x10;
        /// DSTINC[5:5]
        pub const dstinc_POS: u32 = 5;
        pub const dstinc_MASK: u32 = 0x20;
        /// SIZE[7:6]
        pub const size_POS: u32 = 6;
        pub const size_MASK: u32 = 0xc0;
        /// STATUS[10:8]
        pub const status_POS: u32 = 8;
        pub const status_MASK: u32 = 0x700;
        /// RESTART[11:11]
        pub const restart_POS: u32 = 11;
        pub const restart_MASK: u32 = 0x800;
        /// TC[12:12]
        pub const tc_POS: u32 = 12;
        pub const tc_MASK: u32 = 0x1000;
        /// TE[13:13]
        pub const te_POS: u32 = 13;
        pub const te_MASK: u32 = 0x2000;
        /// SRCLOAD[14:14]
        pub const srcload_POS: u32 = 14;
        pub const srcload_MASK: u32 = 0x4000;
        /// DSTLOAD[15:15]
        pub const dstload_POS: u32 = 15;
        pub const dstload_MASK: u32 = 0x8000;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// CNT[15:0]
        pub const cnt_POS: u32 = 0;
        pub const cnt_MASK: u32 = 0xffff;
        /// REPEAT[19:16]
        pub const repeat_POS: u32 = 16;
        pub const repeat_MASK: u32 = 0xf0000;
    }
    /// SRCADDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod srcaddr {
        /// SRCADDR[31:0]
        pub const srcaddr_POS: u32 = 0;
        pub const srcaddr_MASK: u32 = 0xffffffff;
    }
    /// DSTADDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dstaddr {
        /// DSTADDR[31:0]
        pub const dstaddr_POS: u32 = 0;
        pub const dstaddr_MASK: u32 = 0xffffffff;
    }
    /// TRIG 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod trig {
        /// TYPE[0:0]
        pub const r#type_POS: u32 = 0;
        pub const r#type_MASK: u32 = 0x1;
        /// SOFTSRC[1:1]
        pub const softsrc_POS: u32 = 1;
        pub const softsrc_MASK: u32 = 0x2;
        /// HARDSRC[7:2]
        pub const hardsrc_POS: u32 = 2;
        pub const hardsrc_MASK: u32 = 0xfc;
    }
    /// CCNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccnt {
        /// CCNT[15:0]
        pub const ccnt_POS: u32 = 0;
        pub const ccnt_MASK: u32 = 0xffff;
        /// CREPEAT[19:16]
        pub const crepeat_POS: u32 = 16;
        pub const crepeat_MASK: u32 = 0xf0000;
    }
    /// CSRCADDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csrcaddr {
        /// CSRCADDR[31:0]
        pub const csrcaddr_POS: u32 = 0;
        pub const csrcaddr_MASK: u32 = 0xffffffff;
    }
    /// CDSTADDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cdstaddr {
        /// CDSTADDR[31:0]
        pub const cdstaddr_POS: u32 = 0;
        pub const cdstaddr_MASK: u32 = 0xffffffff;
    }
}
/// `EAU` 位段常量
pub mod eau {
    /// CSR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod csr {
        /// MODE[1:0]
        pub const mode_POS: u32 = 0;
        pub const mode_MASK: u32 = 0x3;
        /// OVR[2:2]
        pub const ovr_POS: u32 = 2;
        pub const ovr_MASK: u32 = 0x4;
        /// ZERO[3:3]
        pub const zero_POS: u32 = 3;
        pub const zero_MASK: u32 = 0x8;
        /// BUSY[4:4]
        pub const busy_POS: u32 = 4;
        pub const busy_MASK: u32 = 0x10;
    }
    /// DIVIDEND 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dividend {
        /// DIVIDEND[31:0]
        pub const dividend_POS: u32 = 0;
        pub const dividend_MASK: u32 = 0xffffffff;
    }
    /// DIVISOR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod divisor {
        /// DIVISOR[31:0]
        pub const divisor_POS: u32 = 0;
        pub const divisor_MASK: u32 = 0xffffffff;
    }
    /// QUOTIENT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod quotient {
        /// QUOTIENT[31:0]
        pub const quotient_POS: u32 = 0;
        pub const quotient_MASK: u32 = 0xffffffff;
    }
    /// REMAINDER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod remainder {
        /// REMAINDER[31:0]
        pub const remainder_POS: u32 = 0;
        pub const remainder_MASK: u32 = 0xffffffff;
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
        /// FETCH[3:3]
        pub const fetch_POS: u32 = 3;
        pub const fetch_MASK: u32 = 0x8;
        /// CACHE[4:4]
        pub const cache_POS: u32 = 4;
        pub const cache_MASK: u32 = 0x10;
        /// CACHEINVALID[5:5]
        pub const cacheinvalid_POS: u32 = 5;
        pub const cacheinvalid_MASK: u32 = 0x20;
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
        /// SDKERR[2:2]
        pub const sdkerr_POS: u32 = 2;
        pub const sdkerr_MASK: u32 = 0x4;
        /// CACHEON[3:3]
        pub const cacheon_POS: u32 = 3;
        pub const cacheon_MASK: u32 = 0x8;
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
        /// SDKERR[2:2]
        pub const sdkerr_POS: u32 = 2;
        pub const sdkerr_MASK: u32 = 0x4;
        /// CACHEON[3:3]
        pub const cacheon_POS: u32 = 3;
        pub const cacheon_MASK: u32 = 0x8;
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
        /// SDKERR[2:2]
        pub const sdkerr_POS: u32 = 2;
        pub const sdkerr_MASK: u32 = 0x4;
        /// CACHEON[3:3]
        pub const cacheon_POS: u32 = 3;
        pub const cacheon_MASK: u32 = 0x8;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
    }
    /// PDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod pdr {
        /// PIN3[3:3]
        pub const pin3_POS: u32 = 3;
        pub const pin3_MASK: u32 = 0x8;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
    }
    /// AFRH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod afrh {
        /// PIN8[2:0]
        pub const pin8_POS: u32 = 0;
        pub const pin8_MASK: u32 = 0x7;
        /// PIN9[6:4]
        pub const pin9_POS: u32 = 4;
        pub const pin9_MASK: u32 = 0x70;
        /// PIN10[10:8]
        pub const pin10_POS: u32 = 8;
        pub const pin10_MASK: u32 = 0x700;
        /// PIN11[14:12]
        pub const pin11_POS: u32 = 12;
        pub const pin11_MASK: u32 = 0x7000;
        /// PIN12[18:16]
        pub const pin12_POS: u32 = 16;
        pub const pin12_MASK: u32 = 0x70000;
        /// PIN13[22:20]
        pub const pin13_POS: u32 = 20;
        pub const pin13_MASK: u32 = 0x700000;
        /// PIN14[26:24]
        pub const pin14_POS: u32 = 24;
        pub const pin14_MASK: u32 = 0x7000000;
        /// PIN15[30:28]
        pub const pin15_POS: u32 = 28;
        pub const pin15_MASK: u32 = 0x70000000;
    }
    /// AFRL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod afrl {
        /// PIN0[2:0]
        pub const pin0_POS: u32 = 0;
        pub const pin0_MASK: u32 = 0x7;
        /// PIN1[6:4]
        pub const pin1_POS: u32 = 4;
        pub const pin1_MASK: u32 = 0x70;
        /// PIN2[10:8]
        pub const pin2_POS: u32 = 8;
        pub const pin2_MASK: u32 = 0x700;
        /// PIN3[14:12]
        pub const pin3_POS: u32 = 12;
        pub const pin3_MASK: u32 = 0x7000;
        /// PIN4[18:16]
        pub const pin4_POS: u32 = 16;
        pub const pin4_MASK: u32 = 0x70000;
        /// PIN5[22:20]
        pub const pin5_POS: u32 = 20;
        pub const pin5_MASK: u32 = 0x700000;
        /// PIN6[26:24]
        pub const pin6_POS: u32 = 24;
        pub const pin6_MASK: u32 = 0x7000000;
        /// PIN7[30:28]
        pub const pin7_POS: u32 = 28;
        pub const pin7_MASK: u32 = 0x70000000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// BRR9[9:9]
        pub const brr9_POS: u32 = 9;
        pub const brr9_MASK: u32 = 0x200;
        /// BRR10[10:10]
        pub const brr10_POS: u32 = 10;
        pub const brr10_MASK: u32 = 0x400;
        /// BRR11[11:11]
        pub const brr11_POS: u32 = 11;
        pub const brr11_MASK: u32 = 0x800;
        /// BRR12[12:12]
        pub const brr12_POS: u32 = 12;
        pub const brr12_MASK: u32 = 0x1000;
        /// BRR13[13:13]
        pub const brr13_POS: u32 = 13;
        pub const brr13_MASK: u32 = 0x2000;
        /// BRR14[14:14]
        pub const brr14_POS: u32 = 14;
        pub const brr14_MASK: u32 = 0x4000;
        /// BRR15[15:15]
        pub const brr15_POS: u32 = 15;
        pub const brr15_MASK: u32 = 0x8000;
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
        /// BSS9[9:9]
        pub const bss9_POS: u32 = 9;
        pub const bss9_MASK: u32 = 0x200;
        /// BSS10[10:10]
        pub const bss10_POS: u32 = 10;
        pub const bss10_MASK: u32 = 0x400;
        /// BSS11[11:11]
        pub const bss11_POS: u32 = 11;
        pub const bss11_MASK: u32 = 0x800;
        /// BSS12[12:12]
        pub const bss12_POS: u32 = 12;
        pub const bss12_MASK: u32 = 0x1000;
        /// BSS13[13:13]
        pub const bss13_POS: u32 = 13;
        pub const bss13_MASK: u32 = 0x2000;
        /// BSS14[14:14]
        pub const bss14_POS: u32 = 14;
        pub const bss14_MASK: u32 = 0x4000;
        /// BSS15[15:15]
        pub const bss15_POS: u32 = 15;
        pub const bss15_MASK: u32 = 0x8000;
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
        /// BRR9[25:25]
        pub const brr9_POS: u32 = 25;
        pub const brr9_MASK: u32 = 0x2000000;
        /// BRR10[26:26]
        pub const brr10_POS: u32 = 26;
        pub const brr10_MASK: u32 = 0x4000000;
        /// BRR11[27:27]
        pub const brr11_POS: u32 = 27;
        pub const brr11_MASK: u32 = 0x8000000;
        /// BRR12[28:28]
        pub const brr12_POS: u32 = 28;
        pub const brr12_MASK: u32 = 0x10000000;
        /// BRR13[29:29]
        pub const brr13_POS: u32 = 29;
        pub const brr13_MASK: u32 = 0x20000000;
        /// BRR14[30:30]
        pub const brr14_POS: u32 = 30;
        pub const brr14_MASK: u32 = 0x40000000;
        /// BRR15[31:31]
        pub const brr15_POS: u32 = 31;
        pub const brr15_MASK: u32 = 0x80000000;
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
        /// PIN9[9:9]
        pub const pin9_POS: u32 = 9;
        pub const pin9_MASK: u32 = 0x200;
        /// PIN10[10:10]
        pub const pin10_POS: u32 = 10;
        pub const pin10_MASK: u32 = 0x400;
        /// PIN11[11:11]
        pub const pin11_POS: u32 = 11;
        pub const pin11_MASK: u32 = 0x800;
        /// PIN12[12:12]
        pub const pin12_POS: u32 = 12;
        pub const pin12_MASK: u32 = 0x1000;
        /// PIN13[13:13]
        pub const pin13_POS: u32 = 13;
        pub const pin13_MASK: u32 = 0x2000;
        /// PIN14[14:14]
        pub const pin14_POS: u32 = 14;
        pub const pin14_MASK: u32 = 0x4000;
        /// PIN15[15:15]
        pub const pin15_POS: u32 = 15;
        pub const pin15_MASK: u32 = 0x8000;
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
        /// CCDS[3:3]
        pub const ccds_POS: u32 = 3;
        pub const ccds_MASK: u32 = 0x8;
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
        /// CC2DE[10:10]
        pub const cc2de_POS: u32 = 10;
        pub const cc2de_MASK: u32 = 0x400;
        /// CC3DE[11:11]
        pub const cc3de_POS: u32 = 11;
        pub const cc3de_MASK: u32 = 0x800;
        /// CC4DE[12:12]
        pub const cc4de_POS: u32 = 12;
        pub const cc4de_MASK: u32 = 0x1000;
        /// TDE[14:14]
        pub const tde_POS: u32 = 14;
        pub const tde_MASK: u32 = 0x4000;
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
/// `HALLTIM` 位段常量
pub mod halltim {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// FLT1EN[1:1]
        pub const flt1en_POS: u32 = 1;
        pub const flt1en_MASK: u32 = 0x2;
        /// DIV[3:2]
        pub const div_POS: u32 = 2;
        pub const div_MASK: u32 = 0xc;
        /// MMS[6:4]
        pub const mms_POS: u32 = 4;
        pub const mms_MASK: u32 = 0x70;
        /// FLT2LEN[22:8]
        pub const flt2len_POS: u32 = 8;
        pub const flt2len_MASK: u32 = 0x7fff00;
        /// SOFTCAP[23:23]
        pub const softcap_POS: u32 = 23;
        pub const softcap_MASK: u32 = 0x800000;
    }
    /// DIER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod dier {
        /// CAPIE[0:0]
        pub const capie_POS: u32 = 0;
        pub const capie_MASK: u32 = 0x1;
        /// OVIE[1:1]
        pub const ovie_POS: u32 = 1;
        pub const ovie_MASK: u32 = 0x2;
        /// MATCHIE[2:2]
        pub const matchie_POS: u32 = 2;
        pub const matchie_MASK: u32 = 0x4;
        /// CAPDMA[4:4]
        pub const capdma_POS: u32 = 4;
        pub const capdma_MASK: u32 = 0x10;
        /// OVDMA[5:5]
        pub const ovdma_POS: u32 = 5;
        pub const ovdma_MASK: u32 = 0x20;
        /// MATCHDMA[6:6]
        pub const matchdma_POS: u32 = 6;
        pub const matchdma_MASK: u32 = 0x40;
        /// SOFTDMA[7:7]
        pub const softdma_POS: u32 = 7;
        pub const softdma_MASK: u32 = 0x80;
    }
    /// CNT 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cnt {
        /// DATA[23:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xffffff;
    }
    /// ARR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod arr {
        /// DATA[23:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xffffff;
    }
    /// WIDTH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod width {
        /// DATA[23:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xffffff;
    }
    /// CCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ccr {
        /// DATA[23:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xffffff;
    }
    /// ISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod isr {
        /// CAPF[0:0]
        pub const capf_POS: u32 = 0;
        pub const capf_MASK: u32 = 0x1;
        /// OVF[1:1]
        pub const ovf_POS: u32 = 1;
        pub const ovf_MASK: u32 = 0x2;
        /// MATCHF[2:2]
        pub const matchf_POS: u32 = 2;
        pub const matchf_MASK: u32 = 0x4;
    }
    /// ICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod icr {
        /// CAPF[0:0]
        pub const capf_POS: u32 = 0;
        pub const capf_MASK: u32 = 0x1;
        /// OVF[1:1]
        pub const ovf_POS: u32 = 1;
        pub const ovf_MASK: u32 = 0x2;
        /// MATCHF[2:2]
        pub const matchf_POS: u32 = 2;
        pub const matchf_MASK: u32 = 0x4;
    }
}
/// `I2C` 位段常量
pub mod i2c {
    /// INSEL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod insel {
        /// SCLINSEL[2:0]
        pub const sclinsel_POS: u32 = 0;
        pub const sclinsel_MASK: u32 = 0x7;
        /// SDAINSEL[5:3]
        pub const sdainsel_POS: u32 = 3;
        pub const sdainsel_MASK: u32 = 0x38;
    }
    /// MICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod micr {
        /// PACKET[8:8]
        pub const packet_POS: u32 = 8;
        pub const packet_MASK: u32 = 0x100;
        /// STOP[9:9]
        pub const stop_POS: u32 = 9;
        pub const stop_MASK: u32 = 0x200;
        /// NACK[10:10]
        pub const nack_POS: u32 = 10;
        pub const nack_MASK: u32 = 0x400;
        /// ARBI[11:11]
        pub const arbi_POS: u32 = 11;
        pub const arbi_MASK: u32 = 0x800;
        /// FIFO[12:12]
        pub const fifo_POS: u32 = 12;
        pub const fifo_MASK: u32 = 0x1000;
        /// PINLOW[13:13]
        pub const pinlow_POS: u32 = 13;
        pub const pinlow_MASK: u32 = 0x2000;
        /// MATCH[14:14]
        pub const r#match_POS: u32 = 14;
        pub const r#match_MASK: u32 = 0x4000;
    }
    /// MCR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mcr0 {
        /// MEN[0:0]
        pub const men_POS: u32 = 0;
        pub const men_MASK: u32 = 0x1;
        /// RESET[1:1]
        pub const reset_POS: u32 = 1;
        pub const reset_MASK: u32 = 0x2;
        /// SLEEPEN[2:2]
        pub const sleepen_POS: u32 = 2;
        pub const sleepen_MASK: u32 = 0x4;
        /// DBGEN[3:3]
        pub const dbgen_POS: u32 = 3;
        pub const dbgen_MASK: u32 = 0x8;
        /// CLKSRC[7:6]
        pub const clksrc_POS: u32 = 6;
        pub const clksrc_MASK: u32 = 0xc0;
        /// TXFIFORST[8:8]
        pub const txfiforst_POS: u32 = 8;
        pub const txfiforst_MASK: u32 = 0x100;
        /// RXFIFORST[9:9]
        pub const rxfiforst_POS: u32 = 9;
        pub const rxfiforst_MASK: u32 = 0x200;
    }
    /// MISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod misr {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// PACKET[8:8]
        pub const packet_POS: u32 = 8;
        pub const packet_MASK: u32 = 0x100;
        /// STOP[9:9]
        pub const stop_POS: u32 = 9;
        pub const stop_MASK: u32 = 0x200;
        /// NACK[10:10]
        pub const nack_POS: u32 = 10;
        pub const nack_MASK: u32 = 0x400;
        /// ARBI[11:11]
        pub const arbi_POS: u32 = 11;
        pub const arbi_MASK: u32 = 0x800;
        /// FIFO[12:12]
        pub const fifo_POS: u32 = 12;
        pub const fifo_MASK: u32 = 0x1000;
        /// PINLOW[13:13]
        pub const pinlow_POS: u32 = 13;
        pub const pinlow_MASK: u32 = 0x2000;
        /// MATCH[14:14]
        pub const r#match_POS: u32 = 14;
        pub const r#match_MASK: u32 = 0x4000;
        /// MSTBUSY[24:24]
        pub const mstbusy_POS: u32 = 24;
        pub const mstbusy_MASK: u32 = 0x1000000;
        /// BUSBUSY[25:25]
        pub const busbusy_POS: u32 = 25;
        pub const busbusy_MASK: u32 = 0x2000000;
    }
    /// MIER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mier {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// PACKET[8:8]
        pub const packet_POS: u32 = 8;
        pub const packet_MASK: u32 = 0x100;
        /// STOP[9:9]
        pub const stop_POS: u32 = 9;
        pub const stop_MASK: u32 = 0x200;
        /// NACK[10:10]
        pub const nack_POS: u32 = 10;
        pub const nack_MASK: u32 = 0x400;
        /// ARBI[11:11]
        pub const arbi_POS: u32 = 11;
        pub const arbi_MASK: u32 = 0x800;
        /// FIFO[12:12]
        pub const fifo_POS: u32 = 12;
        pub const fifo_MASK: u32 = 0x1000;
        /// PINLOW[13:13]
        pub const pinlow_POS: u32 = 13;
        pub const pinlow_MASK: u32 = 0x2000;
        /// MATCH[14:14]
        pub const r#match_POS: u32 = 14;
        pub const r#match_MASK: u32 = 0x4000;
    }
    /// MDER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mder {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
    }
    /// MCR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mcr1 {
        /// CIRFIFO[8:8]
        pub const cirfifo_POS: u32 = 8;
        pub const cirfifo_MASK: u32 = 0x100;
        /// RDMO[9:9]
        pub const rdmo_POS: u32 = 9;
        pub const rdmo_MASK: u32 = 0x200;
    }
    /// MCR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mcr2 {
        /// PRESCALE[2:0]
        pub const prescale_POS: u32 = 0;
        pub const prescale_MASK: u32 = 0x7;
        /// AUTOSTOP[8:8]
        pub const autostop_POS: u32 = 8;
        pub const autostop_MASK: u32 = 0x100;
        /// IGNACK[9:9]
        pub const ignack_POS: u32 = 9;
        pub const ignack_MASK: u32 = 0x200;
        /// TIMECFG[10:10]
        pub const timecfg_POS: u32 = 10;
        pub const timecfg_MASK: u32 = 0x400;
        /// TRGSEL[15:15]
        pub const trgsel_POS: u32 = 15;
        pub const trgsel_MASK: u32 = 0x8000;
        /// MATCFG[18:16]
        pub const matcfg_POS: u32 = 16;
        pub const matcfg_MASK: u32 = 0x70000;
        /// PINCFG[26:24]
        pub const pincfg_POS: u32 = 24;
        pub const pincfg_MASK: u32 = 0x7000000;
    }
    /// SCR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod scr2 {
        /// PINCFG[26:24]
        pub const pincfg_POS: u32 = 24;
        pub const pincfg_MASK: u32 = 0x7000000;
    }
    /// MCR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mcr3 {
        /// BUSIDLE[11:0]
        pub const busidle_POS: u32 = 0;
        pub const busidle_MASK: u32 = 0xfff;
        /// FLTSCL[19:16]
        pub const fltscl_POS: u32 = 16;
        pub const fltscl_MASK: u32 = 0xf0000;
        /// FLTSDA[27:24]
        pub const fltsda_POS: u32 = 24;
        pub const fltsda_MASK: u32 = 0xf000000;
    }
    /// MCR4 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mcr4 {
        /// PINLOW[19:8]
        pub const pinlow_POS: u32 = 8;
        pub const pinlow_MASK: u32 = 0xfff00;
    }
    /// MMATCH 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mmatch {
        /// MATCH0[7:0]
        pub const match0_POS: u32 = 0;
        pub const match0_MASK: u32 = 0xff;
        /// MATCH1[23:16]
        pub const match1_POS: u32 = 16;
        pub const match1_MASK: u32 = 0xff0000;
    }
    /// MCCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mccr {
        /// CLKLO[5:0]
        pub const clklo_POS: u32 = 0;
        pub const clklo_MASK: u32 = 0x3f;
        /// CLKHI[13:8]
        pub const clkhi_POS: u32 = 8;
        pub const clkhi_MASK: u32 = 0x3f00;
        /// SETHOLD[21:16]
        pub const sethold_POS: u32 = 16;
        pub const sethold_MASK: u32 = 0x3f0000;
        /// DATAVD[29:24]
        pub const datavd_POS: u32 = 24;
        pub const datavd_MASK: u32 = 0x3f000000;
    }
    /// MFIFOCR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mfifocr {
        /// TXWATER[1:0]
        pub const txwater_POS: u32 = 0;
        pub const txwater_MASK: u32 = 0x3;
        /// RXWATER[16:16]
        pub const rxwater_POS: u32 = 16;
        pub const rxwater_MASK: u32 = 0x10000;
    }
    /// MFIFOSR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mfifosr {
        /// TXCNT[1:0]
        pub const txcnt_POS: u32 = 0;
        pub const txcnt_MASK: u32 = 0x3;
        /// RXCNT[17:16]
        pub const rxcnt_POS: u32 = 16;
        pub const rxcnt_MASK: u32 = 0x30000;
    }
    /// MTDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mtdr {
        /// DATA[7:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xff;
        /// CMD[10:8]
        pub const cmd_POS: u32 = 8;
        pub const cmd_MASK: u32 = 0x700;
    }
    /// MRDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mrdr {
        /// DATA[7:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xff;
        /// EMPTY[14:14]
        pub const empty_POS: u32 = 14;
        pub const empty_MASK: u32 = 0x4000;
    }
    /// SICR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sicr {
        /// RESTART[8:8]
        pub const restart_POS: u32 = 8;
        pub const restart_MASK: u32 = 0x100;
        /// STOP[9:9]
        pub const stop_POS: u32 = 9;
        pub const stop_MASK: u32 = 0x200;
        /// BIT[10:10]
        pub const bit_POS: u32 = 10;
        pub const bit_MASK: u32 = 0x400;
        /// FIFO[11:11]
        pub const fifo_POS: u32 = 11;
        pub const fifo_MASK: u32 = 0x800;
    }
    /// SCR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod scr0 {
        /// SEN[0:0]
        pub const sen_POS: u32 = 0;
        pub const sen_MASK: u32 = 0x1;
        /// RESET[1:1]
        pub const reset_POS: u32 = 1;
        pub const reset_MASK: u32 = 0x2;
        /// FLTEN[4:4]
        pub const flten_POS: u32 = 4;
        pub const flten_MASK: u32 = 0x10;
        /// CLKSRC[7:6]
        pub const clksrc_POS: u32 = 6;
        pub const clksrc_MASK: u32 = 0xc0;
        /// TXFIFORST[8:8]
        pub const txfiforst_POS: u32 = 8;
        pub const txfiforst_MASK: u32 = 0x100;
        /// RXFIFORST[9:9]
        pub const rxfiforst_POS: u32 = 9;
        pub const rxfiforst_MASK: u32 = 0x200;
    }
    /// SISR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sisr {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// ADDR[2:2]
        pub const addr_POS: u32 = 2;
        pub const addr_MASK: u32 = 0x4;
        /// TACK[3:3]
        pub const tack_POS: u32 = 3;
        pub const tack_MASK: u32 = 0x8;
        /// RESTART[8:8]
        pub const restart_POS: u32 = 8;
        pub const restart_MASK: u32 = 0x100;
        /// STOP[9:9]
        pub const stop_POS: u32 = 9;
        pub const stop_MASK: u32 = 0x200;
        /// BIT[10:10]
        pub const bit_POS: u32 = 10;
        pub const bit_MASK: u32 = 0x400;
        /// FIFO[11:11]
        pub const fifo_POS: u32 = 11;
        pub const fifo_MASK: u32 = 0x800;
        /// AM0[12:12]
        pub const am0_POS: u32 = 12;
        pub const am0_MASK: u32 = 0x1000;
        /// AM1[13:13]
        pub const am1_POS: u32 = 13;
        pub const am1_MASK: u32 = 0x2000;
        /// GC[14:14]
        pub const gc_POS: u32 = 14;
        pub const gc_MASK: u32 = 0x4000;
        /// ALERT[15:15]
        pub const alert_POS: u32 = 15;
        pub const alert_MASK: u32 = 0x8000;
        /// SLVBUSY[24:24]
        pub const slvbusy_POS: u32 = 24;
        pub const slvbusy_MASK: u32 = 0x1000000;
        /// BUSBUSY[25:25]
        pub const busbusy_POS: u32 = 25;
        pub const busbusy_MASK: u32 = 0x2000000;
    }
    /// SIER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sier {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// ADDR[2:2]
        pub const addr_POS: u32 = 2;
        pub const addr_MASK: u32 = 0x4;
        /// TACK[3:3]
        pub const tack_POS: u32 = 3;
        pub const tack_MASK: u32 = 0x8;
        /// RESTART[8:8]
        pub const restart_POS: u32 = 8;
        pub const restart_MASK: u32 = 0x100;
        /// STOP[9:9]
        pub const stop_POS: u32 = 9;
        pub const stop_MASK: u32 = 0x200;
        /// BIT[10:10]
        pub const bit_POS: u32 = 10;
        pub const bit_MASK: u32 = 0x400;
        /// FIFO[11:11]
        pub const fifo_POS: u32 = 11;
        pub const fifo_MASK: u32 = 0x800;
        /// AM0[12:12]
        pub const am0_POS: u32 = 12;
        pub const am0_MASK: u32 = 0x1000;
        /// AM1[13:13]
        pub const am1_POS: u32 = 13;
        pub const am1_MASK: u32 = 0x2000;
        /// GC[14:14]
        pub const gc_POS: u32 = 14;
        pub const gc_MASK: u32 = 0x4000;
        /// ALERT[15:15]
        pub const alert_POS: u32 = 15;
        pub const alert_MASK: u32 = 0x8000;
    }
    /// SDER 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sder {
        /// TXE[0:0]
        pub const txe_POS: u32 = 0;
        pub const txe_MASK: u32 = 0x1;
        /// RXNE[1:1]
        pub const rxne_POS: u32 = 1;
        pub const rxne_MASK: u32 = 0x2;
        /// ADDR[2:2]
        pub const addr_POS: u32 = 2;
        pub const addr_MASK: u32 = 0x4;
    }
    /// SCR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod scr1 {
        /// ADRSTALL[0:0]
        pub const adrstall_POS: u32 = 0;
        pub const adrstall_MASK: u32 = 0x1;
        /// RXSTALL[1:1]
        pub const rxstall_POS: u32 = 1;
        pub const rxstall_MASK: u32 = 0x2;
        /// TXSTALL[2:2]
        pub const txstall_POS: u32 = 2;
        pub const txstall_MASK: u32 = 0x4;
        /// ACKSTALL[3:3]
        pub const ackstall_POS: u32 = 3;
        pub const ackstall_MASK: u32 = 0x8;
        /// GCEN[8:8]
        pub const gcen_POS: u32 = 8;
        pub const gcen_MASK: u32 = 0x100;
        /// ALERTEN[9:9]
        pub const alerten_POS: u32 = 9;
        pub const alerten_MASK: u32 = 0x200;
        /// TXCFG[10:10]
        pub const txcfg_POS: u32 = 10;
        pub const txcfg_MASK: u32 = 0x400;
        /// RXCFG[11:11]
        pub const rxcfg_POS: u32 = 11;
        pub const rxcfg_MASK: u32 = 0x800;
        /// IGNACK[12:12]
        pub const ignack_POS: u32 = 12;
        pub const ignack_MASK: u32 = 0x1000;
        /// TRGSEL[15:15]
        pub const trgsel_POS: u32 = 15;
        pub const trgsel_MASK: u32 = 0x8000;
        /// ADDRCFG[18:16]
        pub const addrcfg_POS: u32 = 16;
        pub const addrcfg_MASK: u32 = 0x70000;
    }
    /// SCR3 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod scr3 {
        /// CLKHOLD[3:0]
        pub const clkhold_POS: u32 = 0;
        pub const clkhold_MASK: u32 = 0xf;
        /// DATAVD[13:8]
        pub const datavd_POS: u32 = 8;
        pub const datavd_MASK: u32 = 0x3f00;
        /// FLTSCL[19:16]
        pub const fltscl_POS: u32 = 16;
        pub const fltscl_MASK: u32 = 0xf0000;
        /// FLTSDA[27:24]
        pub const fltsda_POS: u32 = 24;
        pub const fltsda_MASK: u32 = 0xf000000;
    }
    /// SAMR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod samr {
        /// ADDR0[10:1]
        pub const addr0_POS: u32 = 1;
        pub const addr0_MASK: u32 = 0x7fe;
        /// ADDR1[26:17]
        pub const addr1_POS: u32 = 17;
        pub const addr1_MASK: u32 = 0x7fe0000;
    }
    /// SASR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod sasr {
        /// RADDR[10:0]
        pub const raddr_POS: u32 = 0;
        pub const raddr_MASK: u32 = 0x7ff;
        /// ANV[14:14]
        pub const anv_POS: u32 = 14;
        pub const anv_MASK: u32 = 0x4000;
    }
    /// STAR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod star {
        /// TXNACK[0:0]
        pub const txnack_POS: u32 = 0;
        pub const txnack_MASK: u32 = 0x1;
    }
    /// STDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod stdr {
        /// DATA[7:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xff;
    }
    /// SRDR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod srdr {
        /// DATA[7:0]
        pub const data_POS: u32 = 0;
        pub const data_MASK: u32 = 0xff;
        /// RXEMPTY[14:14]
        pub const rxempty_POS: u32 = 14;
        pub const rxempty_MASK: u32 = 0x4000;
        /// SOF[15:15]
        pub const sof_POS: u32 = 15;
        pub const sof_MASK: u32 = 0x8000;
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
        /// TRIGSEL[15:12]
        pub const trigsel_POS: u32 = 12;
        pub const trigsel_MASK: u32 = 0xf000;
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
    /// CR0 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr0 {
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
    /// CR1 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr1 {
        /// CH1SRC[2:0]
        pub const ch1src_POS: u32 = 0;
        pub const ch1src_MASK: u32 = 0x7;
        /// CH2SRC[5:3]
        pub const ch2src_POS: u32 = 3;
        pub const ch2src_MASK: u32 = 0x38;
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
        /// RISE[2:2]
        pub const rise_POS: u32 = 2;
        pub const rise_MASK: u32 = 0x4;
        /// FALL[3:3]
        pub const fall_POS: u32 = 3;
        pub const fall_MASK: u32 = 0x8;
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
/// `OPA` 位段常量
pub mod opa {
    /// CR 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// MODE[2:1]
        pub const mode_POS: u32 = 1;
        pub const mode_MASK: u32 = 0x6;
        /// INP1EN[4:4]
        pub const inp1en_POS: u32 = 4;
        pub const inp1en_MASK: u32 = 0x10;
        /// INP2EN[5:5]
        pub const inp2en_POS: u32 = 5;
        pub const inp2en_MASK: u32 = 0x20;
        /// INP3EN[6:6]
        pub const inp3en_POS: u32 = 6;
        pub const inp3en_MASK: u32 = 0x40;
        /// INP4EN[7:7]
        pub const inp4en_POS: u32 = 7;
        pub const inp4en_MASK: u32 = 0x80;
        /// INN1EN[8:8]
        pub const inn1en_POS: u32 = 8;
        pub const inn1en_MASK: u32 = 0x100;
        /// INN2EN[9:9]
        pub const inn2en_POS: u32 = 9;
        pub const inn2en_MASK: u32 = 0x200;
        /// AMP[12:10]
        pub const amp_POS: u32 = 10;
        pub const amp_MASK: u32 = 0x1c00;
        /// BIAS[15:13]
        pub const bias_POS: u32 = 13;
        pub const bias_MASK: u32 = 0xe000;
    }
    /// CAL 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cal {
        /// CALEN[0:0]
        pub const calen_POS: u32 = 0;
        pub const calen_MASK: u32 = 0x1;
        /// CALMODE[1:1]
        pub const calmode_POS: u32 = 1;
        pub const calmode_MASK: u32 = 0x2;
        /// START[2:2]
        pub const start_POS: u32 = 2;
        pub const start_MASK: u32 = 0x4;
        /// SOFTTRIG[3:3]
        pub const softtrig_POS: u32 = 3;
        pub const softtrig_MASK: u32 = 0x8;
        /// CALPERIOD[7:4]
        pub const calperiod_POS: u32 = 4;
        pub const calperiod_MASK: u32 = 0xf0;
        /// AZRUN[10:10]
        pub const azrun_POS: u32 = 10;
        pub const azrun_MASK: u32 = 0x400;
        /// CLKDIV[13:11]
        pub const clkdiv_POS: u32 = 11;
        pub const clkdiv_MASK: u32 = 0x3800;
        /// ADC1START1[14:14]
        pub const adc1start1_POS: u32 = 14;
        pub const adc1start1_MASK: u32 = 0x4000;
        /// ADC2START1[15:15]
        pub const adc2start1_POS: u32 = 15;
        pub const adc2start1_MASK: u32 = 0x8000;
        /// ATIM1TRGO[16:16]
        pub const atim1trgo_POS: u32 = 16;
        pub const atim1trgo_MASK: u32 = 0x10000;
        /// ATIM1OC4[17:17]
        pub const atim1oc4_POS: u32 = 17;
        pub const atim1oc4_MASK: u32 = 0x20000;
        /// ATIM1OC5[18:18]
        pub const atim1oc5_POS: u32 = 18;
        pub const atim1oc5_MASK: u32 = 0x40000;
        /// ATIM1OC6[19:19]
        pub const atim1oc6_POS: u32 = 19;
        pub const atim1oc6_MASK: u32 = 0x80000;
        /// GTIM1TRGO[20:20]
        pub const gtim1trgo_POS: u32 = 20;
        pub const gtim1trgo_MASK: u32 = 0x100000;
        /// GTIM1OC1[21:21]
        pub const gtim1oc1_POS: u32 = 21;
        pub const gtim1oc1_MASK: u32 = 0x200000;
        /// GTIM1OC2[22:22]
        pub const gtim1oc2_POS: u32 = 22;
        pub const gtim1oc2_MASK: u32 = 0x400000;
        /// GTIM2TRGO[23:23]
        pub const gtim2trgo_POS: u32 = 23;
        pub const gtim2trgo_MASK: u32 = 0x800000;
        /// GTIM2OC1[24:24]
        pub const gtim2oc1_POS: u32 = 24;
        pub const gtim2oc1_MASK: u32 = 0x1000000;
        /// GTIM2OC2[25:25]
        pub const gtim2oc2_POS: u32 = 25;
        pub const gtim2oc2_MASK: u32 = 0x2000000;
        /// GTIM3TRGO[26:26]
        pub const gtim3trgo_POS: u32 = 26;
        pub const gtim3trgo_MASK: u32 = 0x4000000;
        /// GTIM3OC1[27:27]
        pub const gtim3oc1_POS: u32 = 27;
        pub const gtim3oc1_MASK: u32 = 0x8000000;
        /// GTIM3OC2[28:28]
        pub const gtim3oc2_POS: u32 = 28;
        pub const gtim3oc2_MASK: u32 = 0x10000000;
        /// GTIM4TRGO[29:29]
        pub const gtim4trgo_POS: u32 = 29;
        pub const gtim4trgo_MASK: u32 = 0x20000000;
        /// GTIM4OC1[30:30]
        pub const gtim4oc1_POS: u32 = 30;
        pub const gtim4oc1_MASK: u32 = 0x40000000;
        /// GTIM4OC2[31:31]
        pub const gtim4oc2_POS: u32 = 31;
        pub const gtim4oc2_MASK: u32 = 0x80000000;
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
        /// BR[30:24]
        pub const br_POS: u32 = 24;
        pub const br_MASK: u32 = 0x7f000000;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// EN[0:0]
        pub const en_POS: u32 = 0;
        pub const en_MASK: u32 = 0x1;
        /// DMARX[1:1]
        pub const dmarx_POS: u32 = 1;
        pub const dmarx_MASK: u32 = 0x2;
        /// DMATX[2:2]
        pub const dmatx_POS: u32 = 2;
        pub const dmatx_MASK: u32 = 0x4;
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
        /// HSEBRKEN[10:10]
        pub const hsebrken_POS: u32 = 10;
        pub const hsebrken_MASK: u32 = 0x400;
        /// LSEBRKEN[11:11]
        pub const lsebrken_POS: u32 = 11;
        pub const lsebrken_MASK: u32 = 0x800;
        /// CLLBRKEN[12:12]
        pub const cllbrken_POS: u32 = 12;
        pub const cllbrken_MASK: u32 = 0x1000;
        /// DSBRKEN[13:13]
        pub const dsbrken_POS: u32 = 13;
        pub const dsbrken_MASK: u32 = 0x2000;
        /// LVDBRKEN[14:14]
        pub const lvdbrken_POS: u32 = 14;
        pub const lvdbrken_MASK: u32 = 0x4000;
        /// RAMBRKEN[15:15]
        pub const rambrken_POS: u32 = 15;
        pub const rambrken_MASK: u32 = 0x8000;
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
        /// TRIM[8:0]
        pub const trim_POS: u32 = 0;
        pub const trim_MASK: u32 = 0x1ff;
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
        /// PDRIVER[11:8]
        pub const pdriver_POS: u32 = 8;
        pub const pdriver_MASK: u32 = 0xf00;
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
        /// GTIM2[2:2]
        pub const gtim2_POS: u32 = 2;
        pub const gtim2_MASK: u32 = 0x4;
        /// GTIM3[3:3]
        pub const gtim3_POS: u32 = 3;
        pub const gtim3_MASK: u32 = 0x8;
        /// GTIM4[4:4]
        pub const gtim4_POS: u32 = 4;
        pub const gtim4_MASK: u32 = 0x10;
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
        /// WWDT[10:10]
        pub const wwdt_POS: u32 = 10;
        pub const wwdt_MASK: u32 = 0x400;
        /// HALLTIM[11:11]
        pub const halltim_POS: u32 = 11;
        pub const halltim_MASK: u32 = 0x800;
    }
    /// AHBEN 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ahben {
        /// DMA[0:0]
        pub const dma_POS: u32 = 0;
        pub const dma_MASK: u32 = 0x1;
        /// FLASH[1:1]
        pub const flash_POS: u32 = 1;
        pub const flash_MASK: u32 = 0x2;
        /// CRC[2:2]
        pub const crc_POS: u32 = 2;
        pub const crc_MASK: u32 = 0x4;
        /// EAU[3:3]
        pub const eau_POS: u32 = 3;
        pub const eau_MASK: u32 = 0x8;
        /// GPIOA[4:4]
        pub const gpioa_POS: u32 = 4;
        pub const gpioa_MASK: u32 = 0x10;
        /// GPIOB[5:5]
        pub const gpiob_POS: u32 = 5;
        pub const gpiob_MASK: u32 = 0x20;
        /// GPIOC[6:6]
        pub const gpioc_POS: u32 = 6;
        pub const gpioc_MASK: u32 = 0x40;
        /// GPIOF[7:7]
        pub const gpiof_POS: u32 = 7;
        pub const gpiof_MASK: u32 = 0x80;
        /// CORDIC[8:8]
        pub const cordic_POS: u32 = 8;
        pub const cordic_MASK: u32 = 0x100;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
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
        /// WWDT[5:5]
        pub const wwdt_POS: u32 = 5;
        pub const wwdt_MASK: u32 = 0x20;
        /// I2C1[6:6]
        pub const i2c1_POS: u32 = 6;
        pub const i2c1_MASK: u32 = 0x40;
        /// LPTIM[7:7]
        pub const lptim_POS: u32 = 7;
        pub const lptim_MASK: u32 = 0x80;
        /// OPA[9:9]
        pub const opa_POS: u32 = 9;
        pub const opa_MASK: u32 = 0x200;
        /// DAC[10:10]
        pub const dac_POS: u32 = 10;
        pub const dac_MASK: u32 = 0x400;
        /// I2C2[11:11]
        pub const i2c2_POS: u32 = 11;
        pub const i2c2_MASK: u32 = 0x800;
        /// HALLTIM[12:12]
        pub const halltim_POS: u32 = 12;
        pub const halltim_MASK: u32 = 0x1000;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
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
        /// GTIM2[7:7]
        pub const gtim2_POS: u32 = 7;
        pub const gtim2_MASK: u32 = 0x80;
        /// UART3[8:8]
        pub const uart3_POS: u32 = 8;
        pub const uart3_MASK: u32 = 0x100;
        /// GTIM3[11:11]
        pub const gtim3_POS: u32 = 11;
        pub const gtim3_MASK: u32 = 0x800;
        /// GTIM4[12:12]
        pub const gtim4_POS: u32 = 12;
        pub const gtim4_MASK: u32 = 0x1000;
        /// SPI2[13:13]
        pub const spi2_POS: u32 = 13;
        pub const spi2_MASK: u32 = 0x2000;
        /// SPI3[14:14]
        pub const spi3_POS: u32 = 14;
        pub const spi3_MASK: u32 = 0x4000;
        /// KEY[31:16]
        pub const key_POS: u32 = 16;
        pub const key_MASK: u32 = 0xffff0000;
    }
    /// AHBRST 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod ahbrst {
        /// DMA[0:0]
        pub const dma_POS: u32 = 0;
        pub const dma_MASK: u32 = 0x1;
        /// FLASH[1:1]
        pub const flash_POS: u32 = 1;
        pub const flash_MASK: u32 = 0x2;
        /// CRC[2:2]
        pub const crc_POS: u32 = 2;
        pub const crc_MASK: u32 = 0x4;
        /// EAU[3:3]
        pub const eau_POS: u32 = 3;
        pub const eau_MASK: u32 = 0x8;
        /// GPIOA[4:4]
        pub const gpioa_POS: u32 = 4;
        pub const gpioa_MASK: u32 = 0x10;
        /// GPIOB[5:5]
        pub const gpiob_POS: u32 = 5;
        pub const gpiob_MASK: u32 = 0x20;
        /// GPIOC[6:6]
        pub const gpioc_POS: u32 = 6;
        pub const gpioc_MASK: u32 = 0x40;
        /// GPIOF[7:7]
        pub const gpiof_POS: u32 = 7;
        pub const gpiof_MASK: u32 = 0x80;
        /// CORDIC[8:8]
        pub const cordic_POS: u32 = 8;
        pub const cordic_MASK: u32 = 0x100;
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
        /// WWDT[5:5]
        pub const wwdt_POS: u32 = 5;
        pub const wwdt_MASK: u32 = 0x20;
        /// I2C1[6:6]
        pub const i2c1_POS: u32 = 6;
        pub const i2c1_MASK: u32 = 0x40;
        /// LPTIM[7:7]
        pub const lptim_POS: u32 = 7;
        pub const lptim_MASK: u32 = 0x80;
        /// OPA[9:9]
        pub const opa_POS: u32 = 9;
        pub const opa_MASK: u32 = 0x200;
        /// DAC[10:10]
        pub const dac_POS: u32 = 10;
        pub const dac_MASK: u32 = 0x400;
        /// I2C2[11:11]
        pub const i2c2_POS: u32 = 11;
        pub const i2c2_MASK: u32 = 0x800;
        /// HALLTIM[12:12]
        pub const halltim_POS: u32 = 12;
        pub const halltim_MASK: u32 = 0x1000;
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
        /// GTIM2[7:7]
        pub const gtim2_POS: u32 = 7;
        pub const gtim2_MASK: u32 = 0x80;
        /// UART3[8:8]
        pub const uart3_POS: u32 = 8;
        pub const uart3_MASK: u32 = 0x100;
        /// GTIM3[11:11]
        pub const gtim3_POS: u32 = 11;
        pub const gtim3_MASK: u32 = 0x800;
        /// GTIM4[12:12]
        pub const gtim4_POS: u32 = 12;
        pub const gtim4_MASK: u32 = 0x1000;
        /// SPI2[13:13]
        pub const spi2_POS: u32 = 13;
        pub const spi2_MASK: u32 = 0x2000;
        /// SPI3[14:14]
        pub const spi3_POS: u32 = 14;
        pub const spi3_MASK: u32 = 0x4000;
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
    /// MCO 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod mco {
        /// SOURCE[3:0]
        pub const source_POS: u32 = 0;
        pub const source_MASK: u32 = 0xf;
        /// DIV[7:4]
        pub const div_POS: u32 = 4;
        pub const div_MASK: u32 = 0xf0;
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
        /// PARITY[2:2]
        pub const parity_POS: u32 = 2;
        pub const parity_MASK: u32 = 0x4;
        /// PARITYEN[3:3]
        pub const parityen_POS: u32 = 3;
        pub const parityen_MASK: u32 = 0x8;
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
        /// DMARX[6:6]
        pub const dmarx_POS: u32 = 6;
        pub const dmarx_MASK: u32 = 0x40;
        /// DMATX[7:7]
        pub const dmatx_POS: u32 = 7;
        pub const dmatx_MASK: u32 = 0x80;
        /// TIMCR[10:8]
        pub const timcr_POS: u32 = 8;
        pub const timcr_MASK: u32 = 0x700;
        /// SWAP[11:11]
        pub const swap_POS: u32 = 11;
        pub const swap_MASK: u32 = 0x800;
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
        /// HYS[3:3]
        pub const hys_POS: u32 = 3;
        pub const hys_MASK: u32 = 0x8;
        /// IE[5:5]
        pub const ie_POS: u32 = 5;
        pub const ie_MASK: u32 = 0x20;
        /// POL[6:6]
        pub const pol_POS: u32 = 6;
        pub const pol_MASK: u32 = 0x40;
        /// WINDOW[7:7]
        pub const window_POS: u32 = 7;
        pub const window_MASK: u32 = 0x80;
        /// INN[9:8]
        pub const inn_POS: u32 = 8;
        pub const inn_MASK: u32 = 0x300;
        /// INP[13:12]
        pub const inp_POS: u32 = 12;
        pub const inp_MASK: u32 = 0x3000;
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
        /// BLANKTIME[10:8]
        pub const blanktime_POS: u32 = 8;
        pub const blanktime_MASK: u32 = 0x700;
        /// BLANKLVL[11:11]
        pub const blanklvl_POS: u32 = 11;
        pub const blanklvl_MASK: u32 = 0x800;
    }
    /// CR2 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod cr2 {
        /// ATIMTRGO[0:0]
        pub const atimtrgo_POS: u32 = 0;
        pub const atimtrgo_MASK: u32 = 0x1;
        /// ATIMTRGO2[1:1]
        pub const atimtrgo2_POS: u32 = 1;
        pub const atimtrgo2_MASK: u32 = 0x2;
        /// ATIMOC1REFC[2:2]
        pub const atimoc1refc_POS: u32 = 2;
        pub const atimoc1refc_MASK: u32 = 0x4;
        /// ATIMOC2REFC[3:3]
        pub const atimoc2refc_POS: u32 = 3;
        pub const atimoc2refc_MASK: u32 = 0x8;
        /// ATIMOC3REFC[4:4]
        pub const atimoc3refc_POS: u32 = 4;
        pub const atimoc3refc_MASK: u32 = 0x10;
        /// ATIMOC4REFC[5:5]
        pub const atimoc4refc_POS: u32 = 5;
        pub const atimoc4refc_MASK: u32 = 0x20;
        /// ATIMOC5REFC[6:6]
        pub const atimoc5refc_POS: u32 = 6;
        pub const atimoc5refc_MASK: u32 = 0x40;
        /// ATIMOC6REFC[7:7]
        pub const atimoc6refc_POS: u32 = 7;
        pub const atimoc6refc_MASK: u32 = 0x80;
        /// GTIM1TRGO[8:8]
        pub const gtim1trgo_POS: u32 = 8;
        pub const gtim1trgo_MASK: u32 = 0x100;
        /// GTIM1OC1REFC[9:9]
        pub const gtim1oc1refc_POS: u32 = 9;
        pub const gtim1oc1refc_MASK: u32 = 0x200;
        /// GTIM1OC2REFC[10:10]
        pub const gtim1oc2refc_POS: u32 = 10;
        pub const gtim1oc2refc_MASK: u32 = 0x400;
        /// GTIM1OC3REFC[11:11]
        pub const gtim1oc3refc_POS: u32 = 11;
        pub const gtim1oc3refc_MASK: u32 = 0x800;
        /// GTIM1OC4REFC[12:12]
        pub const gtim1oc4refc_POS: u32 = 12;
        pub const gtim1oc4refc_MASK: u32 = 0x1000;
        /// GTIM2TRGO[13:13]
        pub const gtim2trgo_POS: u32 = 13;
        pub const gtim2trgo_MASK: u32 = 0x2000;
        /// GTIM2OC1REFC[14:14]
        pub const gtim2oc1refc_POS: u32 = 14;
        pub const gtim2oc1refc_MASK: u32 = 0x4000;
        /// GTIM2OC2REFC[15:15]
        pub const gtim2oc2refc_POS: u32 = 15;
        pub const gtim2oc2refc_MASK: u32 = 0x8000;
        /// GTIM2OC3REFC[16:16]
        pub const gtim2oc3refc_POS: u32 = 16;
        pub const gtim2oc3refc_MASK: u32 = 0x10000;
        /// GTIM2OC4REFC[17:17]
        pub const gtim2oc4refc_POS: u32 = 17;
        pub const gtim2oc4refc_MASK: u32 = 0x20000;
        /// GTIM3TRGO[18:18]
        pub const gtim3trgo_POS: u32 = 18;
        pub const gtim3trgo_MASK: u32 = 0x40000;
        /// GTIM3OC1REFC[19:19]
        pub const gtim3oc1refc_POS: u32 = 19;
        pub const gtim3oc1refc_MASK: u32 = 0x80000;
        /// GTIM3OC2REFC[20:20]
        pub const gtim3oc2refc_POS: u32 = 20;
        pub const gtim3oc2refc_MASK: u32 = 0x100000;
        /// GTIM3OC3REFC[21:21]
        pub const gtim3oc3refc_POS: u32 = 21;
        pub const gtim3oc3refc_MASK: u32 = 0x200000;
        /// GTIM3OC4REFC[22:22]
        pub const gtim3oc4refc_POS: u32 = 22;
        pub const gtim3oc4refc_MASK: u32 = 0x400000;
        /// GTIM4TRGO[23:23]
        pub const gtim4trgo_POS: u32 = 23;
        pub const gtim4trgo_MASK: u32 = 0x800000;
        /// GTIM4OC1REFC[24:24]
        pub const gtim4oc1refc_POS: u32 = 24;
        pub const gtim4oc1refc_MASK: u32 = 0x1000000;
        /// GTIM4OC2REFC[25:25]
        pub const gtim4oc2refc_POS: u32 = 25;
        pub const gtim4oc2refc_MASK: u32 = 0x2000000;
        /// GTIM4OC3REFC[26:26]
        pub const gtim4oc3refc_POS: u32 = 26;
        pub const gtim4oc3refc_MASK: u32 = 0x4000000;
        /// GTIM4OC4REFC[27:27]
        pub const gtim4oc4refc_POS: u32 = 27;
        pub const gtim4oc4refc_MASK: u32 = 0x8000000;
        /// BTIM1TRGO[28:28]
        pub const btim1trgo_POS: u32 = 28;
        pub const btim1trgo_MASK: u32 = 0x10000000;
        /// BTIM2TRGO[29:29]
        pub const btim2trgo_POS: u32 = 29;
        pub const btim2trgo_MASK: u32 = 0x20000000;
        /// BTIM3TRGO[30:30]
        pub const btim3trgo_POS: u32 = 30;
        pub const btim3trgo_MASK: u32 = 0x40000000;
        /// HALLTIMTRGO[31:31]
        pub const halltimtrgo_POS: u32 = 31;
        pub const halltimtrgo_MASK: u32 = 0x80000000;
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
    /// REF 位域
    #[allow(non_upper_case_globals)] // 保留与厂商头文件一致的字段拼写
    pub mod r#ref {
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
/// 外部中断号(IRQ0..31,厂商 IRQn 枚举)
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
    /// IRQ7:GPIOC
    Gpioc = 7,
    /// IRQ8:GPIOF
    Gpiof = 8,
    /// IRQ9:DMACH12
    Dmach12 = 9,
    /// IRQ10:DMACH34
    Dmach34 = 10,
    /// IRQ11:CORDIC
    Cordic = 11,
    /// IRQ12:ADC1
    Adc1 = 12,
    /// IRQ13:ATIM
    Atim = 13,
    /// IRQ14:VC13
    Vc13 = 14,
    /// IRQ15:VC24
    Vc24 = 15,
    /// IRQ16:GTIM1
    Gtim1 = 16,
    /// IRQ17:GTIM2
    Gtim2 = 17,
    /// IRQ18:GTIM34
    Gtim34 = 18,
    /// IRQ19:LPTIM
    Lptim = 19,
    /// IRQ20:BTIM1
    Btim1 = 20,
    /// IRQ21:BTIM2
    Btim2 = 21,
    /// IRQ22:BTIM3_HALLTIM
    Btim3Halltim = 22,
    /// IRQ23:I2C1
    I2C1 = 23,
    /// IRQ24:I2C2
    I2C2 = 24,
    /// IRQ25:SPI1
    Spi1 = 25,
    /// IRQ26:SPI23
    Spi23 = 26,
    /// IRQ27:UART1
    Uart1 = 27,
    /// IRQ28:UART2
    Uart2 = 28,
    /// IRQ29:UART3
    Uart3 = 29,
    /// IRQ30:ADC2_DAC
    Adc2Dac = 30,
    /// IRQ31:CLKFAULT
    Clkfault = 31,
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
        /// IRQ7:向量符号 `GPIOC`
        GPIOC = 7;
        /// IRQ8:向量符号 `GPIOF`
        GPIOF = 8;
        /// IRQ9:向量符号 `DMACH12`
        DMACH12 = 9;
        /// IRQ10:向量符号 `DMACH34`
        DMACH34 = 10;
        /// IRQ11:向量符号 `CORDIC`
        CORDIC = 11;
        /// IRQ12:向量符号 `ADC1`
        ADC1 = 12;
        /// IRQ13:向量符号 `ATIM`
        ATIM = 13;
        /// IRQ14:向量符号 `VC13`
        VC13 = 14;
        /// IRQ15:向量符号 `VC24`
        VC24 = 15;
        /// IRQ16:向量符号 `GTIM1`
        GTIM1 = 16;
        /// IRQ17:向量符号 `GTIM2`
        GTIM2 = 17;
        /// IRQ18:向量符号 `GTIM34`
        GTIM34 = 18;
        /// IRQ19:向量符号 `LPTIM`
        LPTIM = 19;
        /// IRQ20:向量符号 `BTIM1`
        BTIM1 = 20;
        /// IRQ21:向量符号 `BTIM2`
        BTIM2 = 21;
        /// IRQ22:向量符号 `BTIM3_HALLTIM`
        BTIM3_HALLTIM = 22;
        /// IRQ23:向量符号 `I2C1`
        I2C1 = 23;
        /// IRQ24:向量符号 `I2C2`
        I2C2 = 24;
        /// IRQ25:向量符号 `SPI1`
        SPI1 = 25;
        /// IRQ26:向量符号 `SPI23`
        SPI23 = 26;
        /// IRQ27:向量符号 `UART1`
        UART1 = 27;
        /// IRQ28:向量符号 `UART2`
        UART2 = 28;
        /// IRQ29:向量符号 `UART3`
        UART3 = 29;
        /// IRQ30:向量符号 `ADC2_DAC`
        ADC2_DAC = 30;
        /// IRQ31:向量符号 `CLKFAULT`
        CLKFAULT = 31;
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
    /// ADC1
    pub adc1: &'static ADC,
    /// ATIM
    pub atim: &'static ATIM,
    /// BGR
    pub bgr: &'static BGR,
    /// BTIM1
    pub btim1: &'static BTIM,
    /// CORDIC
    pub cordic: &'static CORDIC,
    /// CRC
    pub crc: &'static CRC,
    /// DAC
    pub dac: &'static DAC,
    /// DMA
    pub dma: &'static DMA,
    /// DMACHANNEL1
    pub dmachannel1: &'static DMACHANNEL,
    /// EAU
    pub eau: &'static EAU,
    /// FLASH
    pub flash: &'static FLASH,
    /// GPIOA
    pub gpioa: &'static GPIO,
    /// GTIM1
    pub gtim1: &'static GTIM,
    /// HALLTIM
    pub halltim: &'static HALLTIM,
    /// I2C1
    pub i2_c1: &'static I2C,
    /// IRMOD
    pub irmod: &'static IRMOD,
    /// IWDT
    pub iwdt: &'static IWDT,
    /// LPTIM
    pub lptim: &'static LPTIM,
    /// LVD
    pub lvd: &'static LVD,
    /// OPA1
    pub opa1: &'static OPA,
    /// RAM
    pub ram: &'static RAM,
    /// RTC
    pub rtc: &'static RTC,
    /// SPI1
    pub spi1: &'static SPI,
    /// SYSCTRL
    pub sysctrl: &'static SYSCTRL,
    /// UART1
    pub uart1: &'static UART,
    /// VC1
    pub vc1: &'static VC,
    /// VC12REF
    pub vc12_ref: &'static VCREF,
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
            adc1: unsafe { &*(ADC1_BASE as *const ADC) },
            atim: unsafe { &*(ATIM_BASE as *const ATIM) },
            bgr: unsafe { &*(BGR_BASE as *const BGR) },
            btim1: unsafe { &*(BTIM1_BASE as *const BTIM) },
            cordic: unsafe { &*(CORDIC_BASE as *const CORDIC) },
            crc: unsafe { &*(CRC_BASE as *const CRC) },
            dac: unsafe { &*(DAC_BASE as *const DAC) },
            dma: unsafe { &*(DMA_BASE as *const DMA) },
            dmachannel1: unsafe { &*(DMACHANNEL1_BASE as *const DMACHANNEL) },
            eau: unsafe { &*(EAU_BASE as *const EAU) },
            flash: unsafe { &*(FLASH_BASE as *const FLASH) },
            gpioa: unsafe { &*(GPIOA_BASE as *const GPIO) },
            gtim1: unsafe { &*(GTIM1_BASE as *const GTIM) },
            halltim: unsafe { &*(HALLTIM_BASE as *const HALLTIM) },
            i2_c1: unsafe { &*(I2C1_BASE as *const I2C) },
            irmod: unsafe { &*(IRMOD_BASE as *const IRMOD) },
            iwdt: unsafe { &*(IWDT_BASE as *const IWDT) },
            lptim: unsafe { &*(LPTIM_BASE as *const LPTIM) },
            lvd: unsafe { &*(LVD_BASE as *const LVD) },
            opa1: unsafe { &*(OPA1_BASE as *const OPA) },
            ram: unsafe { &*(RAM_BASE as *const RAM) },
            rtc: unsafe { &*(RTC_BASE as *const RTC) },
            spi1: unsafe { &*(SPI1_BASE as *const SPI) },
            sysctrl: unsafe { &*(SYSCTRL_BASE as *const SYSCTRL) },
            uart1: unsafe { &*(UART1_BASE as *const UART) },
            vc1: unsafe { &*(VC1_BASE as *const VC) },
            vc12_ref: unsafe { &*(VC12REF_BASE as *const VCREF) },
        }
    }
}

// ================= 布局静态断言(厂商头 Size 注释) =================
const _: () = {
    assert!(core::mem::size_of::<ADC>() == 124,
        "ADC 布局错误:厂商头文件 Size = 124");
    assert!(core::mem::size_of::<ATIM>() == 116,
        "ATIM 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<BGR>() == 4,
        "BGR 布局错误:厂商头文件 Size = 4");
    assert!(core::mem::size_of::<BTIM>() == 48,
        "BTIM 布局错误:厂商头文件 Size = 48");
    assert!(core::mem::size_of::<CORDIC>() == 16,
        "CORDIC 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<CRC>() == 16,
        "CRC 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<DAC>() == 56,
        "DAC 布局错误:厂商头文件 Size = 56");
    assert!(core::mem::size_of::<DMA>() == 160,
        "DMA 布局错误:厂商头文件 Size = 160");
    assert!(core::mem::size_of::<DMACHANNEL>() == 32,
        "DMACHANNEL 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<EAU>() == 20,
        "EAU 布局错误:厂商头文件 Size = 20");
    assert!(core::mem::size_of::<FLASH>() == 116,
        "FLASH 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<GPIO>() == 100,
        "GPIO 布局错误:厂商头文件 Size = 100");
    assert!(core::mem::size_of::<GTIM>() == 116,
        "GTIM 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<HALLTIM>() == 36,
        "HALLTIM 布局错误:厂商头文件 Size = 36");
    assert!(core::mem::size_of::<I2C>() == 372,
        "I2C 布局错误:厂商头文件 Size = 372");
    assert!(core::mem::size_of::<IRMOD>() == 12,
        "IRMOD 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<IWDT>() == 40,
        "IWDT 布局错误:厂商头文件 Size = 40");
    assert!(core::mem::size_of::<LPTIM>() == 36,
        "LPTIM 布局错误:厂商头文件 Size = 36");
    assert!(core::mem::size_of::<LVD>() == 12,
        "LVD 布局错误:厂商头文件 Size = 12");
    assert!(core::mem::size_of::<OPA>() == 8,
        "OPA 布局错误:厂商头文件 Size = 8");
    assert!(core::mem::size_of::<RAM>() == 16,
        "RAM 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<RTC>() == 92,
        "RTC 布局错误:厂商头文件 Size = 92");
    assert!(core::mem::size_of::<SPI>() == 32,
        "SPI 布局错误:厂商头文件 Size = 32");
    assert!(core::mem::size_of::<SYSCTRL>() == 116,
        "SYSCTRL 布局错误:厂商头文件 Size = 116");
    assert!(core::mem::size_of::<UART>() == 64,
        "UART 布局错误:厂商头文件 Size = 64");
    assert!(core::mem::size_of::<VC>() == 16,
        "VC 布局错误:厂商头文件 Size = 16");
    assert!(core::mem::size_of::<VCREF>() == 4,
        "VCREF 布局错误:厂商头文件 Size = 4");
};