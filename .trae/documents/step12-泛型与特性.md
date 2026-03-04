# 步骤 12：泛型与特性学习

## 操作目标
掌握 Rust 的泛型和特性系统，包括泛型数据类型、特性定义和生命周期标注。

## 执行方法

### 1. 泛型数据类型

#### 学习内容
- 泛型函数
- 泛型结构体
- 泛型枚举
- 泛型方法

#### 示例代码
```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// 泛型方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定类型的方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 泛型枚举
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 2. 特性定义

#### 学习内容
- 特性的定义
- 特性的实现
- 特性作为参数
- 特性作为返回类型
- 特性约束

#### 示例代码
```rust
// 定义特性
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为类型实现特性
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 特性作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 特性约束
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 多个特性约束
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// where 子句
pub fn notify<T>(item: &T) where T: Summary + Display {
    println!("Breaking news! {}", item.summarize());
}

// 特性作为返回类型
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

### 3. 生命周期

#### 学习内容
- 生命周期的概念
- 生命周期标注
- 生命周期省略规则
- 静态生命周期

#### 示例代码
```rust
// 生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期
let s: &'static str = "I have a static lifetime";
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 24 天
- **完成时间**：学习计划第 26 天

## 预期成果
- 能够定义和使用泛型函数和类型
- 掌握特性的定义和实现
- 理解特性约束和使用场景
- 能够使用生命周期标注解决借用问题

## 优先级
- **优先级**：高
- **理由**：泛型和特性是 Rust 中实现代码重用和抽象的重要工具，是编写灵活和可维护代码的基础

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 泛型代码性能问题 | 理解 Rust 的泛型实现，避免过度使用泛型 |
| 特性实现错误 | 确保正确实现特性的所有方法 |
| 生命周期标注错误 | 遵循生命周期省略规则，必要时添加显式标注 |

## 验证步骤

1. 编写泛型函数和类型
2. 定义和实现特性
3. 使用特性约束和 where 子句
4. 编写包含生命周期标注的代码
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成泛型与特性学习后，继续学习 [步骤 13：测试学习](step13-测试.md)。
