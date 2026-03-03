# 步骤 8：枚举与模式匹配

## 操作目标
掌握枚举的定义和使用，以及模式匹配的基本概念和应用。

## 执行方法

### 1. 枚举定义

#### 学习内容
- 枚举的定义
- 枚举变体
- 枚举的实例化
- 枚举的方法实现

#### 示例代码
```rust
// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

// 枚举实例化
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// 带数据的枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// 为枚举实现方法
impl IpAddr {
    fn call(&self) {
        println!("调用 IP 地址");
    }
}

home.call();
```

### 2. Option 枚举

#### 学习内容
- Option 枚举的定义
- Option 的使用场景
- Option 的方法

#### 示例代码
```rust
// Option 枚举的使用
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

// Option 的方法
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 3. match 控制流

#### 学习内容
- match 表达式的语法
- 模式匹配
- 通配模式
- 下划线模式

#### 示例代码
```rust
// match 表达式
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

// 带数据的模式匹配
enum UsState {
    Alabama,
    Alaska,
    // ... 其他州
}

enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_state(coin: CoinWithState) -> u32 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("州纪念币来自 {:?}！", state);
            25
        },
    }
}

// 通配模式和下划线
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // 通配模式
}

// 或使用下划线
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(), // 下划线模式
}
```

### 4. if let 和 let...else

#### 学习内容
- if let 语法
- let...else 语法
- 适用场景

#### 示例代码
```rust
// if let 简化模式匹配
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("是 3");
} else {
    println!("不是 3");
}

// let...else
let Some(x) = some_u8_value else {
    println!("不是 Some");
    return;
};
println!("值是: {}", x);
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 13 天
- **完成时间**：学习计划第 14 天

## 预期成果
- 能够定义和使用枚举
- 掌握 Option 枚举的使用
- 理解 match 表达式的模式匹配
- 能够使用 if let 和 let...else 简化代码

## 优先级
- **优先级**：高
- **理由**：枚举和模式匹配是 Rust 中处理多种可能情况的强大工具，是编写健壮代码的基础

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 模式匹配不完整 | 确保覆盖所有可能的情况，或使用通配模式 |
| Option 处理不当 | 注意空值情况，使用适当的方法处理 Option |
| 枚举变体使用错误 | 确保使用正确的枚举变体和语法 |

## 验证步骤

1. 定义一个枚举并实例化
2. 实现枚举的方法
3. 使用 match 表达式进行模式匹配
4. 使用 if let 和 let...else 简化代码
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成枚举与模式匹配学习后，继续学习 [步骤 9：模块系统学习](step09-模块系统.md)。
