# -*- coding: utf-8 -*-
"""
cw32-pac-gen:从厂商 CMSIS 头文件生成零依赖 PAC crate(cw32l012-pac 风格)。

输入:fan2nd/cw32-hal2 仓库中 sha 锁定的厂商 SDK 头文件(Apache-2.0,
sources/approved-sdk-members/),其 svd2header 风格注释提供:
  - (@ 0x...) 外设基地址 / 寄存器偏移
  - Size = N 结构体尺寸(用于编译期断言)
  - __IOM/__IM/__OM 访问语义 → Rw/Ro/Wo
  - IRQn 枚举(含 FAULT=31——官方 SVD 缺的那项,SDK 头里有)

同时吸收该仓库 inputs/<family>.yaml 的人工修正(register_overrides /
supplemental_interrupts)。

用法: python gen.py <family>   例如 cw32f030 / cw32l010 / cw32l011 ...
"""
import re
import sys
import os
import yaml

HAL2 = os.environ.get(
    "CW32_HAL2",
    r"C:/Users/Administrator/AppData/Local/Temp/cw32-hal2",
)


def snake(name: str) -> str:
    out = []
    for i, c in enumerate(name):
        if c.isupper() and i and (not name[i - 1].isupper() or (i + 1 < len(name) and name[i + 1].islower())):
            out.append("_")
        out.append(c.lower())
    s = "".join(out)
    return s[1:] if s.startswith("_") else s


def parse_header(path: str):
    """返回 (peripherals, interrupts)。
    peripherals: [{typ, instances: [(name, base)], regs: [(name, offset, access)], size}]
    interrupts: [(name, num)]
    """
    src = open(path, encoding="utf-8", errors="replace").read()

    peripherals = []
    # 段首:typedef struct /*!< (@ 0xBASE) NAME Structure
    starts = list(re.finditer(
        r"typedef struct\s*\{?\s*/\*!<\s*\(@\s*(0x[0-9A-Fa-f]+)\)\s*(\S+)\s+Structure", src))
    end_re = re.compile(r"\}\s*(\w+)_TypeDef;\s*/\*!<\s*Size\s*=\s*(\d+)")
    # 寄存器成员:__IOM/__IM/__OM uint32_t NAME; 且带 (@ 0x偏移) 注释(位域行有 " : n;" 不匹配)
    reg_re = re.compile(
        r"__(IOM|IM|OM)\s+uint32_t\s+(\w+)\s*;\s*/\*!<\s*\(@\s*(0x[0-9A-Fa-f]+)\)"
    )
    for i, m in enumerate(starts):
        base, inst_name = int(m.group(1), 16), m.group(2)
        chunk_end = starts[i + 1].start() if i + 1 < len(starts) else len(src)
        body = src[m.start():chunk_end]
        e = end_re.search(body)
        if not e:
            continue
        typ, size = e.group(1), int(e.group(2))
        regs = []
        seen_off = set()
        for acc, name, off_s in reg_re.findall(body):
            off = int(off_s, 16)
            # union 的读/写双视图(CCMRxCAP/CCMRxCMP 等)落在同一偏移:
            # 只保留首个视图,整寄存器 Rw 访问已覆盖两种语义
            if off in seen_off:
                continue
            seen_off.add(off)
            regs.append((name, off, {"IOM": "rw", "IM": "ro", "OM": "wo"}[acc]))
        peripherals.append(
            {"typ": typ, "instances": [(inst_name, base)], "regs": regs, "size": size}
        )

    # 同 Typedef 的多实例合并(如 UART1/2/3 → UART_TypeDef)
    merged = []
    for p in peripherals:
        for q in merged:
            if q["typ"] == p["typ"] and q["regs"] == p["regs"] and q["size"] == p["size"]:
                q["instances"].extend(p["instances"])
                break
        else:
            merged.append(p)

    # 中断:枚举正数项
    irq_re = re.compile(r"(\w+)_IRQn\s*=\s*(\d+)\s*,")
    interrupts = [(n, int(num)) for n, num in irq_re.findall(src) if int(num) >= 0]
    interrupts.sort(key=lambda x: x[1])
    return merged, interrupts


def load_corrections(family: str):
    p = os.path.join(HAL2, "cw32-data", "inputs", f"{family}.yaml")
    if not os.path.exists(p):
        return {}, []
    d = yaml.safe_load(open(p, encoding="utf-8"))
    overrides = {}
    for o in d.get("register_overrides", []):
        overrides[(o["block"], o["register"])] = o["access"].lower()
    supp = [(i["name"].upper(), int(i["number"])) for i in d.get("supplemental_interrupts", [])]
    return overrides, supp


def emit_struct(p, overrides):
    typ = p["typ"]
    rust_name = typ.removesuffix("_TypeDef")
    lines = [f"/// `{rust_name}` 寄存器组(生成自厂商头文件)"]
    for inst, base in p["instances"]:
        lines.append(f"///\n/// - `{inst}` 基地址 `{base:#010X}`")
    lines.append("#[allow(dead_code)]")
    lines.append("#[repr(C)]")
    lines.append(f"pub struct {rust_name} {{")
    off = 0
    pad_i = 0
    for name, reg_off, access in p["regs"]:
        if reg_off > off:
            lines.append(f"    _pad{pad_i}: [u32; {(reg_off - off) // 4}], // {off:#04x}..{reg_off:#04x}")
            pad_i += 1
        # 应用人工修正:块名匹配任一实例名或 Typedef
        acc = access
        for (block, reg), a in overrides.items():
            if reg.upper() == name.upper() and block in [i[0] for i in p["instances"]] + [rust_name.upper()]:
                acc = {"read": "ro", "write": "wo", "readwrite": "rw"}[a]
        t = {"rw": "Rw", "ro": "Ro", "wo": "Wo"}[acc]
        kw = {"match", "ref", "in", "for", "if", "else", "do", "fn", "box", "type", "use", "break", "continue"}
        fname = name.lower()
        if fname in kw:
            fname = "r#" + fname
        lines.append(f"    /// 0x{reg_off:02X} {name}({'RW' if acc=='rw' else acc.upper()})")
        lines.append(f"    pub {fname}: {t}<u32>,")
        off = reg_off + 4
    if p["size"] > off:
        lines.append(f"    _pad{pad_i}: [u32; {(p['size'] - off) // 4}], // 尾部到 Size={p['size']}")
    lines.append("}")
    return "\n".join(lines), rust_name


def gen(family: str):
    hdr = os.path.join(
        HAL2, "sources", "approved-sdk-members", "sdk-members",
    )
    # 定位头文件:目录名任意版本,内含 Libraries/inc/<family>.h
    path = None
    for root, _dirs, files in os.walk(hdr):
        if f"{family}.h" in files:
            path = os.path.join(root, f"{family}.h")
            break
    if not path:
        sys.exit(f"找不到 {family}.h")

    peripherals, interrupts = parse_header(path)
    overrides, supp = load_corrections(family)

    # 合并补充中断(SDK 头已有则跳过)
    have = {n for n, _ in interrupts}
    for n, num in supp:
        if n not in have:
            interrupts.append((n, num))
    interrupts.sort(key=lambda x: x[1])
    # 稠密化:保留槽位补 RESERVEDn 占位(向量槽存在但无中断源,
    # device.x 照常 PROVIDE 到 DefaultHandler)
    top = interrupts[-1][1]
    by_num = dict(interrupts)
    interrupts = [(by_num.get(n, f"RESERVED{n}"), n) for n in range(top + 1)]

    crate = family.replace("-", "_")
    out = os.path.join(os.path.dirname(__file__), "out", f"{family}-pac")
    os.makedirs(os.path.join(out, "src"), exist_ok=True)
    os.makedirs(os.path.join(out, "tests"), exist_ok=True)

    # ---------- lib.rs ----------
    L = []
    L.append(f"//! # {crate}-pac")
    L.append("//!")
    L.append(f"//! {family.upper()} (Cortex-M0+) 的寄存器访问层,由 cw32-pac-gen 从厂商")
    L.append("//! SDK 头文件生成(布局/偏移/尺寸取自 svd2header 注释),并吸收")
    L.append("//! fan2nd/cw32-hal2 数据层的人工修正。零依赖,只使用 core。")
    L.append("//!")
    L.append("//! 数据来源与生成方式见仓库 THIRD_PARTY.md。")
    L.append("")
    L.append("#![no_std]")
    L.append("#![allow(rustdoc::broken_intra_doc_links)]")
    L.append("#![forbid(unsafe_op_in_unsafe_fn)]")
    L.append("")

    # Rw/Ro/Wo 原语
    L.append(open(os.path.join(os.path.dirname(__file__), "primitives.rs"), encoding="utf-8").read())
    L.append("")

    # 基地址
    L.append("// ================= 外设基地址(厂商头文件注释) =================")
    L.append("")
    struct_names = []
    periph_fields = []
    for p in peripherals:
        body, rust_name = emit_struct(p, overrides)
        for inst, base in p["instances"]:
            L.append(f"/// {inst}({rust_name})基地址")
            L.append(f"pub const {inst}_BASE: usize = {base:#010x};")
            periph_fields.append((snake(inst), rust_name, inst))
        struct_names.append((body, rust_name, p))
    L.append("")
    L.append("// ================= 寄存器块 =================")
    L.append("")
    for body, _, _ in struct_names:
        L.append(body)
        L.append("")

    # 中断枚举
    L.append("// ================= 中断号 =================")
    L.append("/// 外部中断号(IRQ0..%d,厂商 IRQn 枚举)" % (interrupts[-1][1] if interrupts else 0))
    L.append("#[repr(u8)]")
    L.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    L.append("pub enum Interrupt {")
    for n, num in interrupts:
        L.append(f"    /// IRQ{num}:{n}")
        L.append(f"    {n.title().replace('_','')} = {num},")
    L.append("}")
    L.append("")
    L.append("impl Interrupt {")
    L.append("    /// 在 NVIC 中使能该中断")
    L.append("    pub fn enable(self) {")
    L.append("        let irq = self as u32;")
    L.append("        unsafe { core::ptr::write_volatile(0xE000_E100 as *mut u32, 1 << irq); }")
    L.append("    }")
    L.append("    /// 在 NVIC 中禁用该中断")
    L.append("    pub fn disable(self) {")
    L.append("        let irq = self as u32;")
    L.append("        unsafe { core::ptr::write_volatile(0xE000_E180 as *mut u32, 1 << irq); }")
    L.append("    }")
    L.append("    /// 置为挂起")
    L.append("    pub fn pend(self) {")
    L.append("        let irq = self as u32;")
    L.append("        unsafe { core::ptr::write_volatile(0xE000_E200 as *mut u32, 1 << irq); }")
    L.append("    }")
    L.append("    /// 清除挂起")
    L.append("    pub fn unpend(self) {")
    L.append("        let irq = self as u32;")
    L.append("        unsafe { core::ptr::write_volatile(0xE000_E280 as *mut u32, 1 << irq); }")
    L.append("    }")
    L.append("}")
    L.append("")

    # typelevel 中断 + SYMBOLS + rt 向量表(与 cw32l012-pac 0.2.1 同构,单源)
    L.append(open(os.path.join(os.path.dirname(__file__), "interrupt_tl.rs"), encoding="utf-8").read()
             .replace("__IRQ_TABLE__", "\n".join(
                 f"        /// IRQ{n}:向量符号 `{sym}`\n        {sym} = {n};" for sym, n in interrupts)))
    L.append("")

    # Peripherals
    L.append("// ================= 外设单例 =================")
    L.append("/// 芯片外设单例。[`Peripherals::take`] 只能成功一次。")
    L.append("pub struct Peripherals {")
    for field, rust_name, inst in periph_fields:
        L.append(f"    /// {inst}")
        L.append(f"    pub {field}: &'static {rust_name},")
    L.append("}")
    L.append("")
    L.append("impl Peripherals {")
    L.append("    /// 获取外设单例;第二次及以后调用返回 `None`。")
    L.append("    pub fn take() -> Option<Self> {")
    L.append("        static mut TAKEN: bool = false;")
    L.append("        // SAFETY: 单核裸机环境;约定首次调用发生在使能中断之前。")
    L.append("        unsafe {")
    L.append("            if TAKEN { None } else { TAKEN = true; Some(Self::steal()) }")
    L.append("        }")
    L.append("    }")
    L.append("")
    L.append("    /// 无视单例直接取得外设引用。")
    L.append("    ///")
    L.append("    /// # Safety")
    L.append("    /// 调用方需自行保证不出现别名所有权冲突。")
    L.append("    pub unsafe fn steal() -> Self {")
    L.append("        Peripherals {")
    for field, rust_name, inst in periph_fields:
        L.append(f"            {field}: unsafe {{ &*({inst}_BASE as *const {rust_name}) }},")
    L.append("        }")
    L.append("    }")
    L.append("}")
    L.append("")

    # 布局断言
    L.append("// ================= 布局静态断言(厂商头 Size 注释) =================")
    L.append("const _: () = {")
    for _, rust_name, p in struct_names:
        L.append(f"    assert!(core::mem::size_of::<{rust_name}>() == {p['size']},")
        L.append(f"        \"{rust_name} 布局错误:厂商头文件 Size = {p['size']}\");")
    L.append("};")
    open(os.path.join(out, "src", "lib.rs"), "w", encoding="utf-8", newline="\n").write("\n".join(L))

    # ---------- tests/layout.rs ----------
    T = ["//! 生成产物回归:布局尺寸与中断表密度(生成时自动核对)。", "",
         f"use {crate}_pac::*;", "",
         "#[test]", "fn block_sizes_match_vendor_headers() {"]
    for _, rust_name, p in struct_names:
        T.append(f"    assert_eq!(core::mem::size_of::<{rust_name}>(), {p['size']}, \"{rust_name}\");")
    T.append("}")
    T.append("")
    T.append("#[test]")
    T.append("fn interrupt_table_dense_and_ordered() {")
    T.append(f"    let all: [(Interrupt, u32); {len(interrupts)}] = [")
    for n, num in interrupts:
        T.append(f"        (Interrupt::{n.title().replace('_','')} , {num}),")
    T.append("    ];")
    T.append("    for (i, (_, n)) in all.iter().enumerate() {")
    T.append("        assert_eq!(*n as usize, i, \"IRQ 表必须按 0..n 稠密排列\");")
    T.append("    }")
    T.append("    assert_eq!(all.len(), 1 + all.last().unwrap().1 as usize);")
    T.append("}")
    open(os.path.join(out, "tests", "layout.rs"), "w", encoding="utf-8", newline="\n").write("\n".join(T))

    # ---------- memory.x(取该家族最大容量) ----------
    parts = yaml.safe_load(open(os.path.join(HAL2, "cw32-data", "parts.yaml"), encoding="utf-8"))["parts"]
    fam = family.upper()
    fam_parts = [p for p in parts if p.get("family", "").upper() == fam or p["name"].upper().startswith(fam)]
    flash = max((m["size"] for p in fam_parts for m in p["memory"] if m["kind"] == "flash"), default=65536)
    ram = max((m["size"] for p in fam_parts for m in p["memory"] if m["kind"] == "ram"), default=8192)
    open(os.path.join(out, "memory.x"), "w", encoding="utf-8", newline="\n").write(
        f"/* {fam} 存储器布局(家族最大配置:Flash {flash//1024}K / SRAM {ram//1024}K) */\n"
        "MEMORY\n{\n"
        f"  FLASH : ORIGIN = 0x00000000, LENGTH = {flash//1024}K\n"
        f"  RAM   : ORIGIN = 0x20000000, LENGTH = {ram//1024}K\n"
        "}\n\n_stack_start = ORIGIN(RAM) + LENGTH(RAM);\n")

    # ---------- build.rs / Cargo.toml ----------
    open(os.path.join(out, "build.rs"), "w", encoding="utf-8", newline="\n").write(
        open(os.path.join(os.path.dirname(__file__), "build_template.rs"), encoding="utf-8").read()
        .replace("__IRQS__", "\n".join(
            f'    ("{sym}", {n}),' for sym, n in interrupts)))
    open(os.path.join(out, "Cargo.toml"), "w", encoding="utf-8", newline="\n").write(f"""[package]
name = "{family}-pac"
version = "0.1.0"
edition = "2021"
rust-version = "1.60"
description = "Zero-dependency register access crate (PAC) for the {fam} Cortex-M0+ MCU, generated by cw32-pac-gen from the vendor SDK headers (Apache-2.0)"
categories = ["embedded", "hardware-support", "no-std"]
keywords = ["cw32", "{family}", "pac", "register", "cortex-m"]
license = "MIT OR Apache-2.0"
readme = "README.md"

[dependencies]
cortex-m-rt = {{ version = "0.7", optional = true, features = ["device"] }}

[features]
rt = ["dep:cortex-m-rt"]
""")

    print(f"生成 {out}")
    print(f"  外设块 {len(struct_names)} 个,实例 {len(periph_fields)} 个,中断 {len(interrupts)} 项")
    return out


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("用法: python gen.py <family>  (如 cw32f030)")
    gen(sys.argv[1].lower())
