# 步骤 21：模式匹配学习

## 操作目标
掌握 Rust 的模式匹配，包括模式的使用场景、语法和高级用法。

## 执行方法

### 1. 模式使用场景

#### 学习内容
- match 表达式中的模式
- if let 表达式中的模式
- while let 表达式中的模式
- for 循环中的模式
- let 语句中的模式
- 函数参数中的模式

#### 示例代码
```rust
// match 表达式中的模式
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// if let 表达式中的模式
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("是 3");
} else {
    println!("不是 3");
}

// while let 表达式中的模式
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{} 被弹出", top);
}

// for 循环中的模式
let v = vec!['a', 'b', 'c'];
for (index, value) in v.iter().enumerate() {
    println!("{}: '{}'", index, value);
}

// let 语句中的模式
let (x, y, z) = (1, 2, 3);
println!("x = {}, y = {}, z = {}", x, y, z);

// 函数参数中的模式
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("坐标: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);
```

### 2. 可反驳性

#### 学习内容
- 可反驳模式
- 不可反驳模式
- 模式的可反驳性与使用场景

#### 示例代码
```rust
// 不可反驳模式（总是匹配）
let x = 5;
let y = x;

// 可反驳模式（可能不匹配）
let some_option_value: Option<i32> = Some(5);
if let Some(x) = some_option_value {
    println!("x = {}", x);
}

// 可反驳模式在 match 分支中
match some_option_value {
    Some(x) => println!("x = {}", x),
    None => println!("没有值"),
}
```

### 3. 模式语法

#### 学习内容
- 字面值模式
- 变量模式
- 通配符模式
- 下划线模式
- 结构体模式
- 元组模式
- 枚举模式
- 引用模式
- 混合模式

#### 示例代码
```rust
// 字面值模式
let x = 1;
match x {
    1 => println!("一"),
    2 => println!("二"),
    _ => println!("其他"),
}

// 变量模式
let x = Some(5);
match x {
    Some(value) => println!("值: {}", value),
    None => println!("没有值"),
}

// 通配符模式
let (x, y) = (1, 2);
match (x, y) {
    (1, y) => println!("x 是 1, y 是 {}", y),
    _ => println!("其他"),
}

// 下划线模式
let (x, y) = (1, 2);
match (x, y) {
    (1, _) => println!("x 是 1"),
    _ => println!("其他"),
}

// 结构体模式
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
match p {
    Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
    Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
    Point { x, y } => println!("在点 ({}, {})", x, y),
}

// 元组模式
let triple = (1, 2, 3);
match triple {
    (0, y, z) => println!("第一个元素是 0, y = {}, z = {}", y, z),
    (1, ..) => println!("第一个元素是 1, 其他元素忽略"),
    (_, y, _) => println!("第二个元素是 {}", y),
}

// 枚举模式
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(255, 0, 0);
match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    Message::Write(text) => println!("写入: {}", text),
    Message::ChangeColor(r, g, b) => println!("颜色变为 ({}, {}, {})", r, g, b),
}

// 引用模式
let x = 5;
let y = &x;
match y {
    &value => println!("y 是 x 的引用，值为 {}", value),
}

// 混合模式
enum OptionalInt {
    Value(i32),
    Missing,
}

let opt = OptionalInt::Value(5);
match opt {
    OptionalInt::Value(i) if i > 0 => println!("正整数: {}", i),
    OptionalInt::Value(i) => println!("非正整数: {}", i),
    OptionalInt::Missing => println!("缺失值"),
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 54 天
- **完成时间**：学习计划第 55 天

## 预期成果
- 掌握模式匹配的各种使用场景
- 理解可反驳模式和不可反驳模式的区别
- 能够使用各种模式语法
- 能够编写复杂的模式匹配表达式

## 优先级
- **优先级**：中
- **理由**：模式匹配是 Rust 中强大的语言特性，能够使代码更简洁、更安全

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 模式匹配不完整 | 确保覆盖所有可能的情况，或使用通配符 |
| 模式过于复杂 | 保持模式简洁，避免过度嵌套 |
| 性能问题 | 注意模式匹配的性能影响，特别是在大型枚举上 |

## 验证步骤

1. 编写使用不同场景下的模式匹配代码
2. 尝试使用各种模式语法
3. 编写包含复杂模式的代码
4. 执行 `cargo check` 检查代码是否正确
5. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成模式匹配学习后，继续学习 [步骤 22：高级特性学习](step22-高级特性.md)。
