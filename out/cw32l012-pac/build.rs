// rt feature 构建脚本:生成 device.x(PROVIDE 弱别名)+ 链接参数。
use std::env;
use std::fs;
use std::path::PathBuf;

/// (向量符号名, IRQ 号)——与 src/lib.rs 的 typelevel_irqs! 同一来源。
#[allow(unused)]
const IRQS: &[(&str, u16)] = &[
    ("WDT", 0),
    ("LVD", 1),
    ("RTC", 2),
    ("FLASHRAM", 3),
    ("SYSCTRL", 4),
    ("GPIOA", 5),
    ("GPIOB", 6),
    ("GPIOC", 7),
    ("GPIOF", 8),
    ("DMACH12", 9),
    ("DMACH34", 10),
    ("CORDIC", 11),
    ("ADC1", 12),
    ("ATIM", 13),
    ("VC13", 14),
    ("VC24", 15),
    ("GTIM1", 16),
    ("GTIM2", 17),
    ("GTIM34", 18),
    ("LPTIM", 19),
    ("BTIM1", 20),
    ("BTIM2", 21),
    ("BTIM3_HALLTIM", 22),
    ("I2C1", 23),
    ("I2C2", 24),
    ("SPI1", 25),
    ("SPI23", 26),
    ("UART1", 27),
    ("UART2", 28),
    ("UART3", 29),
    ("ADC2_DAC", 30),
    ("CLKFAULT", 31),
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let rt_enabled = env::var_os("CARGO_FEATURE_RT").is_some();
    if !rt_enabled || !target.starts_with("thumb") {
        return;
    }

    let mut device_x = String::from(
        "/* 由 build.rs 生成:未绑定的外设中断弱别名到 DefaultHandler */\n",
    );
    let mut last: i64 = -1;
    for &(name, num) in IRQS {
        assert!(num as i64 == last + 1, "IRQS 表必须按编号稠密排列");
        last = num as i64;
        device_x.push_str(&format!("PROVIDE({name} = DefaultHandler);\n"));
    }

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out.join("device.x"), device_x).unwrap();

    println!("cargo:rustc-link-search=native={}", out.display());
}
