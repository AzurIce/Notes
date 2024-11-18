# STM32

## 目录

### [c1-cortex-m-quickstart](c1-cortex-m-quickstart/README.md)

**PART1**：`p1-hello`：基于 `cortex_m` 这个 **Micro-architecture Crate** 编写的在微控制器上运行的 Hello World。（运行这个 `example` 需要注释掉 `Cargo.toml` 中的 `stm32f1` 和 `stm32f1xx-hal` 库）

**PART2**：`p2-cortex-syst`：基于 `cortex_m` 这个 **Micro-architecture Crate** 访问 *Cortex-M* 的 *SYST* 外围设备来实现一个 1 秒的等待

前面提到了 *Cortex-M* 处理器也自带一些外围设备，这个 *SYST* 就是个例子。

然而只使用 Micro-architecture 库，只能做到执行普通的 Rust 代码并在 RAM 内移动数据。

如果想要做到信息的输入输出（比如闪烁一个 LED、检测按钮按下等）就需要访问 **外围设备** 以及他们的 Memory Mapped Registers。

<img src="./assets/crates.png" alt="img" style="zoom: 80%;" />

底层的库：

- Micro-architecture Crate，如 [cortex-m - crates.io: Rust Package Registry](https://crates.io/crates/cortex-m)：

    可以启用/禁用处理器的中断、访问 SysTick 外围设备等（都是 Cortex-M 架构处理器具有的功能）

- Peripheral Access Crate，如 [stm32f1 - crates.io: Rust Package Registry](https://crates.io/crates/stm32f1)：

    外围设备访问，可以直接访问到对应的寄存器。

**PART3**：`p3-pac`：使用 *stm32f1* 这个 **Peripheral Access Crate** 来点亮开发板的 LED

这就是在 STM32  中 *Cortex-M* 处理器以外的外围设备了。

通过 RCC（Reset and Clock Control）启用 GPIOA，然后通过 GPIOA 的 CR（Configuration Register）来设置针脚模式（输出）以及配置（推挽输出），最后通过针脚的 BSRR（Bit Set/Reset Register）来设置针脚的输出位为 `1` 点亮 LED。

然后用 **PART2** 的等待方式，配上设置输出位为 `0` 和 `1` 就可以实现 1Hz 的 LED 闪烁。

**PART4**：`p4-hal`：使用 *stm32f1xx-hal* 这个 HAL Crate 来重写 **PART3**。

### [c2-tutorial](c2-tutorial/README.md)



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
