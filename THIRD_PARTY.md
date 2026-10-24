# 第三方来源声明

## fan2nd/cw32-hal2(CW32 社区项目)

- 来源:<https://github.com/fan2nd/cw32-hal2>(MIT OR Apache-2.0)
- 社区内部约定允许吸收(2026-10-24 口头授权)
- 吸收范围:
  - **`sources/approved-sdk-members/` 的厂商 SDK 头文件**(各家族
    `Libraries/inc/cw32xxxx.h`,厂商 SDK 以 Apache-2.0 授权随其仓库分发,
    由上游 SHA-256 锁定)。本生成器直接消费头文件中 svd2header 风格的
    `(@ 0x偏移)` / `Size = N` / `__IOM|__IM|__OM` 注释生成寄存器布局。
  - **`cw32-data/inputs/<family>.yaml` 的人工修正**:`register_overrides`
    (访问权限修正,如 GPIOx_ISR 只读)与 `supplemental_interrupts`
    (补官方 SVD 缺失的 IRQ,如 FAULT=31)。
  - **`cw32-data/parts.yaml`**:各家族 Flash/RAM 容量(生成 memory.x)。
- 未吸收:其 metapac 生成架构、embassy 驱动(另行按文件级吸收,见
  embassy-cw32 仓库 THIRD_PARTY.md)。

## 生成产物中的厂商数据

各 `out/<family>-pac` crate 的寄存器名/偏移/尺寸/中断号转录自上述
厂商头文件(经生成器机械转换),位段常量与中文文档注释为生成器
附加,不含厂商专有文本。

## A030 说明

CW32A030 与 CW32F030 寄存器级兼容(上游 `register-source-aliases.yaml`
记录了该映射依据),无独立 SDK 头;使用 `cw32f030-pac` 即可。
