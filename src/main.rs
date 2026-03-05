mod basics;
mod ownership;
mod data_structures;
mod error_handling;
mod generics_traits;
mod functional;
mod smart_pointers;
mod concurrency;
mod advanced;

use basics::*;
use ownership::*;
use data_structures::*;
use error_handling::*;
use generics_traits::*;
use functional::*;
use smart_pointers::*;
use concurrency::*;
use advanced::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          Rust 编程语言知识体系完整示例代码                  ║");
    println!("║          Rust Programming Language Complete Examples        ║");
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

    println!("\n✅ 所有示例代码演示完成！");
}
