# STM32

## 芯片

买了这块（STM32F103C8T6）：[【STM32入门教程】应该是全B站最好的STM32教程了_哔哩哔哩_bilibili](https://www.bilibili.com/video/BV12v4y1y7uV)

- **F**

    表示通用性

    **L** 低功耗，**H** 高性能

- **103**

    **1** 表示内核为 **Cortex-M3**

    **03** 表示功能与性能级别，基础级别。

- **C**

    表示 64KiB 闪存

- **8**

    表示 SRAM 大小为 20KiB

- **T**

    表示 LQFP 封装

- **6** 表示标准工业级温度范围

## Rust Crate 结构

![img](./assets/crates.png)

- Micro-architecture Crate：

    对微处理器的低级访问，比如对于 Cortex-M：[cortex-m - crates.io: Rust Package Registry](https://crates.io/crates/cortex-m)

- Peripheral Access Crate：

    外围设备访问，比如对于 STM32F1：[stm32f1 - crates.io: Rust Package Registry](https://crates.io/crates/stm32f1)

再往上一层是 HAL Crate：

进一步的封装 [stm32-hal2 - crates.io: Rust Package Registry](https://crates.io/crates/stm32-hal2)

再往上就是对于特定的开发板，比如 stm32f3-discovery：[crates.io: Rust Package Registry](https://crates.io/crates/stm32f3-discovery)
