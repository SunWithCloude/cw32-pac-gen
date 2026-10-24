# -*- coding: utf-8 -*-
"""从 F030 DFP 的 FLM 提取烧写算法,生成 probe-rs 目标定义 CW32F030_Series.yaml"""
import base64
import struct
import subprocess
import sys

FLM = r"C:/Users/Administrator/AppData/Local/Temp/f030pack/Flash/FlashCW32F030.FLM"
OUT = r"C:/Users/Administrator/AppData/Local/Temp/CW32F030_Series.yaml"

# ---- ELF 解析(32 位小端):取 PrgCode/PrgData 段与符号地址 ----
def parse_elf(path):
    data = open(path, "rb").read()
    assert data[:4] == b"\x7fELF"
    e_shoff, = struct.unpack_from("<I", data, 0x20)
    e_shentsize, e_shnum, e_shstrndx = struct.unpack_from("<HHH", data, 0x2E)
    shs = []
    for i in range(e_shnum):
        off = e_shoff + i * e_shentsize
        name_off, _typ, _flags, _addr, offset, size = struct.unpack_from("<IIIIII", data, off)
        shs.append((name_off, offset, size))
    strtab_off = shs[e_shstrndx][1]
    sections = {}
    for name_off, offset, size in shs:
        end = data.index(b"\0", strtab_off + name_off)
        name = data[strtab_off + name_off:end].decode()
        sections[name] = (offset, size)
    prg_code = data[sections["PrgCode"][0]:sections["PrgCode"][0] + sections["PrgCode"][1]]
    prg_data = data[sections["PrgData"][0]:sections["PrgData"][0] + sections["PrgData"][1]]
    # 符号:nm 输出
    syms = {}
    out = subprocess.run(["nm", path], capture_output=True, text=True).stdout
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[1] in "TtDd":
            syms[parts[2]] = int(parts[0], 16)
    return prg_code + prg_data, sections["PrgData"][1], syms  # data 段的 VMA==PrgCode 大小

blob, data_size, syms = parse_elf(FLM)
b64 = base64.b64encode(blob).decode()
data_off = len(blob) - data_size  # data_section_offset = PrgCode 大小

# F030 变体(parts.yaml):C8/K8/F8=64K,F6=32K;RAM 全系 8K
variants = [
    ("CW32F030C8", 0x10000), ("CW32F030K8", 0x10000),
    ("CW32F030F8", 0x10000), ("CW32F030F6", 0x8000),
    ("CW32A030C8", 0x10000),  # A030 寄存器/Flash 同 F030C8
]

y = []
y.append("name: CW32F030 Series")
y.append("generated_from_pack: true")
y.append("pack_file_release: 1.0.5")
y.append("variants:")
for name, flash_end in variants:
    y.append(f"- name: {name}")
    y.append("  cores:")
    y.append("  - name: main")
    y.append("    type: armv6m")
    y.append("    core_access_options: !Arm")
    y.append("      ap: !v1 0")
    y.append("  memory_map:")
    y.append("  - !Nvm")
    y.append("    name: IROM1")
    y.append("    range:")
    y.append("      start: 0x0")
    y.append(f"      end: 0x{flash_end:x}")
    y.append("    cores:")
    y.append("    - main")
    y.append("    access:")
    y.append("      write: false")
    y.append("      boot: true")
    y.append("  - !Ram")
    y.append("    name: IRAM1")
    y.append("    range:")
    y.append("      start: 0x20000000")
    y.append("      end: 0x20002000")
    y.append("    cores:")
    y.append("    - main")
    y.append("  flash_algorithms:")
    y.append("  - flashcw32f030")
y.append("flash_algorithms:")
y.append("- name: flashcw32f030")
y.append("  description: CW32F030 (from WHXY.CW32F030_DFP 1.0.5 FlashCW32F030.FLM)")
y.append("  default: true")
y.append(f"  instructions: {b64}")
y.append(f"  pc_init: 0x{syms['Init']:x}")
y.append(f"  pc_uninit: 0x{syms['UnInit']:x}")
y.append(f"  pc_program_page: 0x{syms['ProgramPage']:x}")
y.append(f"  pc_erase_sector: 0x{syms['EraseSector']:x}")
y.append(f"  pc_erase_all: 0x{syms['EraseChip']:x}")
y.append(f"  data_section_offset: 0x{data_off:x}")
y.append("  flash_properties:")
y.append("    address_range:")
y.append("      start: 0x0")
y.append("      end: 0x10000")
y.append("    page_size: 0x200")
y.append("    erased_byte_value: 0xff")
y.append("    program_page_timeout: 100")
y.append("    erase_sector_timeout: 3000")
y.append("    sectors:")
y.append("    - size: 0x200")
y.append("      address: 0x0")

open(OUT, "w", encoding="utf-8", newline="\n").write("\n".join(y) + "\n")
print(f"写出 {OUT}")
print(f"  指令块 {len(blob)} 字节(data @0x{data_off:x}),符号: " +
      ", ".join(f"{k}=0x{v:x}" for k, v in sorted(syms.items()) if k in ("Init","UnInit","ProgramPage","EraseSector","EraseChip")))
