mod basics;
mod ownership;
mod data_structures;
mod error_handling;
mod generics_traits;
mod functional;
mod smart_pointers;
mod concurrency;
mod advanced;
mod proc_macros;
mod async_internals;
mod ffi;
mod type_system;
mod performance;
mod ecosystem;
mod architecture;
mod embedded;
mod wasm;

use basics::*;
use ownership::*;
use data_structures::*;
use error_handling::*;
use generics_traits::*;
use functional::*;
use smart_pointers::*;
use concurrency::*;
use advanced::*;
use proc_macros::*;
use async_internals::*;
use ffi::*;
use type_system::*;
use performance::*;
use ecosystem::*;
use architecture::*;
use embedded::*;
use wasm::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          Rust 编程语言知识体系完整示例代码                  ║");
    println!("║          Rust Programming Language Complete Examples        ║");
    println!("║                    版本 2.0 - 进阶版                        ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    demo_basics();
    demo_ownership();
    demo_data_structures();
    demo_error_handling();
    demo_generics_traits();
    demo_functional();
    demo_smart_pointers();
    demo_concurrency();
    demo_advanced();
    demo_proc_macros();
    demo_async_internals();
    demo_ffi();
    demo_type_system();
    demo_performance();
    demo_ecosystem();
    demo_architecture();
    demo_embedded();
    demo_wasm();

    println!("\n✅ 所有示例代码演示完成！");
}
