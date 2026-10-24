# [Draft] probe-rs PR: Add CW32F030 target support

> 提交处:fork [probe-rs/probe-rs](https://github.com/probe-rs/probe-rs),
> 在 `probe-rs/targets/` 添加 `CW32F030_Series.yaml`(本仓库 tools/ 下已就绪),
> 然后开 PR。以下为英文正文草稿。

---

**Title:** Add CW32F030 target support

**Body:**

Adds the WHXY (CMSemicon / Wuhan Xinyuan) CW32F030 family — Cortex-M0+,
64 MHz, same vendor as the CW32L0 series merged in #3682, but the "classic"
peripheral generation with a different memory map (e.g. GPIOC @ 0x4800_0800)
and its own flash controller.

- Source: vendor DFP `WHXY.CW32F030_DFP` 1.0.5 (four devices: CW32F030C8 /
  K8 / F8 with 64 KiB flash, CW32F030F6 with 32 KiB; all 8 KiB SRAM).
- The flash algorithm is converted from the pack's `FlashCW32F030.FLM`.
  Its DevDscr was parsed per the Open-CMSIS-Pack flash-algorithm spec
  (page-size 512 B, erased value 0xFF, program-page timeout 100 ms,
  erase-sector timeout 3000 ms, one uniform 512 B sector class covering
  the whole device, `SECTOR_END`-terminated).
- Hardware-verified on a CW32F030C8T7 board over ST-Link V2 / SWD:
  `download` (0.5 s), `verify` ("Verification successful"), full-chip
  `erase` + blank readback (0xFFFFFFFF), re-program, `reset`, and a GPIO
  blink confirming the core runs the flashed image (vector table readback
  SP=0x2000_2000 / reset=0xC1).
- File structure follows the existing `CW32L0_Series.yaml` conventions.

Happy to adjust naming/format as maintainers prefer. I can prepare the
remaining CW32 families (F003/F020/A030/L010-L083/R031/W031) the same way
if this one looks good — the conversion is scripted.

---

**中文备注(不进 PR):**
- 我们仓库 `tools/CW32F030_Series.yaml` 即待提交文件;
- flm2yaml.py 已按规范重写,可批量复制到其他家族(需要各家族 DFP);
- 提交前把 yaml 里 `generated_from_pack: true` 保留(确实源自 pack)。
