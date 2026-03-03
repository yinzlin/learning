# 步骤 5：核心概念学习

## 操作目标
掌握 Rust 的基本语法和核心概念，包括变量、数据类型、函数、注释和控制流。

## 执行方法

### 1. 变量与可变性

#### 学习内容
- 变量的声明和初始化
- 变量的可变性（mut）
- 常量（const）
- 隐藏（shadowing）

#### 示例代码
```rust
// 不可变变量
let x = 5;

// 可变变量
let mut y = 5;
y = 6;

// 常量
const MAX_POINTS: u32 = 100_000;

// 隐藏
let z = 5;
let z = z + 1;
```

### 2. 数据类型

#### 学习内容
- 标量类型：整数、浮点数、布尔值、字符
- 复合类型：元组、数组

#### 示例代码
```rust
// 标量类型
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'z';

// 复合类型
let tuple: (i32, f64, bool) = (500, 6.4, true);
let array: [i32; 5] = [1, 2, 3, 4, 5];
```

### 3. 函数

#### 学习内容
- 函数的定义和调用
- 函数参数
- 函数返回值

#### 示例代码
```rust
// 函数定义
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 函数调用
let result = add(5, 3);
```

### 4. 注释

#### 学习内容
- 单行注释
- 多行注释
- 文档注释

#### 示例代码
```rust
// 单行注释

/*
多行注释
多行注释
*/

/// 文档注释
/// 这是一个加法函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 5. 控制流

#### 学习内容
- if 表达式
- 循环：loop、while、for
- match 表达式

#### 示例代码
```rust
// if 表达式
let number = 7;
if number % 2 == 0 {
    println!("偶数");
} else {
    println!("奇数");
}

// loop 循环
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break;
    }
}

// while 循环
let mut number = 3;
while number != 0 {
    println!("{}", number);
    number -= 1;
}

// for 循环
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("值: {}", element);
}

// match 表达式
let coin = Coin::Penny;
let value = match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
};
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 5 天
- **完成时间**：学习计划第 7 天

## 预期成果
- 理解并能应用 Rust 的基本语法
- 掌握变量、数据类型、函数、注释和控制流的使用
- 能够编写简单的 Rust 程序

## 优先级
- **优先级**：高
- **理由**：核心概念是学习 Rust 的基础，是后续所有学习的前提

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 概念理解困难 | 多编写示例代码，参考官方文档和社区资源 |
| 语法错误 | 仔细检查代码，使用 `cargo check` 验证 |
| 类型错误 | 注意类型标注，理解 Rust 的类型系统 |

## 验证步骤

1. 编写包含变量、数据类型、函数、注释和控制流的示例代码
2. 执行 `cargo check` 检查代码是否正确
3. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成核心概念学习后，继续学习 [步骤 6：所有权系统学习](step06-所有权系统.md)。
