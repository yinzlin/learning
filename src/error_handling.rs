/// 错误处理示例模块
/// 涵盖：不可恢复错误、可恢复错误、错误处理策略

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};

/// 演示不可恢复错误
fn demo_panic() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 不可恢复错误 (panic!)           │");
    println!("└─────────────────────────────────────┘");

    println!("\npanic! 宏用于不可恢复的错误：");
    println!("示例：panic!(\"程序发生错误！\");");
    println!("注意：实际运行会终止程序，此处仅演示语法");

    println!("\n常见的 panic 场景：");
    println!("1. 数组越界访问：v[99] 当 v.len() < 100");
    println!("2. 整数溢出（debug 模式）");
    println!("3. 显式调用 panic! 宏");
}

/// 演示可恢复错误
fn demo_result() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 可恢复错误 (Result<T, E>)       │");
    println!("└─────────────────────────────────────┘");

    println!("\nResult 枚举定义：");
    println!("enum Result<T, E> {{");
    println!("    Ok(T),");
    println!("    Err(E),");
    println!("}}");

    println!("\n使用 match 处理 Result：");
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("文件打开成功");
            Some(file)
        }
        Err(error) => {
            println!("文件打开失败：{}", error);
            None
        }
    };

    println!("\n使用 unwrap 和 expect：");
    println!("unwrap：成功返回值，失败 panic");
    println!("expect：同上，但可自定义错误信息");
    println!("示例：File::open(\"hello.txt\").expect(\"无法打开文件\");");

    println!("\n错误传播（? 操作符）：");
    println!("? 操作符简化错误传播：");
    println!("let mut file = File::open(\"hello.txt\")?;");
    println!("file.read_to_string(&mut username)?;");
}

/// 自定义错误类型
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO 错误：{}", e),
            MyError::Parse(e) => write!(f, "解析错误：{}", e),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Parse(e) => Some(e),
        }
    }
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

/// 演示错误处理策略
fn demo_error_strategies() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 错误处理策略                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n何时使用 panic! vs Result：");
    println!("使用 panic! 的情况：");
    println!("  - 示例、原型代码、测试");
    println!("  - 编译器无法检测的无效状态");
    println!("  - 程序处于无法恢复的坏状态");

    println!("\n使用 Result 的情况：");
    println!("  - 预期可能失败的失败场景");
    println!("  - 调用者需要决定如何处理错误");

    println!("\n自定义错误类型：");
    println!("定义：enum MyError {{ Io(io::Error), Parse(ParseIntError) }}");
    println!("实现：Display、Error、From trait");

    println!("\n最佳实践：");
    println!("1. 使用 ? 操作符传播错误");
    println!("2. 为库定义自定义错误类型");
    println!("3. 使用 thiserror 或 anyhow 库简化错误处理");
    println!("4. 在适当层级统一处理错误");
}

/// 演示实际错误处理场景
fn demo_practical_error_handling() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 实际应用示例                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n读取并解析文件中的数字：");
    fn read_and_parse(content: &str) -> Result<i32, MyError> {
        let trimmed = content.trim();
        let number: i32 = trimmed.parse()?;
        Ok(number)
    }

    match read_and_parse("42") {
        Ok(n) => println!("解析成功：{}", n),
        Err(e) => println!("解析失败：{}", e),
    }

    match read_and_parse("not a number") {
        Ok(n) => println!("解析成功：{}", n),
        Err(e) => println!("解析失败：{}", e),
    }

    println!("\n链式错误处理：");
    fn process_file() -> Result<String, MyError> {
        let mut content = String::new();
        File::open("nonexistent.txt")
            .map_err(MyError::from)?
            .read_to_string(&mut content)
            .map_err(MyError::from)?;
        Ok(content)
    }

    match process_file() {
        Ok(content) => println!("文件内容：{}", content),
        Err(e) => println!("处理失败：{}", e),
    }
}

/// 公开的演示函数
pub fn demo_error_handling() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块四：错误处理                    ║");
    println!("╚═════════════════════════════════════════╝");

    demo_panic();
    demo_result();
    demo_error_strategies();
    demo_practical_error_handling();
}
