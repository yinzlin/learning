# Rust 编程语言知识体系完整整理

## 目录
- [1. 基础入门](#1-基础入门)
  - [1.1 环境搭建](#11-环境搭建)
  - [1.2 Hello World 与基本编译](#12-hello-world-与基本编译)
  - [1.3 Cargo 项目管理](#13-cargo-项目管理)
- [2. 核心概念](#2-核心概念)
  - [2.1 变量与可变性](#21-变量与可变性)
  - [2.2 数据类型](#22-数据类型)
  - [2.3 函数](#23-函数)
  - [2.4 注释](#24-注释)
  - [2.5 控制流](#25-控制流)
- [3. 所有权系统](#3-所有权系统)
  - [3.1 所有权概念](#31-所有权概念)
  - [3.2 引用与借用](#32-引用与借用)
  - [3.3 切片类型](#33-切片类型)
- [4. 数据结构](#4-数据结构)
  - [4.1 结构体](#41-结构体)
  - [4.2 枚举](#42-枚举)
  - [4.3 集合类型](#43-集合类型)
- [5. 错误处理](#5-错误处理)
  - [5.1 不可恢复错误](#51-不可恢复错误)
  - [5.2 可恢复错误](#52-可恢复错误)
  - [5.3 错误处理策略](#53-错误处理策略)
- [6. 泛型与特性](#6-泛型与特性)
  - [6.1 泛型数据类型](#61-泛型数据类型)
  - [6.2 特性](#62-特性)
  - [6.3 生命周期](#63-生命周期)
- [7. 测试](#7-测试)
  - [7.1 编写测试](#71-编写测试)
  - [7.2 控制测试运行](#72-控制测试运行)
  - [7.3 测试组织](#73-测试组织)
- [8. 命令行程序开发](#8-命令行程序开发)
  - [8.1 命令行参数](#81-命令行参数)
  - [8.2 文件操作](#82-文件操作)
  - [8.3 测试驱动开发](#83-测试驱动开发)
- [9. 函数式编程](#9-函数式编程)
  - [9.1 闭包](#91-闭包)
  - [9.2 迭代器](#92-迭代器)
- [10. Cargo 高级功能](#10-cargo-高级功能)
  - [10.1 发布配置](#101-发布配置)
  - [10.2 发布 Crate](#102-发布-crate)
  - [10.3 工作空间](#103-工作空间)
- [11. 智能指针](#11-智能指针)
  - [11.1 Box&lt;T&gt;](#111-boxt)
  - [11.2 Drop 特性](#112-drop-特性)
  - [11.3 Rc&lt;T&gt;](#113-rct)
  - [11.4 RefCell&lt;T&gt;](#114-refcellt)
- [12. 并发编程](#12-并发编程)
  - [12.1 线程](#121-线程)
  - [12.2 消息传递](#122-消息传递)
  - [12.3 共享状态](#123-共享状态)
  - [12.4 同步原语](#124-同步原语)
- [13. 异步编程](#13-异步编程)
  - [13.1 Futures 与 async/await](#131-futures-与-asyncawait)
  - [13.2 异步并发](#132-异步并发)
  - [13.3 Streams](#133-streams)
  - [13.4 异步 I/O](#134-异步-io)
- [14. 面向对象编程](#14-面向对象编程)
  - [14.1 面向对象特性](#141-面向对象特性)
  - [14.2 特质对象](#142-特质对象)
  - [14.3 设计模式](#143-设计模式)
- [15. 模式匹配](#15-模式匹配)
  - [15.1 使用场景](#151-使用场景)
  - [15.2 可反驳性](#152-可反驳性)
  - [15.3 模式语法](#153-模式语法)
- [16. 高级特性](#16-高级特性)
  - [16.1 不安全 Rust](#161-不安全-rust)
  - [16.2 高级特性](#162-高级特性)
  - [16.3 高级类型](#163-高级类型)
  - [16.4 高级函数和闭包](#164-高级函数和闭包)
  - [16.5 宏](#165-宏)
- [17. Web 服务器开发](#17-web-服务器开发)
  - [17.1 单线程服务器](#171-单线程服务器)
  - [17.2 多线程服务器](#172-多线程服务器)
  - [17.3 优雅关闭](#173-优雅关闭)

---

## 1. 基础入门

### 1.1 环境搭建

#### 安装 Rust

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

**Windows:**
1. 访问 [Rust 官网](https://www.rust-lang.org) 下载安装程序
2. 运行安装程序并按照提示完成安装
3. Windows 系统可能需要安装 Visual Studio 提供的链接器

#### 验证安装
```bash
rustc --version
cargo --version
```

#### 更新 Rust
```bash
rustup update
```

#### 查看本地文档
```bash
rustup doc --book
```

### 1.2 Hello World 与基本编译

#### 创建源文件
```rust
fn main() {
    println!("Hello, world!");
}
```

#### 编译程序
```bash
rustc main.rs
```

#### 运行程序
**Linux/macOS:**
```bash
./main
```

**Windows:**
```bash
.\main.exe
```

### 1.3 Cargo 项目管理

#### 创建新项目
```bash
cargo new hello_cargo
```

#### 项目结构
```
hello_cargo/
├── Cargo.toml
└── src/
    └── main.rs
```

#### Cargo.toml 配置
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

#### 常用命令
```bash
# 构建项目
cargo build

# 运行项目
cargo run

# 检查代码（不生成可执行文件）
cargo check

# 构建发布版本（优化）
cargo build --release
```

---

## 2. 核心概念

### 2.1 变量与可变性

#### 不可变变量
```rust
let x = 5;
```

#### 可变变量
```rust
let mut y = 5;
y = 6;
```

#### 常量
```rust
const MAX_POINTS: u32 = 100_000;
```

#### 隐藏（Shadowing）
```rust
let z = 5;
let z = z + 1;
```

### 2.2 数据类型

#### 标量类型
```rust
// 整数
let integer: i32 = 42;

// 浮点数
let float: f64 = 3.14;

// 布尔值
let boolean: bool = true;

// 字符
let character: char = 'z';
```

#### 复合类型
```rust
// 元组
let tuple: (i32, f64, bool) = (500, 6.4, true);

// 数组
let array: [i32; 5] = [1, 2, 3, 4, 5];
```

### 2.3 函数

#### 函数定义
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### 函数调用
```rust
let result = add(5, 3);
```

### 2.4 注释

#### 单行注释
```rust
// 这是单行注释
```

#### 多行注释
```rust
/*
这是多行注释
这是多行注释
*/
```

#### 文档注释
```rust
/// 这是文档注释
/// 用于生成文档
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 2.5 控制流

#### if 表达式
```rust
let number = 7;
if number % 2 == 0 {
    println!("偶数");
} else {
    println!("奇数");
}
```

#### loop 循环
```rust
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break;
    }
}
```

#### while 循环
```rust
let mut number = 3;
while number != 0 {
    println!("{}", number);
    number -= 1;
}
```

#### for 循环
```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("值: {}", element);
}
```

---

## 3. 所有权系统

### 3.1 所有权概念

#### 所有权转移
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 的所有权转移给 s2
// println!("{}", s1); // 编译错误，s1 已无效
```

#### 复制语义
```rust
let x = 5;
let y = x; // x 的值被复制，x 仍然有效
println!("x = {}, y = {}", x, y);
```

#### 克隆语义
```rust
let s3 = String::from("hello");
let s4 = s3.clone(); // 显式克隆，s3 仍然有效
println!("s3 = {}, s4 = {}", s3, s4);
```

### 3.2 引用与借用

#### 不可变引用
```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("字符串 '{}' 的长度是 {}", s1, len);
```

#### 可变引用
```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s2 = String::from("hello");
change(&mut s2);
println!("{}", s2);
```

#### 借用规则
1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
2. 引用必须总是有效的

```rust
let mut s3 = String::from("hello");
let r1 = &s3;
let r2 = &s3; // 多个不可变引用，允许
// let r3 = &mut s3; // 编译错误，不能同时有可变和不可变引用
println!("r1 = {}, r2 = {}", r1, r2);
```

### 3.3 切片类型

#### 字符串切片
```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
println!("{} {}", hello, world);
```

#### 数组切片
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
println!("切片元素: {:?}", slice);
```

---

## 4. 数据结构

### 4.1 结构体

#### 定义和实例化
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

#### 元组结构体
```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

#### 方法实现
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

### 4.2 枚举

#### 枚举定义
```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
```

#### 带数据的枚举
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
```

#### Option 枚举
```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

#### match 表达式
```rust
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
```

#### if let
```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("是 3");
} else {
    println!("不是 3");
}
```

### 4.3 集合类型

#### 向量（Vector）
```rust
// 创建向量
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);

// 使用 vec! 宏
let v2 = vec![1, 2, 3];

// 访问元素
let third: &i32 = &v2[2];
let third = v2.get(2); // 返回 Option

// 遍历
for i in &v2 {
    println!("{}", i);
}
```

#### 字符串（String）
```rust
// 创建字符串
let s1 = String::new();
let s2 = "initial contents".to_string();
let s3 = String::from("initial contents");

// 修改字符串
let mut s4 = String::from("hello");
s4.push_str(", world");
s4.push('!');

// 拼接
let s5 = String::from("hello");
let s6 = String::from("world");
let s7 = format!("{} {}", s5, s6);
```

#### 哈希映射（HashMap）
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 访问
let team_name = String::from("Blue");
let score = scores.get(&team_name);

// 遍历
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// entry API
scores.entry(String::from("Red")).or_insert(30);
```

---

## 5. 错误处理

### 5.1 不可恢复错误

#### panic! 宏
```rust
fn main() {
    panic!("程序发生错误！");
}
```

#### 索引越界触发 panic
```rust
let v = vec![1, 2, 3];
v[99]; // 索引越界，触发 panic
```

### 5.2 可恢复错误

#### Result 枚举
```rust
use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

#### 处理 Result
```rust
fn main() {
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("错误: {}", e),
    }
}
```

### 5.3 错误处理策略

#### 自定义错误类型
```rust
use std::error::Error;
use std::fmt;

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

impl Error for MyError {}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}
```

---

## 6. 泛型与特性

### 6.1 泛型数据类型

#### 泛型函数
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

#### 泛型结构体
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

### 6.2 特性

#### 定义特性
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### 实现特性
```rust
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

#### 特性作为参数
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### 6.3 生命周期

#### 生命周期标注
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### 结构体中的生命周期
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

---

## 7. 测试

### 7.1 编写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }
}
```

### 7.2 控制测试运行

```bash
# 运行特定测试
cargo test it_works

# 运行包含特定字符串的测试
cargo test works

# 忽略测试
#[test]
#[ignore]
fn expensive_test() {
    // 耗时操作
}

# 运行忽略的测试
cargo test -- --ignored
```

### 7.3 测试组织

- **单元测试**: 放在 src 目录中，与代码在同一文件
- **集成测试**: 放在 tests 目录中
- **文档测试**: 在文档注释中编写示例代码

---

## 8. 命令行程序开发

### 8.1 命令行参数

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("命令行参数: {:?}", args);
    
    if args.len() < 2 {
        println!("使用方法: {} <参数>", args[0]);
        return;
    }
    
    let query = &args[1];
    let file_path = &args[2];
    
    println!("搜索: {}", query);
    println!("在文件: {} 中搜索", file_path);
}
```

### 8.2 文件操作

```rust
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

---

## 9. 函数式编程

### 9.1 闭包

```rust
// 闭包定义
let add_one = |x| x + 1;

// 多参数闭包
let add = |a, b| a + b;

// 捕获环境变量
let x = 5;
let add_x = |y| x + y;

// 闭包作为参数
fn apply<F>(f: F) where F: FnOnce() {
    f();
}
```

### 9.2 迭代器

```rust
// 基本迭代器
let v1 = vec![1, 2, 3];
for val in v1.iter() {
    println!("值: {}", val);
}

// 消费适配器
let total: i32 = v1.iter().sum();

// 迭代器适配器
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

// 链式调用
let v3: Vec<_> = v1.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

---

## 10. Cargo 高级功能

### 10.1 发布配置

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### 10.2 发布 Crate

```toml
[package]
name = "my_crate"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
description = "A fantastic crate"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/my_crate"
```

```bash
# 登录
cargo login

# 发布
cargo publish
```

### 10.3 工作空间

```toml
[workspace]
members = [
    "adder",
    "subtractor",
    "multiplier",
]
```

```bash
# 构建整个工作空间
cargo build

# 运行特定 crate
cargo run -p adder
```

---

## 11. 智能指针

### 11.1 Box&lt;T&gt;

```rust
// 堆上存储数据
let b = Box::new(5);

// 递归类型
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
```

### 11.2 Drop 特性

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("释放 CustomSmartPointer，数据: {}", self.data);
    }
}

let c = CustomSmartPointer { data: String::from("my stuff") };
drop(c); // 手动释放
```

### 11.3 Rc&lt;T&gt;

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);
let c = Rc::clone(&a);

println!("引用计数: {}", Rc::strong_count(&a)); // 3
```

### 11.4 RefCell&lt;T&gt;

```rust
use std::cell::RefCell;

let x = RefCell::new(5);

{
    let mut y = x.borrow_mut();
    *y += 1;
}

println!("x = {:?}", x.borrow());
```

---

## 12. 并发编程

### 12.1 线程

```rust
use std::thread;
use std::time::Duration;

// 创建线程
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("线程中: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// 等待线程结束
handle.join().unwrap();
```

### 12.2 消息传递

```rust
use std::sync::mpsc;

// 创建通道
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("你好");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("收到: {}", received);
```

### 12.3 共享状态

```rust
use std::sync::{Mutex, Arc};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("结果: {}", *counter.lock().unwrap());
```

### 12.4 同步原语

#### 原子类型
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```

---

## 13. 异步编程

### 13.1 Futures 与 async/await

```rust
// Cargo.toml: tokio = { version = "1.0", features = ["full"] }

#[tokio::main]
async fn main() {
    hello_world().await;
}

async fn hello_world() {
    println!("hello");
    say_world().await;
}

async fn say_world() {
    println!("world");
}
```

### 13.2 异步并发

```rust
use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 完成");
}

async fn task2() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 完成");
}

#[tokio::main]
async fn main() {
    tokio::join!(task1(), task2());
    println!("所有任务完成");
}
```

### 13.3 Streams

```rust
use tokio::stream::StreamExt;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let mut stream = tokio::stream::iter(vec![1, 2, 3, 4, 5]);
    
    while let Some(item) = stream.next().await {
        println!("Item: {}", item);
        time::sleep(Duration::from_millis(500)).await;
    }
}
```

### 13.4 异步 I/O

```rust
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 写入文件
    let mut file = File::create("hello.txt").await?;
    file.write_all(b"Hello, world!").await?;
    
    // 读取文件
    let mut file = File::open("hello.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("文件内容: {}", contents);
    
    Ok(())
}
```

---

## 14. 面向对象编程

### 14.1 面向对象特性

```rust
// 封装
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### 14.2 特质对象

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: {}", self.label);
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}
```

### 14.3 设计模式

#### 策略模式
```rust
pub trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

pub struct CreditCardPayment {
    card_number: String,
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) {
        println!("使用信用卡 {} 支付 {} 元", self.card_number, amount);
    }
}

pub struct ShoppingCart {
    payment_strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    pub fn checkout(&self, amount: f64) {
        self.payment_strategy.pay(amount);
    }
}
```

---

## 15. 模式匹配

### 15.1 使用场景

```rust
// match 表达式
enum Coin { Penny, Nickel, Dime, Quarter }
let value = match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
};

// if let
if let Some(3) = some_u8_value {
    println!("是 3");
}

// while let
while let Some(top) = stack.pop() {
    println!("{} 被弹出", top);
}

// for 循环
for (index, value) in v.iter().enumerate() {
    println!("{}: '{}'", index, value);
}

// let 语句
let (x, y, z) = (1, 2, 3);
```

### 15.2 模式语法

```rust
// 字面值模式
let x = 1;
match x {
    1 => println!("一"),
    2 => println!("二"),
    _ => println!("其他"),
}

// 结构体模式
struct Point { x: i32, y: i32 }
let p = Point { x: 1, y: 2 };
match p {
    Point { x, y: 0 } => println!("在 x 轴上"),
    Point { x: 0, y } => println!("在 y 轴上"),
    Point { x, y } => println!("在点 ({}, {})", x, y),
}

// 元组模式
let triple = (1, 2, 3);
match triple {
    (0, y, z) => println!("y = {}, z = {}", y, z),
    (1, ..) => println!("第一个元素是 1"),
    (_, y, _) => println!("第二个元素是 {}", y),
}

// 枚举模式
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Move { x: 10, y: 20 };
match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    Message::Write(text) => println!("写入: {}", text),
}
```

---

## 16. 高级特性

### 16.1 不安全 Rust

```rust
// 原始指针
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1: {:?}", *r1);
    println!("r2: {:?}", *r2);
}

// 不安全函数
unsafe fn dangerous() {
    println!("不安全操作");
}

fn main() {
    unsafe {
        dangerous();
    }
}
```

### 16.2 高级特性

#### 关联类型
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 16.3 高级类型

#### 新类型模式
```rust
struct Years(u64);
struct Days(u64);

impl Years {
    fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}
```

#### 类型别名
```rust
type Kilometers = i32;
let x: Kilometers = 5;
```

#### 从未类型
```rust
fn never_returns() -> ! {
    panic!("这个函数永远不会返回");
}
```

### 16.4 高级函数和闭包

#### 函数指针
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

#### 返回闭包
```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

### 16.5 宏

#### 声明式宏
```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    say_hello!();
    say_hello!("World");
}
```

---

## 17. Web 服务器开发

### 17.1 单线程服务器

```rust
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### 17.2 多线程服务器

```rust
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
```

---

## 附录

### Rust 学习资源

#### 官方资源
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 编程语言](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

#### 社区资源
- [Rust 论坛](https://users.rust-lang.org/)
- [Rust 子reddit](https://www.reddit.com/r/rust/)

---

**文档版本**: 1.0  
**最后更新**: 2026-03-06  
**基于**: step01-step24 学习资料
