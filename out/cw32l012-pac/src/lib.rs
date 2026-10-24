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