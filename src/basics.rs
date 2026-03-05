/// 基础概念示例模块
/// 涵盖：变量、数据类型、函数、控制流

/// 演示变量与可变性
fn demo_variables() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 变量与可变性                    │");
    println!("└─────────────────────────────────────┘");

    let x = 5;
    println!("不可变变量 x = {}", x);

    let mut y = 5;
    println!("可变变量 y = {}", y);
    y = 6;
    println!("修改后 y = {}", y);

    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("隐藏后的 z = {}", z);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("隐藏改变类型：spaces = {}", spaces);
}

/// 演示数据类型
fn demo_data_types() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 数据类型                        │");
    println!("└─────────────────────────────────────┘");

    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';

    println!("标量类型：integer={}, float={}, boolean={}, char={}", 
             integer, float, boolean, character);

    let tuple: (i32, f64, bool) = (500, 6.4, true);
    let (x, y, z) = tuple;
    println!("元组解构：x={}, y={}, z={}", x, y, z);
    println!("元组索引：tuple.0={}, tuple.1={}, tuple.2={}", 
             tuple.0, tuple.1, tuple.2);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组：{:?}", array);
    
    let repeated = [3; 5];
    println!("重复数组 [3; 5] = {:?}", repeated);
}

/// 演示函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) {
    println!("你好，{}！", name);
}

fn demo_functions() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 函数                            │");
    println!("└─────────────────────────────────────┘");

    let result = add(5, 3);
    println!("add(5, 3) = {}", result);

    greet("Rust 学习者");

    let early_return = |x: i32| {
        if x > 5 {
            return x * 2;
        }
        x + 1
    };
    println!("早期返回：early_return(10) = {}", early_return(10));
}

/// 演示控制流
fn demo_control_flow() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 控制流                          │");
    println!("└─────────────────────────────────────┘");

    let number = 7;
    if number % 2 == 0 {
        println!("if 表达式：{} 是偶数", number);
    } else {
        println!("if 表达式：{} 是奇数", number);
    }

    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("if 作为表达式：value = {}", value);

    println!("\nloop 循环示例：");
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            break count * 2;
        }
    };
    println!("loop 返回值：{}", result);

    println!("\nwhile 循环示例：");
    let mut number = 3;
    while number != 0 {
        println!("倒计时：{}", number);
        number -= 1;
    }
    println!("发射！");

    println!("\nfor 循环示例：");
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("数组元素：{}", element);
    }

    println!("\n范围循环示例：");
    for number in 1..=3 {
        println!("范围 1..=3：{}", number);
    }

    println!("\n反向循环示例：");
    for number in (1..4).rev() {
        println!("反向范围：{}", number);
    }
}

/// 演示注释类型
fn demo_comments() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. 注释                            │");
    println!("└─────────────────────────────────────┘");

    println!("单行注释：// 这是单行注释");
    println!("多行注释：/* 这是多行注释 */");
    println!("文档注释：/// 用于生成文档");
}

/// 公开的演示函数
pub fn demo_basics() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块一：基础概念                    ║");
    println!("╚═════════════════════════════════════════╝");

    demo_variables();
    demo_data_types();
    demo_functions();
    demo_control_flow();
    demo_comments();
}
