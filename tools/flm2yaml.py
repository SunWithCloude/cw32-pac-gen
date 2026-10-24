# -*- coding: utf-8 -*-
"""从 F030 DFP 的 FLM 提取烧写算法,生成 probe-rs 目标定义 CW32F030_Series.yaml

DevDscr(FlashDevice 结构)按 Open-CMSIS-Pack 规范解析:
https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/flashAlgorithm.html
字段序:Version u16 / DevName char[128] / DevType u16 / (对齐) DevAdr u32 /
szDev u32 / szPage u32 / Res u32 / ValEmpty u8 (对齐) / TOProg u32 (ms) /
TOErase u32 (ms) / 扇区表 {u32 size, u32 startAddr}... / SECTOR_END
(0xFFFFFFFF,0xFFFFFFFF)。注意:扇区条目第二项是**起始地址**而非数量;
单一尺寸类覆盖到下一条目起始或设备末尾。"""
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

# ---- DevDscr 按规范解析(校验页大小/扇区,不再依赖记忆) ----
raw = open(FLM, "rb").read()
DD_FILE_OFF, DD_SIZE = 0x2ac, 0x10a0
dd = raw[DD_FILE_OFF:DD_FILE_OFF + DD_SIZE]
import struct as _st
_szpage, = _st.unpack_from("<I", dd, 0x8C)
_valemp, = _st.unpack_from("<B", dd, 0x94)
_toprog, = _st.unpack_from("<I", dd, 0x98)
_toerase, = _st.unpack_from("<I", dd, 0x9C)
_sectors = []
_off = 0xA0
while _off + 8 <= len(dd):
    _sz, _ad = _st.unpack_from("<II", dd, _off)
    if _sz == 0xFFFFFFFF:
        break
    _sectors.append((_sz, _ad))
    _off += 8
assert _sectors and _sectors[0][0] == 0x200 and _sectors[0][1] == 0, _sectors
b64 = base64.b64encode(blob).decode()
data_off = len(blob) - data_size  # data_section_offset = PrgCode 大小

# F030 变体(parts.yaml):C8/K8/F8=64K,F6=32K;RAM 全系 8K
variants = [
    ("CW32F030C8", 0x10000), ("CW32F030K8", 0x10000),
    ("CW32F030F8", 0x10000), ("CW32F030F6", 0x8000),
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
y.append(f"    page_size: 0x{_szpage:x}")
y.append(f"    erased_byte_value: 0x{_valemp:02x}")
y.append(f"    program_page_timeout: {_toprog}")
y.append(f"    erase_sector_timeout: {_toerase}")
y.append("    sectors:")
for _sz, _ad in _sectors:
    y.append(f"    - size: 0x{_sz:x}")
    y.append(f"      address: 0x{_ad:x}")

open(OUT, "w", encoding="utf-8", newline="\n").write("\n".join(y) + "\n")
print(f"写出 {OUT}")
print(f"  指令块 {len(blob)} 字节(data @0x{data_off:x}),符号: " +
      ", ".join(f"{k}=0x{v:x}" for k, v in sorted(syms.items()) if k in ("Init","UnInit","ProgramPage","EraseSector","EraseChip")))
