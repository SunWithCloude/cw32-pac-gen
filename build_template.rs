// rt feature 构建脚本:生成 device.x(PROVIDE 弱别名)+ 链接参数。
use std::env;
use std::fs;
use std::path::PathBuf;

/// (向量符号名, IRQ 号)——与 src/lib.rs 的 typelevel_irqs! 同一来源。
#[allow(unused)]
const IRQS: &[(&str, u16)] = &[
__IRQS__
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
