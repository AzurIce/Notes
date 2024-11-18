# c1-cortex-m-quickstart

## 一、p1-hello

基于 *cortex-m* 库的 Hello World，以及基础调试操作。

### 1. 创建项目

首先用 `cargo-generate` 创建一个模板项目：

```
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

创建完成后除了 `Cargo.toml` 之外还有有几个主要的文件：

- `.cargo/config.toml`：包含一些有关构建的设置（比如目标架构等）
- `memory.x`：内存布局文件，会被 `build.rs` 使用，最终会被用于控制写入设备
- `openocd.cfg`、`openocd.gdb`：*openocd* 相关

修改 `cargo/config.toml`，修改 target 为 STM32F1 对应的 Cortex-M3 的架构，并且安装对应的编译目标：

```toml
[build]
target = "thumbv7m-none-eabi"        # Cortex-M3
```

```
rustup target add thumbv7m-none-eabi
```

根据芯片的内存布局修改 `memory.x`，（参考 [assets/stm32f103c8.pdf](assets/stm32f103c8.pdf) 中的 *4 Memory mapping*）调整 FLASH 和 RAM 的位置和大小：

```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
```

然后就是修改一下 `main.rs`，打印个 `Hello, world!`：

```rust
#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    loop {
        // your code goes here
    }
}

```

然后就可以构建了：

```
cargo build
```

### 2. Openocd

![image-20241118030321300](./assets/image-20241118030321300.png)

```
PS > openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
Open On-Chip Debugger 0.12.0 (2023-01-14-23:37)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD      
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : clock speed 1000 kHz
Info : STLINK V2J37S7 (API v2) VID:PID 0483:3748
Info : Target voltage: 3.175000
Info : [stm32f1x.cpu] Cortex-M3 r1p1 processor detected
Info : [stm32f1x.cpu] target has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f1x.cpu on 3333
Info : Listening on port 3333 for gdb connections
```

修改 `openocd.cfg`：

```
source [find interface/stlink.cfg]
source [find target/stm32f1x.cfg]
```

之后就可以直接：

```
PS > openocd
Open On-Chip Debugger 0.12.0 (2023-01-14-23:37)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : clock speed 1000 kHz
Info : STLINK V2J37S7 (API v2) VID:PID 0483:3748
Info : Target voltage: 3.154688
Info : [stm32f1x.cpu] Cortex-M3 r1p1 processor detected
Info : [stm32f1x.cpu] target has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f1x.cpu on 3333
Info : Listening on port 3333 for gdb connections
```

### 3. arm-none-eabi-gdb

可以通过安装 gcc-arm-none-eabi 来安装。

```
arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/p1-cortex-m-quickstart
```

> ```
> GNU gdb (Arm GNU Toolchain 13.3.Rel1 (Build arm-13.24)) 14.2.90.20240526-git
> Copyright (C) 2023 Free Software Foundation, Inc.
> License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
> This is free software: you are free to change and redistribute it.
> There is NO WARRANTY, to the extent permitted by law.
> Type "show copying" and "show warranty" for details.
> This GDB was configured as "--host=i686-w64-mingw32 --target=arm-none-eabi".
> Type "show configuration" for configuration details.
> For bug reporting instructions, please see:
> <https://bugs.linaro.org/>.
> Find the GDB manual and other documentation resources online at:
>     <http://www.gnu.org/software/gdb/documentation/>.
> 
> For help, type "help".
> Type "apropos word" to search for commands related to "word"...
> Reading symbols from .\target\thumbv7m-none-eabi\debug\p1-cortex-m-quickstart...
> 
> warning: could not convert 'main' from the host encoding (CP1252) to UTF-32.
> This normally should not happen, please file a bug report.
> (gdb) 
> ```

连接到 OpenOCD：

```
(gdb) target remote :3333
Remote debugging using :3333
0x08003bd2 in ?? ()
```

> 对应的 openocd 产生输出：
>
> ```
> Info : accepting 'gdb' connection on tcp/3333
> Info : device id = 0x20036410
> Info : flash size = 64 KiB
> Warn : Prefer GDB command "target extended-remote :3333" instead of "target remote :3333"
> ```

加载程序到微控制器：

```
(gdb) load
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0xa24 lma 0x8000400
Loading section .rodata, size 0x624 lma 0x8000e24
Start address 0x08000400, load size 5192
Transfer rate: 9 KB/sec, 1730 bytes/write.
```

This program uses semihosting so before we do any semihosting call we have to tell OpenOCD to enable semihosting. You can send commands to OpenOCD using the `monitor` command.

```
(gdb) monitor arm semihosting enable
semihosting is enabled
```

可以设置到 `main` 的断点，并用 `continue` 直接运行到那里：

```
(gdb) break main
Breakpoint 1 at 0x8000440: file src/main.rs, line 13.
Note: automatically using hardware breakpoints for read-only addresses.
(gdb) continue
Continuing.

Breakpoint 1, p1_cortex_m_quickstart::__cortex_m_rt_main_trampoline () at src/main.rs:13
13      #[entry]
```

用 `step` 可以进入到 `main` 函数：

```
(gdb) step
halted: PC: 0x08000444
p1_cortex_m_quickstart::__cortex_m_rt_main () at src/main.rs:15
15          hprintln!("Hello, world!");
```

最后，用 `next` 来恢复程序的运行，应该可以看到 openocd 打出了 `Hello, world!`：

```
(gdb) next
halted: PC: 0x0800044c
halted: PC: 0x08000450
halted: PC: 0x08000452
halted: PC: 0x0800099e
halted: PC: 0x08000458
17          loop {
```

> ```
> Info : halted: PC: 0x0800044c
> Info : halted: PC: 0x08000450
> Info : halted: PC: 0x08000452
> Info : halted: PC: 0x0800099e
> Hello, world!
> Info : halted: PC: 0x08000458
> ```

最后可以通过 `quit` 来退出 GDB：

```
(gdb) quit
A debugging session is active.

        Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n) y
Detaching from program: F:\Notes\STM32\learn-stm32\p1-cortex-m-quickstart\target\thumbv7m-none-eabi\debug\p1-cortex-m-quickstart, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```

### 4. openocd.gdb

上面手动敲 gdb 命令太繁琐了，所以都xie'zai了 `openocd.gdb` 里（像是个脚本：

```
target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind
# # run the next few lines so the panic message is printed immediately
# # the number needs to be adjusted for your panic handler
# commands $bpnum
# next 4
# end

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

monitor arm semihosting enable

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
# monitor tpiu config internal itm.txt uart off 8000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 8000000 must match the core clock frequency
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# # enable ITM port 0
# monitor itm port 0 on

load

# start the process but immediately halt the processor
stepi

```

然后就可以通过简单的一行命令连接并加载程序启动进程：

```
arm-none-eabi-gdb -x openocd.gdb target/thumbv7m-none-eabi/debug/p1-cortex-m-quickstart
```

此外可以通过调整 `.config/config.toml` 来用 `cargo run` 来执行：

```toml
[target.thumbv7m-none-eabi]
# uncomment this to make `cargo run` execute programs on QEMU
# runner = "qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel"

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# uncomment ONE of these three option to make `cargo run` start a GDB session
# which option to pick depends on your system
runner = "arm-none-eabi-gdb -x openocd.gdb"
# runner = "gdb-multiarch -x openocd.gdb"
# runner = "gdb -x openocd.gdb"
```

```terminal
PS > cargo run
// ...
warning: `p1-cortex-m-quickstart` (bin "p1-cortex-m-quickstart") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `arm-none-eabi-gdb -q -x openocd.gdb target\thumbv7m-none-eabi\debug\p1-cortex-m-quickstart`
Reading symbols from target\thumbv7m-none-eabi\debug\p1-cortex-m-quickstart...

warning: could not convert 'main' from the host encoding (CP1252) to UTF-32.
This normally should not happen, please file a bug report.
p1_cortex_m_quickstart::__cortex_m_rt_main () at src/main.rs:17
17          loop {
Breakpoint 1 at 0x80006fa: file src/lib.rs, line 570.
Note: automatically using hardware breakpoints for read-only addresses.
Breakpoint 2 at 0x8000e10: file src/lib.rs, line 560.
Breakpoint 3 at 0x8000780: file src/lib.rs, line 32.
Breakpoint 4 at 0x8000440: file src/main.rs, line 13.
semihosting is enabled
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0xa24 lma 0x8000400
Loading section .rodata, size 0x624 lma 0x8000e24
Start address 0x08000400, load size 5192
Transfer rate: 9 KB/sec, 1730 bytes/write.
halted: PC: 0x08000402
0x08000402 in cortex_m_rt::Reset () at src/lib.rs:497
497     pub unsafe extern "C" fn Reset() -> ! {
(gdb) 
```

## 二、p2-cortex-syst

通过 *cortex_m* 的 syst 外围设备用 `while` 来模拟了一个 1 秒的等待：

```rust
//! Implement a delay of 1 second using systick and while loop

#![no_main]
#![no_std]

use cortex_m::{peripheral::syst::SystClkSource, Peripherals};
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let peripherals = Peripherals::take().unwrap();
    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // 8 MHz according to 2.3.7 Clocks and startup of the datasheet
    syst.set_reload(8_000_000);
    syst.clear_current();
    syst.enable_counter();
    while !syst.has_wrapped() {}

    hprintln!("Hello, world! after 1 second").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

```

## 三、p3-pac

通过 [stm32f1 - crates.io: Rust Package Registry](https://crates.io/crates/stm32f1) 访问 STM32 的外围设备。

在 [assets/PCB原理图.pdf](assets/PCB原理图.pdf) 中可以找到开发板上 RGB LED 的部分：

<img src="./assets/image-20241118143637278.png" alt="image-20241118143637278" style="zoom:50%;" />

还可以捋着 PCB 上的线路对应到 STM32 的针脚上（什么盯针）：

<img src="./assets/image-20241118145124468.png" alt="image-20241118145124468" style="zoom:67%;" />

```rust
//! Light up the LED on PA6 using PAC

#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
#[allow(unused_extern_crates)]
use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f103::Peripherals::take().unwrap();

    let mut syst = cp.SYST;
    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();

    // enable the GPIOA clocks
    p.RCC
        .apb2enr
        .modify(|_, w| w.iopaen().set_bit());

    // configure PA6 to output mode using push-pull
    p.GPIOA
        .crl
        .modify(|_, w| 
            // set PA6 to output mode
            w.mode6().output()
            // set PA6 to push-pull output
            .cnf6().push_pull()
        );

    // set PA6 to high
    p.GPIOA.bsrr.write(|w| w.bs6().set_bit());

    loop {
        // busy wait until the timer wraps around
        while !syst.has_wrapped() {}
        hprintln!(".").unwrap();
    }
}

```

