/// 嵌入式与 no_std 示例模块
/// 涵盖：no_std 环境、embedded-hal、交叉编译、embassy 异步

/// 演示 no_std 环境
fn demo_no_std() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. no_std 环境                     │");
    println!("└─────────────────────────────────────┘");

    println!("\n禁用标准库：");
    println!("#![no_std]");
    println!("#![no_main]");
    println!();
    println!("use core::panic::PanicInfo;");
    println!();
    println!("#[panic_handler]");
    println!("fn panic(_info: &PanicInfo) -> ! {{");
    println!("    loop {{}}");
    println!("}}");

    println!("\ncore vs std：");
    println!("core - 核心库，无操作系统依赖");
    println!("  - 基本类型、trait");
    println!("  - Option, Result");
    println!("  - 迭代器");
    println!("  - Cell, RefCell");
    println!();
    println!("std - 标准库，需要操作系统");
    println!("  - 文件系统");
    println!("  - 网络");
    println!("  - 线程");
    println!("  - 堆分配");

    println!("\n可用的 crate：");
    println!("alloc - 堆分配（Vec, String, Box）");
    println!("collections - 集合类型");
    println!("需要链接 alloc crate");

    println!("\nCargo.toml 配置：");
    println!("[profile.dev]");
    println!("panic = \"abort\"");
    println!();
    println!("[profile.release]");
    println!("panic = \"abort\"");
    println!("lto = true");
    println!("opt-level = \"z\"");

    println!("\n内存布局：");
    println!("MEMORY {{");
    println!("    FLASH : ORIGIN = 0x08000000, LENGTH = 256K");
    println!("    RAM : ORIGIN = 0x20000000, LENGTH = 64K");
    println!("}}");
}

/// 演示 embedded-hal
fn demo_embedded_hal() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. embedded-hal                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n硬件抽象层 trait：");
    println!("数字 I/O：");
    println!("pub trait InputPin {{");
    println!("    type Error;");
    println!("    fn is_high(&self) -> Result<bool, Self::Error>;");
    println!("    fn is_low(&self) -> Result<bool, Self::Error>;");
    println!("}}");
    println!();
    println!("pub trait OutputPin {{");
    println!("    type Error;");
    println!("    fn set_high(&mut self) -> Result<(), Self::Error>;");
    println!("    fn set_low(&mut self) -> Result<(), Self::Error>;");
    println!("}}");

    println!("\nSPI：");
    println!("pub trait SpiBus {{");
    println!("    fn read(&mut self, words: &mut [u8]) -> Result<(), Error>;");
    println!("    fn write(&mut self, words: &[u8]) -> Result<(), Error>;");
    println!("    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error>;");
    println!("}}");

    println!("\nI2C：");
    println!("pub trait I2c {{");
    println!("    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error>;");
    println!("    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error>;");
    println!("}}");

    println!("\n驱动示例：");
    println!("pub struct TemperatureSensor<I2C> {{");
    println!("    i2c: I2C,");
    println!("    address: u8,");
    println!("}}");
    println!();
    println!("impl<I2C: I2c> TemperatureSensor<I2C> {{");
    println!("    pub fn read_temperature(&mut self) -> Result<f32, Error> {{ ... }}");
    println!("}}");

    println!("\n生态：");
    println!("embedded-hal - 核心 trait");
    println!("embedded-hal-async - 异步版本");
    println!("embedded-io - I/O trait");
    println!("embedded-nal - 网络 trait");
}

/// 演示交叉编译
fn demo_cross_compilation() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 交叉编译                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n添加目标：");
    println!("rustup target add thumbv7em-none-eabihf");
    println!("rustup target add aarch64-unknown-none");

    println!("\n常用嵌入式目标：");
    println!("thumbv6m-none-eabi - ARM Cortex-M0/M0+");
    println!("thumbv7m-none-eabi - ARM Cortex-M3");
    println!("thumbv7em-none-eabi - ARM Cortex-M4/M7 (软浮点)");
    println!("thumbv7em-none-eabihf - ARM Cortex-M4F/M7F (硬浮点)");
    println!("riscv32imac-unknown-none-elf - RISC-V 32位");
    println!("riscv64imac-unknown-none-elf - RISC-V 64位");

    println!("\n编译命令：");
    println!("cargo build --target thumbv7em-none-eabihf --release");

    println!("\n.cargo/config.toml：");
    println!("[target.thumbv7em-none-eabihf]");
    println!("runner = \"probe-rs run --chip STM32F407VGTx\"");
    println!("rustflags = [\"-C\", \"link-arg=-Tlink.x\"]");
    println!();
    println!("[build]");
    println!("target = \"thumbv7em-none-eabihf\"");

    println!("\n工具链：");
    println!("probe-rs - 烧录和调试");
    println!("cargo-flash - 快速烧录");
    println!("cargo-embed - 嵌入式开发工具");

    println!("\n调试：");
    println!("probe-rs debug --chip STM32F407VGTx");
    println!("连接 GDB 或 VS Code 调试");
}

/// 演示 embassy 异步框架
fn demo_embassy() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. embassy 异步框架                │");
    println!("└─────────────────────────────────────┘");

    println!("\nembassy 特点：");
    println!("1. 异步嵌入式开发");
    println!("2. 零成本抽象");
    println!("3. 编译时任务调度");
    println!("4. 内置驱动生态");

    println!("\n任务定义：");
    println!("#[embassy_executor::main]");
    println!("async fn main(spawner: Spawner) {{");
    println!("    let p = embassy_stm32::init(Default::default());");
    println!("    spawner.spawn(blink_task(p.PA5)).unwrap();");
    println!("}}");
    println!();
    println!("#[embassy_executor::task]");
    println!("async fn blink_task(mut led: Output<'static>) {{");
    println!("    let mut ticker = Ticker::every(Duration::from_millis(500));");
    println!("    loop {{");
    println!("        led.toggle();");
    println!("        ticker.next().await;");
    println!("    }}");
    println!("}}");

    println!("\n异步驱动：");
    println!("let mut uart = Uart::new(p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, Config::default());");
    println!();
    println!("let mut buf = [0u8; 64];");
    println!("let len = uart.read(&mut buf).await.unwrap();");

    println!("\n执行器：");
    println!("embassy_executor::Executor - 单线程执行器");
    println!("embassy_executor::Spawner - 任务派发");
    println!("embassy_time - 时间和定时器");

    println!("\n支持的硬件：");
    println!("embassy-stm32 - STM32 系列");
    println!("embassy-nrf - Nordic nRF 系列");
    println!("embassy-rp - Raspberry Pi Pico");
}

/// 演示内存管理
fn demo_memory_management() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. 嵌入式内存管理                  │");
    println!("└─────────────────────────────────────┘");

    println!("\n静态分配：");
    println!("static mut BUFFER: [u8; 1024] = [0; 1024];");
    println!("static COUNTER: AtomicU32 = AtomicU32::new(0);");

    println!("\n堆分配（需要 alloc）：");
    println!("use embedded_alloc::Heap;");
    println!();
    println!("#[global_allocator]");
    println!("static HEAP: Heap = Heap::empty();");
    println!();
    println!("fn init_heap() {{");
    println!("    let heap_start = 0x2000_0000 as *mut u8;");
    println!("    let heap_size = 1024 * 16;");
    println!("    unsafe {{ HEAP.init(heap_start, heap_size); }}");
    println!("}}");

    println!("\n链接脚本 (memory.x)：");
    println!("MEMORY {{");
    println!("    FLASH : ORIGIN = 0x08000000, LENGTH = 256K");
    println!("    RAM : ORIGIN = 0x20000000, LENGTH = 64K");
    println!("}}");
    println!();
    println!("SECTIONS {{");
    println!("    .text : {{ *(.text*) }} > FLASH");
    println!("    .rodata : {{ *(.rodata*) }} > FLASH");
    println!("    .data : {{ *(.data*) }} > RAM AT > FLASH");
    println!("    .bss : {{ *(.bss*) }} > RAM");
    println!("}}");

    println!("\n内存优化技巧：");
    println!("1. 使用静态分配");
    println!("2. 避免动态分配");
    println!("3. 使用固定大小缓冲区");
    println!("4. 启用 LTO 优化");
    println!("5. 使用 heapless 集合");
}

/// 公开的演示函数
pub fn demo_embedded() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块十七：嵌入式与 no_std 开发      ║");
    println!("╚═════════════════════════════════════════╝");

    demo_no_std();
    demo_embedded_hal();
    demo_cross_compilation();
    demo_embassy();
    demo_memory_management();
}
