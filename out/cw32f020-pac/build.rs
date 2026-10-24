// rt feature 构建脚本:生成 device.x(PROVIDE 弱别名)+ 链接参数。
use std::env;
use std::fs;
use std::path::PathBuf;

/// (向量符号名, IRQ 号)——与 src/lib.rs 的 typelevel_irqs! 同一来源。
#[allow(unused)]
const IRQS: &[(&str, u16)] = &[
    ("RESERVED0", 0),
    ("RESERVED1", 1),
    ("RESERVED2", 2),
    ("RESERVED3", 3),
    ("RESERVED4", 4),
    ("RESERVED5", 5),
    ("RESERVED6", 6),
    ("RESERVED7", 7),
    ("RESERVED8", 8),
    ("RESERVED9", 9),
    ("RESERVED10", 10),
    ("RESERVED11", 11),
    ("RESERVED12", 12),
    ("RESERVED13", 13),
    ("RESERVED14", 14),
    ("RESERVED15", 15),
    ("RESERVED16", 16),
    ("RESERVED17", 17),
    ("RESERVED18", 18),
    ("RESERVED19", 19),
    ("RESERVED20", 20),
    ("RESERVED21", 21),
    ("RESERVED22", 22),
    ("RESERVED23", 23),
    ("RESERVED24", 24),
    ("RESERVED25", 25),
    ("RESERVED26", 26),
    ("RESERVED27", 27),
    ("RESERVED28", 28),
    ("RESERVED29", 29),
    ("RESERVED30", 30),
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
