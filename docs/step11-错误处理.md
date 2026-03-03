# 步骤 11：错误处理学习

## 操作目标
掌握 Rust 的错误处理机制，包括不可恢复错误和可恢复错误的处理方法。

## 执行方法

### 1. 不可恢复错误

#### 学习内容
- `panic!` 宏的使用
-  panic 的工作原理
- 配置 panic 的行为

#### 示例代码
```rust
// 使用 panic! 宏
fn main() {
    panic!("程序发生错误！");
}

// 索引越界会触发 panic
fn main2() {
    let v = vec![1, 2, 3];
    v[99]; // 索引越界，触发 panic
}

// 自定义 panic 信息
fn main3() {
    let age = -1;
    if age < 0 {
        panic!("年龄不能为负数: {}", age);
    }
}
```

### 2. 可恢复错误

#### 学习内容
- `Result` 枚举的使用
- 错误处理的基本模式
- `?` 操作符的使用

#### 示例代码
```rust
use std::fs::File;
use std::io::{self, Read};

// 使用 Result 处理错误
fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("username.txt");
    
    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 操作符简化错误处理
fn read_username_from_file_simple() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// 链式调用
fn read_username_from_file_chain() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?
        .read_to_string(&mut username)?;
    Ok(username)
}

// 主函数中的错误处理
fn main() {
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("错误: {}", e),
    }
}
```

### 3. 错误处理策略

#### 学习内容
- 何时使用 panic! 和 Result
- 自定义错误类型
- 错误传播

#### 示例代码
```rust
use std::error::Error;
use std::fmt;

// 自定义错误类型
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO 错误: {}", e),
            MyError::Parse(e) => write!(f, "解析错误: {}", e),
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

// 从其他错误转换
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

// 使用自定义错误类型
fn read_and_parse_file() -> Result<i32, MyError> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

// 主函数
fn main() {
    match read_and_parse_file() {
        Ok(number) => println!("读取的数字: {}", number),
        Err(e) => println!("错误: {}", e),
    }
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 21 天
- **完成时间**：学习计划第 23 天

## 预期成果
- 理解不可恢复错误和可恢复错误的区别
- 掌握 `panic!` 宏的使用场景
- 能够使用 `Result` 枚举处理可恢复错误
- 掌握 `?` 操作符的使用
- 能够定义和使用自定义错误类型

## 优先级
- **优先级**：高
- **理由**：错误处理是编写健壮软件的关键部分，Rust 提供了强大的错误处理机制

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 过度使用 panic! | 仅在真正不可恢复的情况下使用 panic! |
| 错误处理代码冗长 | 使用 ? 操作符简化错误处理 |
| 错误类型设计不当 | 合理设计错误类型，确保信息足够且清晰 |

## 验证步骤

1. 编写包含 panic! 的代码，观察程序行为
2. 编写使用 Result 处理错误的代码
3. 使用 ? 操作符简化错误处理
4. 定义和使用自定义错误类型
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认错误处理正确

## 后续步骤

完成错误处理学习后，继续学习 [步骤 12：泛型与特性学习](step12-泛型与特性.md)。
