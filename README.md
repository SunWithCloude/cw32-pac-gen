# cw32-pac-gen

从厂商 CMSIS SDK 头文件**批量生成**零依赖 PAC crate(cw32l012-pac 风格:
`Rw/Ro/Wo` 原语 + `repr(C)` 寄存器块 + 基地址 + `Interrupt` 枚举 +
typelevel 中断 + `rt` 向量表 + 布局断言 + 回归测试)。

数据源为 CW32 社区项目 [fan2nd/cw32-hal2](https://github.com/fan2nd/cw32-hal2)
锁定的厂商头文件与其人工修正层,详见 `THIRD_PARTY.md`。

## 已生成家族(全部通过:宿主机测试 + thumbv6m 交叉编译 rt 开/关)

| crate | 覆盖 | 备注 |
|-------|------|------|
| `cw32f030-pac` | CW32F030 / **CW32A030** | 22 外设块、32 IRQ;A030 寄存器级兼容共用 |
| `cw32f003-pac` | CW32F003 | |
| `cw32f020-pac` | CW32F020 | |
| `cw32l010-pac` | CW32L010 | IRQ 表有保留槽,已稠密化占位 |
| `cw32l011-pac` | CW32L011 | |
| `cw32l012-pac` | CW32L012(生成版) | **手写参考版在 [Cw32l012_pac](https://github.com/SunWithCloude/Cw32l012_pac),以手写版为准** |
| `cw32l031-pac` | CW32L031 | |
| `cw32l052-pac` | CW32L052 | |
| `cw32l083-pac` | CW32L083 | |
| `cw32r031-pac` | CW32R031 | |
| `cw32w031-pac` | CW32W031 | |

## 用法

```bash
# 前置:克隆 fan2nd/cw32-hal2 到 %TEMP%(或设 CW32_HAL2 环境变量)
python gen.py cw32f030        # 输出到 out/cw32f030-pac
```

## 生成器正确性验证

- **交叉验证**:对 CW32L012 用生成器与手写 UM V1.4 核对版对比——
  23 个共同结构中 13 个偏移布局完全一致(UART/SYSCTRL/RTC/DAC/SPI 等),
  其余为两数据源的建模差异(如 I2C 生成版含从机侧全量、BGR 页内/独立
  建模),无一处冲突性偏移。
- 每个生成 crate 自带:布局尺寸断言(厂商头 `Size =` 注释)+ 中断表
  稠密性测试。
- union 读/写双视图寄存器(CAP/CMP 同偏移)自动去重。

## 已知限制

- 位段常量(`pub mod uart { pub const ... }`)尚未生成,整寄存器
  读改写可用;
- L012 的生成版仅作交叉验证产物,正式使用手写版;
- 中断枚举变体名来自厂商 IRQn(如 `FLASHRAM`),个别与 L012 手写版
  命名风格略有差异。
