# Rust 编程语言知识体系完整整理

## 目录
- [Rust 编程语言知识体系完整整理](#rust-编程语言知识体系完整整理)
  - [目录](#目录)
  - [1. 基础入门](#1-基础入门)
    - [1.1 环境搭建](#11-环境搭建)
      - [安装 Rust](#安装-rust)
      - [验证安装](#验证安装)
      - [更新 Rust](#更新-rust)
      - [查看本地文档](#查看本地文档)
    - [1.2 Hello World 与基本编译](#12-hello-world-与基本编译)
      - [创建源文件](#创建源文件)
      - [编译程序](#编译程序)
      - [运行程序](#运行程序)
    - [1.3 Cargo 项目管理](#13-cargo-项目管理)
      - [创建新项目](#创建新项目)
      - [项目结构](#项目结构)
      - [Cargo.toml 配置](#cargotoml-配置)
      - [常用命令](#常用命令)
  - [2. 核心概念](#2-核心概念)
    - [2.1 变量与可变性](#21-变量与可变性)
      - [不可变变量](#不可变变量)
      - [可变变量](#可变变量)
      - [常量](#常量)
      - [隐藏（Shadowing）](#隐藏shadowing)
    - [2.2 数据类型](#22-数据类型)
      - [标量类型](#标量类型)
      - [复合类型](#复合类型)
    - [2.3 函数](#23-函数)
      - [函数定义](#函数定义)
      - [函数调用](#函数调用)
    - [2.4 注释](#24-注释)
      - [单行注释](#单行注释)
      - [多行注释](#多行注释)
      - [文档注释](#文档注释)
    - [2.5 控制流](#25-控制流)
      - [if 表达式](#if-表达式)
      - [loop 循环](#loop-循环)
      - [while 循环](#while-循环)
      - [for 循环](#for-循环)
  - [3. 所有权系统](#3-所有权系统)
    - [3.1 所有权概念](#31-所有权概念)
      - [所有权转移](#所有权转移)
      - [复制语义](#复制语义)
      - [克隆语义](#克隆语义)
    - [3.2 引用与借用](#32-引用与借用)
      - [不可变引用](#不可变引用)
      - [可变引用](#可变引用)
      - [借用规则](#借用规则)
    - [3.3 切片类型](#33-切片类型)
      - [字符串切片](#字符串切片)
      - [数组切片](#数组切片)
  - [4. 数据结构](#4-数据结构)
    - [4.1 结构体](#41-结构体)
      - [定义和实例化](#定义和实例化)
      - [元组结构体](#元组结构体)
      - [方法实现](#方法实现)
    - [4.2 枚举](#42-枚举)
      - [枚举定义](#枚举定义)
      - [带数据的枚举](#带数据的枚举)
      - [Option 枚举](#option-枚举)
      - [match 表达式](#match-表达式)
      - [if let](#if-let)
    - [4.3 集合类型](#43-集合类型)
      - [向量（Vector）](#向量vector)
      - [字符串（String）](#字符串string)
      - [哈希映射（HashMap）](#哈希映射hashmap)
  - [5. 错误处理](#5-错误处理)
    - [5.1 不可恢复错误](#51-不可恢复错误)
      - [panic! 宏](#panic-宏)
      - [索引越界触发 panic](#索引越界触发-panic)
    - [5.2 可恢复错误](#52-可恢复错误)
      - [Result 枚举](#result-枚举)
      - [处理 Result](#处理-result)
    - [5.3 错误处理策略](#53-错误处理策略)
      - [自定义错误类型](#自定义错误类型)
  - [6. 泛型与特性](#6-泛型与特性)
    - [6.1 泛型数据类型](#61-泛型数据类型)
      - [泛型函数](#泛型函数)
      - [泛型结构体](#泛型结构体)
    - [6.2 特性](#62-特性)
      - [定义特性](#定义特性)
      - [实现特性](#实现特性)
      - [特性作为参数](#特性作为参数)
    - [6.3 生命周期](#63-生命周期)
      - [生命周期标注](#生命周期标注)
      - [结构体中的生命周期](#结构体中的生命周期)
  - [7. 测试](#7-测试)
    - [7.1 编写测试](#71-编写测试)
    - [7.2 控制测试运行](#72-控制测试运行)
    - [7.3 测试组织](#73-测试组织)
  - [8. 命令行程序开发](#8-命令行程序开发)
    - [8.1 命令行参数](#81-命令行参数)
    - [8.2 文件操作](#82-文件操作)
  - [9. 函数式编程](#9-函数式编程)
    - [9.1 闭包](#91-闭包)
    - [9.2 迭代器](#92-迭代器)
  - [10. Cargo 高级功能](#10-cargo-高级功能)
    - [10.1 发布配置](#101-发布配置)
    - [10.2 发布 Crate](#102-发布-crate)
    - [10.3 工作空间](#103-工作空间)
  - [11. 智能指针](#11-智能指针)
    - [11.1 Box\<T\>](#111-boxt)
    - [11.2 Drop 特性](#112-drop-特性)
    - [11.3 Rc\<T\>](#113-rct)
    - [11.4 RefCell\<T\>](#114-refcellt)
  - [12. 并发编程](#12-并发编程)
    - [12.1 线程](#121-线程)
    - [12.2 消息传递](#122-消息传递)
    - [12.3 共享状态](#123-共享状态)
    - [12.4 同步原语](#124-同步原语)
      - [原子类型](#原子类型)
  - [13. 异步编程](#13-异步编程)
    - [13.1 Futures 与 async/await](#131-futures-与-asyncawait)
    - [13.2 异步并发](#132-异步并发)
    - [13.3 Streams](#133-streams)
    - [13.4 异步 I/O](#134-异步-io)
  - [14. 面向对象编程](#14-面向对象编程)
    - [14.1 面向对象特性](#141-面向对象特性)
    - [14.2 特质对象](#142-特质对象)
    - [14.3 设计模式](#143-设计模式)
      - [策略模式](#策略模式)
  - [15. 模式匹配](#15-模式匹配)
    - [15.1 使用场景](#151-使用场景)
    - [15.2 模式语法](#152-模式语法)
  - [16. 高级特性](#16-高级特性)
    - [16.1 不安全 Rust](#161-不安全-rust)
    - [16.2 高级特性](#162-高级特性)
      - [关联类型](#关联类型)
    - [16.3 高级类型](#163-高级类型)
      - [新类型模式](#新类型模式)
      - [类型别名](#类型别名)
      - [从未类型](#从未类型)
    - [16.4 高级函数和闭包](#164-高级函数和闭包)
      - [函数指针](#函数指针)
      - [返回闭包](#返回闭包)
    - [16.5 宏](#165-宏)
      - [声明式宏](#声明式宏)
  - [17. Web 服务器开发](#17-web-服务器开发)
    - [17.1 单线程服务器](#171-单线程服务器)
    - [17.2 多线程服务器](#172-多线程服务器)
  - [18. 过程宏深入](#18-过程宏深入)
    - [18.1 声明式宏进阶](#181-声明式宏进阶)
      - [复杂模式匹配](#复杂模式匹配)
      - [递归宏](#递归宏)
      - [卫生性与标识符捕获](#卫生性与标识符捕获)
    - [18.2 派生宏](#182-派生宏)
      - [项目结构](#项目结构-1)
      - [Cargo.toml 配置](#cargotoml-配置-1)
      - [实现自定义派生宏](#实现自定义派生宏)
      - [使用派生宏](#使用派生宏)
      - [带属性的派生宏](#带属性的派生宏)
    - [18.3 属性宏](#183-属性宏)
      - [实现属性宏](#实现属性宏)
      - [使用属性宏](#使用属性宏)
      - [条件编译属性](#条件编译属性)
    - [18.4 函数式过程宏](#184-函数式过程宏)
      - [实现函数式宏](#实现函数式宏)
      - [SQL 查询宏](#sql-查询宏)
      - [编译时计算宏](#编译时计算宏)
    - [18.5 宏最佳实践](#185-宏最佳实践)
      - [错误处理](#错误处理)
      - [文档生成](#文档生成)
      - [性能考虑](#性能考虑)
  - [19. 异步编程底层原理](#19-异步编程底层原理)
    - [19.1 Future Trait 深入](#191-future-trait-深入)
      - [Future Trait 定义](#future-trait-定义)
      - [手动实现 Future](#手动实现-future)
      - [async/await 编译展开](#asyncawait-编译展开)
    - [19.2 Pin 与 Unpin](#192-pin-与-unpin)
      - [为什么需要 Pin](#为什么需要-pin)
      - [Pin 类型](#pin-类型)
      - [Unpin Trait](#unpin-trait)
      - [Pin 安全性保证](#pin-安全性保证)
    - [19.3 自定义 Future](#193-自定义-future)
      - [实现组合器](#实现组合器)
      - [实现 select](#实现-select)
    - [19.4 异步运行时原理](#194-异步运行时原理)
      - [简单执行器实现](#简单执行器实现)
      - [工作窃取调度器](#工作窃取调度器)
  - [20. FFI 与跨语言互操作](#20-ffi-与跨语言互操作)
    - [20.1 调用 C 代码](#201-调用-c-代码)
      - [基本调用](#基本调用)
      - [链接 C 库](#链接-c-库)
      - [回调函数](#回调函数)
    - [20.2 为 C 提供接口](#202-为-c-提供接口)
      - [导出函数](#导出函数)
      - [导出结构体](#导出结构体)
      - [不透明类型模式](#不透明类型模式)
    - [20.3 与 C++ 交互](#203-与-c-交互)
      - [C++ ABI 兼容](#c-abi-兼容)
      - [RAII 包装](#raii-包装)
    - [20.4 bindgen 与 cbindgen](#204-bindgen-与-cbindgen)
      - [使用 bindgen 自动生成绑定](#使用-bindgen-自动生成绑定)
      - [使用 cbindgen 生成 C 头文件](#使用-cbindgen-生成-c-头文件)
      - [cbindgen.toml 配置](#cbindgentoml-配置)
    - [20.5 安全封装模式](#205-安全封装模式)
      - [类型安全包装](#类型安全包装)
      - [资源管理 RAII](#资源管理-raii)
  - [21. 类型系统进阶](#21-类型系统进阶)
    - [21.1 泛型关联类型 (GATs)](#211-泛型关联类型-gats)
      - [基本用法](#基本用法)
      - [实现借用迭代器](#实现借用迭代器)
      - [数据库连接池模式](#数据库连接池模式)
    - [21.2 关联类型默认值](#212-关联类型默认值)
      - [带默认值的关联类型](#带默认值的关联类型)
    - [21.3 类型级编程](#213-类型级编程)
      - [类型级自然数](#类型级自然数)
      - [类型级列表](#类型级列表)
      - [类型级布尔运算](#类型级布尔运算)
      - [类型级条件](#类型级条件)
    - [21.4 高阶类型模拟](#214-高阶类型模拟)
      - [Functor 模拟](#functor-模拟)
      - [Monad 模拟](#monad-模拟)
    - [21.5 幻影类型与类型状态](#215-幻影类型与类型状态)
      - [幻影类型](#幻影类型)
      - [类型状态模式](#类型状态模式)
      - [连接状态机](#连接状态机)
  - [22. 性能优化与基准测试](#22-性能优化与基准测试)
    - [22.1 性能分析工具](#221-性能分析工具)
      - [使用 perf 进行性能分析](#使用-perf-进行性能分析)
      - [使用 cargo-flamegraph](#使用-cargo-flamegraph)
      - [使用 valgrind 分析内存](#使用-valgrind-分析内存)
      - [使用 pprof](#使用-pprof)
    - [22.2 基准测试](#222-基准测试)
      - [使用 criterion.rs](#使用-criterionrs)
      - [比较不同实现](#比较不同实现)
      - [异步基准测试](#异步基准测试)
    - [22.3 编译优化选项](#223-编译优化选项)
      - [Cargo.toml 优化配置](#cargotoml-优化配置)
      - [链接时优化 (LTO)](#链接时优化-lto)
      - [PGO (Profile-Guided Optimization)](#pgo-profile-guided-optimization)
    - [22.4 内存布局优化](#224-内存布局优化)
      - [使用 #\[repr\] 控制布局](#使用-repr-控制布局)
      - [字段排序优化](#字段排序优化)
      - [使用小型数组](#使用小型数组)
      - [使用小型字符串](#使用小型字符串)
    - [22.5 SIMD 向量化](#225-simd-向量化)
      - [使用 std::simd](#使用-stdsimd)
      - [使用 packed\_simd\_2](#使用-packed_simd_2)
      - [手动向量化提示](#手动向量化提示)
  - [23. 生态系统与常用 Crate](#23-生态系统与常用-crate)
    - [23.1 序列化：serde](#231-序列化serde)
      - [基本用法](#基本用法-1)
      - [自定义序列化](#自定义序列化)
      - [枚举序列化](#枚举序列化)
    - [23.2 错误处理：thiserror 与 anyhow](#232-错误处理thiserror-与-anyhow)
      - [使用 thiserror 定义错误](#使用-thiserror-定义错误)
      - [使用 anyhow 处理应用错误](#使用-anyhow-处理应用错误)
      - [组合使用](#组合使用)
    - [23.3 异步运行时：tokio 深入](#233-异步运行时tokio-深入)
      - [运行时配置](#运行时配置)
      - [任务管理](#任务管理)
      - [同步原语](#同步原语)
    - [23.4 数据库：sqlx 与 diesel](#234-数据库sqlx-与-diesel)
      - [使用 sqlx](#使用-sqlx)
      - [使用 diesel](#使用-diesel)
    - [23.5 Web 框架：axum 与 actix-web](#235-web-框架axum-与-actix-web)
      - [使用 axum](#使用-axum)
      - [使用 actix-web](#使用-actix-web)
  - [24. 大型项目架构设计](#24-大型项目架构设计)
    - [24.1 模块化设计原则](#241-模块化设计原则)
      - [模块组织结构](#模块组织结构)
      - [模块可见性控制](#模块可见性控制)
      - [重导出模式](#重导出模式)
    - [24.2 分层架构](#242-分层架构)
      - [领域层](#领域层)
      - [基础设施层](#基础设施层)
      - [应用层](#应用层)
    - [24.3 依赖注入模式](#243-依赖注入模式)
      - [使用 trait 进行依赖注入](#使用-trait-进行依赖注入)
      - [使用 Builder 模式](#使用-builder-模式)
    - [24.4 错误处理策略](#244-错误处理策略)
      - [分层错误处理](#分层错误处理)
      - [错误转换层](#错误转换层)
    - [24.5 可测试性设计](#245-可测试性设计)
      - [使用 trait 隔离外部依赖](#使用-trait-隔离外部依赖)
      - [测试替身模式](#测试替身模式)
  - [25. 嵌入式与 no\_std 开发](#25-嵌入式与-no_std-开发)
    - [25.1 no\_std 环境](#251-no_std-环境)
      - [禁用标准库](#禁用标准库)
      - [Cargo.toml 配置](#cargotoml-配置-2)
      - [使用 core 替代 std](#使用-core-替代-std)
    - [25.2 嵌入式 HAL](#252-嵌入式-hal)
      - [使用 embedded-hal](#使用-embedded-hal)
      - [SPI 设备驱动](#spi-设备驱动)
    - [25.3 交叉编译](#253-交叉编译)
      - [配置目标](#配置目标)
      - [.cargo/config.toml](#cargoconfigtoml)
      - [内存布局脚本](#内存布局脚本)
    - [25.4 嵌入式异步](#254-嵌入式异步)
      - [使用 embassy](#使用-embassy)
      - [异步驱动](#异步驱动)
  - [26. WebAssembly 开发](#26-webassembly-开发)
    - [26.1 编译到 WASM](#261-编译到-wasm)
      - [安装工具链](#安装工具链)
      - [Cargo.toml 配置](#cargotoml-配置-3)
      - [基本编译](#基本编译)
    - [26.2 wasm-bindgen](#262-wasm-bindgen)
      - [导出函数](#导出函数-1)
      - [处理复杂数据](#处理复杂数据)
    - [26.3 与 JavaScript 交互](#263-与-javascript-交互)
      - [调用 JavaScript 函数](#调用-javascript-函数)
      - [处理 Promise](#处理-promise)
      - [事件处理](#事件处理)
    - [26.4 WASM 性能优化](#264-wasm-性能优化)
      - [内存管理](#内存管理)
      - [减少边界检查](#减少边界检查)
      - [使用 wee\_alloc 减小体积](#使用-wee_alloc-减小体积)
      - [构建优化](#构建优化)
  - [附录](#附录)
      - [官方资源](#官方资源)
      - [社区资源](#社区资源)

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

## 18. 过程宏深入

过程宏（Procedural Macros）是 Rust 元编程的核心，允许在编译时操作和生成代码。与声明式宏不同，过程宏操作的是语法树（AST），提供更强大的抽象能力。

### 18.1 声明式宏进阶

#### 复杂模式匹配

```rust
macro_rules! match_expr {
    ($expr:expr) => {
        match $expr {
            0 => println!("零"),
            1..=10 => println!("一到十"),
            n @ 11..=100 => println!("十一到一百: {}", n),
            _ => println!("其他"),
        }
    };
}

macro_rules! hash_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

let scores = hash_map! {
    "Alice" => 95,
    "Bob" => 87,
    "Carol" => 92,
};
```

#### 递归宏

```rust
macro_rules! tuple_impls {
    () => {};
    ($first:ident $($rest:ident)*) => {
        impl<$first, $($rest),*> Tuple for ($first, $($rest),*)
        where
            $first: Debug,
            $($rest: Debug),*
        {
            fn debug_all(&self) {
                let (first, $($rest),*) = self;
                print!("{:?}", first);
                $(
                    print!(", {:?}", $rest);
                )*
                println!();
            }
        }
        tuple_impls!($($rest)*);
    };
}

tuple_impls!(A B C D E);
```

#### 卫生性与标识符捕获

```rust
macro_rules! create_var {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

macro_rules! with_context {
    ($ctx:ident, $block:block) => {
        let $ctx = Context::new();
        let __guard = $ctx.enter();
        $block
    };
}
```

### 18.2 派生宏

派生宏是最常用的过程宏类型，用于自动实现 Trait。

#### 项目结构

```
my_derive_macro/
├── Cargo.toml
└── src/
    └── lib.rs
```

#### Cargo.toml 配置

```toml
[package]
name = "my_derive_macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full", "parsing", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"
```

#### 实现自定义派生宏

```rust
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);
    
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("只支持命名字段的结构体"),
        },
        _ => panic!("只支持结构体"),
    };
    
    let field_names: Vec<&Ident> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    
    let field_types: Vec<_> = fields
        .iter()
        .map(|f| &f.ty)
        .collect();
    
    let option_field_types: Vec<_> = fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            quote!(Option<#ty>)
        })
        .collect();
    
    let build_impl = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: None),*
                }
            }
        }
        
        pub struct #builder_name {
            #(#field_names: #option_field_types),*
        }
        
        impl #builder_name {
            #(
                pub fn #field_names(mut self, value: #field_types) -> Self {
                    self.#field_names = Some(value);
                    self
                }
            )*
            
            pub fn build(self) -> Result<#name, &'static str> {
                #(
                    let #field_names = self.#field_names
                        .ok_or(concat!("字段未设置: ", stringify!(#field_names)))?;
                )*
                Ok(#name {
                    #(#field_names),*
                })
            }
        }
    };
    
    build_impl.into()
}
```

#### 使用派生宏

```rust
use my_derive_macro::Builder;

#[derive(Builder)]
pub struct User {
    name: String,
    email: String,
    age: u32,
}

fn main() {
    let user = User::builder()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(30)
        .build()
        .unwrap();
}
```

#### 带属性的派生宏

```rust
use syn::Attribute;

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("只支持命名字段"),
        },
        _ => panic!("只支持结构体"),
    };
    
    let serialize_fields: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let name = f.ident.as_ref()?;
            let rename = f.attrs.iter()
                .find(|a| a.path().is_ident("serde"))
                .and_then(|a| {
                    a.parse_args::<Ident>().ok()
                });
            
            let key = rename
                .map(|r| r.to_string())
                .unwrap_or_else(|| name.to_string());
            
            Some(quote! {
                map.insert(#key.to_string(), self.#name.to_string());
            })
        })
        .collect();
    
    let expanded = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> std::collections::HashMap<String, String> {
                let mut map = std::collections::HashMap::new();
                #(#serialize_fields)*
                map
            }
        }
    };
    
    expanded.into()
}
```

### 18.3 属性宏

属性宏可以修改或增强它们所附加的项。

#### 实现属性宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Visibility, Signature};

#[proc_macro_attribute]
pub fn log_calls(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;
    
    let log_level = if attr.is_empty() {
        quote!(log::Level::Info)
    } else {
        let level: syn::Ident = syn::parse(attr).unwrap();
        quote!(log::Level::#level)
    };
    
    let expanded = quote! {
        #vis #sig {
            log::log!(#log_level, "调用函数: {}", stringify!(#fn_name));
            let __start = std::time::Instant::now();
            let __result = #block;
            log::log!(#log_level, "函数 {} 执行耗时: {:?}", stringify!(#fn_name), __start.elapsed());
            __result
        }
    };
    
    expanded.into()
}
```

#### 使用属性宏

```rust
use my_macro::log_calls;

#[log_calls]
fn process_data(data: &str) -> usize {
    data.len()
}

#[log_calls(debug)]
fn expensive_computation(n: u64) -> u64 {
    (1..=n).sum()
}
```

#### 条件编译属性

```rust
#[proc_macro_attribute]
pub fn cfg_async(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let sig = &input.sig;
    let is_async = sig.asyncness.is_some();
    
    if is_async {
        quote! {
            #[cfg(feature = "async")]
            #input
        }
        .into()
    } else {
        quote! {
            #[cfg(not(feature = "async"))]
            #input
        }
        .into()
    }
}
```

### 18.4 函数式过程宏

函数式过程宏看起来像函数调用，可以接受任意输入并生成代码。

#### 实现函数式宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Token, LitInt};

struct MatrixDef {
    rows: Vec<Vec<LitInt>>,
}

impl Parse for MatrixDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rows = Vec::new();
        
        while !input.is_empty() {
            let mut row = Vec::new();
            while !input.is_empty() {
                let val: LitInt = input.parse()?;
                row.push(val);
                
                if input.peek(Token![,]) && input.peek2(Token![,]) {
                    input.parse::<Token![,]>()?;
                    break;
                } else if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            rows.push(row);
        }
        
        Ok(MatrixDef { rows })
    }
}

#[proc_macro]
pub fn matrix(input: TokenStream) -> TokenStream {
    let matrix = parse_macro_input!(input as MatrixDef);
    
    let rows = matrix.rows.len();
    let cols = matrix.rows.first().map(|r| r.len()).unwrap_or(0);
    
    let elements: Vec<_> = matrix.rows
        .iter()
        .flat_map(|row| row.iter())
        .collect();
    
    let expanded = quote! {
        {
            let __matrix: [[i64; #cols]; #rows] = [
                #(
                    [#(#elements),*],
                )*
            ];
            Matrix::new(__matrix)
        }
    };
    
    expanded.into()
}
```

#### SQL 查询宏

```rust
use syn::LitStr;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let query = parse_macro_input!(input as LitStr);
    let query_str = query.value();
    
    let expanded = quote! {
        {
            let __query = #query_str;
            let __validated = validate_sql(__query);
            SqlQuery::new(__validated)
        }
    };
    
    expanded.into()
}

sql!("SELECT * FROM users WHERE id = ?");
```

#### 编译时计算宏

```rust
#[proc_macro]
pub fn const_hash(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    let hash = {
        let mut hash: u64 = 0;
        for byte in input_str.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    };
    
    quote!(#hash).into()
}

let id = const_hash!(user_session_token);
```

### 18.5 宏最佳实践

#### 错误处理

```rust
use proc_macro_error::{proc_macro_error, abort};

#[proc_macro]
#[proc_macro_error]
pub fn safe_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MyInput);
    
    if input.fields.is_empty() {
        abort!(input.span(), "至少需要一个字段");
    }
    
    for field in &input.fields {
        if field.name.to_string().starts_with("_") {
            abort!(
                field.name.span(),
                "字段名不能以下划线开头";
                help = "尝试使用其他名称";
            );
        }
    }
    
    quote!(/* ... */).into()
}
```

#### 文档生成

```rust
#[proc_macro_derive(Documented)]
pub fn derive_documented(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let docs = &input.attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .map(|a| a.tokens.to_string())
        .collect::<Vec<_>>();
    
    let expanded = quote! {
        impl Documented for #name {
            fn documentation() -> &'static str {
                concat!(#(#docs, "\n"),*)
            }
        }
    };
    
    expanded.into()
}
```

#### 性能考虑

```rust
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("fn");
        set.insert("let");
        set.insert("if");
        set
    };
}

#[proc_macro]
pub fn check_keywords(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let name = ident.to_string();
    
    if KEYWORDS.contains(name.as_str()) {
        return quote!(compile_error!("不能使用关键字作为标识符")).into();
    }
    
    quote!(#ident).into()
}
```

---

## 19. 异步编程底层原理

深入理解 Rust 异步编程的底层机制，包括 Future trait、Pin/Unpin 以及异步运行时的实现原理。

### 19.1 Future Trait 深入

#### Future Trait 定义

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

#### 手动实现 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });
        
        TimerFuture { shared_state }
    }
}
```

#### async/await 编译展开

```rust
async fn example() -> i32 {
    let a = compute_a().await;
    let b = compute_b().await;
    a + b
}

enum ExampleFuture {
    State0 { a_future: impl Future<Output = i32> },
    State1 { a: i32, b_future: impl Future<Output = i32> },
    State2 { result: i32 },
    Complete,
}

impl Future for ExampleFuture {
    type Output = i32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        loop {
            match &mut *self {
                Self::State0 { a_future } => {
                    match Pin::new(a_future).poll(cx) {
                        Poll::Ready(a) => {
                            *self = Self::State1 {
                                a,
                                b_future: compute_b(),
                            };
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                Self::State1 { a, b_future } => {
                    match Pin::new(b_future).poll(cx) {
                        Poll::Ready(b) => {
                            *self = Self::State2 { result: *a + b };
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                Self::State2 { result } => {
                    let r = *result;
                    *self = Self::Complete;
                    return Poll::Ready(r);
                }
                Self::Complete => panic!("poll after completion"),
            }
        }
    }
}
```

### 19.2 Pin 与 Unpin

#### 为什么需要 Pin

```rust
struct SelfReferential {
    data: String,
    pointer: *const str,
}

impl SelfReferential {
    fn new(s: String) -> Self {
        let mut this = Self {
            data: s,
            pointer: std::ptr::null(),
        };
        this.pointer = &this.data as *const str;
        this
    }
}

fn problematic() {
    let mut a = SelfReferential::new("hello".to_string());
    let mut b = SelfReferential::new("world".to_string());
    
    std::mem::swap(&mut a, &mut b);
    
    // a.pointer 现在指向 b.data，但 a.data 是 "hello"
    // 这导致悬垂指针！
}
```

#### Pin 类型

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct PinnedStruct {
    data: String,
    pointer: *const str,
    _pin: PhantomPinned,
}

impl PinnedStruct {
    fn new(s: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(Self {
            data: s,
            pointer: std::ptr::null(),
            _pin: PhantomPinned,
        });
        
        let self_ptr: *const str = &boxed.data;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).pointer = self_ptr;
        }
        
        boxed
    }
    
    fn get_data(self: Pin<&Self>) -> &str {
        &self.data
    }
    
    fn get_pointer_target(self: Pin<&Self>) -> &str {
        unsafe { &*self.pointer }
    }
}
```

#### Unpin Trait

```rust
pub trait Unpin {}

impl<T> Unpin for &mut T {}
impl<T> Unpin for Box<T> {}
impl<T> Unpin for Vec<T> {}

impl !Unpin for PhantomPinned {}

fn example_unpin<T: Unpin>(value: T) {
    let mut pinned = Box::pin(value);
    
    let mut_ref: &mut T = Pin::as_mut(&mut pinned).get_mut();
    *mut_ref = /* 新值 */;
}

fn example_not_unpin(value: PinnedStruct) {
    let mut pinned = Box::pin(value);
    
    // 编译错误！不能移动被 Pin 的值
    // let mut_ref: &mut PinnedStruct = Pin::as_mut(&mut pinned).get_mut();
}
```

#### Pin 安全性保证

```rust
use std::pin::Pin;

struct SafePinnedFuture<F> {
    future: F,
    _pin: PhantomPinned,
}

impl<F: Future> SafePinnedFuture<F> {
    pub fn new(future: F) -> Pin<Box<Self>> {
        Box::pin(Self {
            future,
            _pin: PhantomPinned,
        })
    }
    
    pub fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        unsafe {
            let future = &mut self.get_unchecked_mut().future;
            Pin::new_unchecked(future).poll(cx)
        }
    }
}
```

### 19.3 自定义 Future

#### 实现组合器

```rust
pub struct Map<F, T, U, Func> {
    future: F,
    func: Option<Func>,
    _marker: PhantomData<(T, U)>,
}

impl<F, T, U, Func> Future for Map<F, T, U, Func>
where
    F: Future<Output = T>,
    Func: FnOnce(T) -> U,
{
    type Output = U;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<U> {
        let this = unsafe { self.as_mut().get_unchecked_mut() };
        
        match Pin::new_unchecked(&mut this.future).poll(cx) {
            Poll::Ready(value) => {
                let func = this.func.take().unwrap();
                Poll::Ready(func(value))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<F: Future> FutureExt for F {
    fn map<T, Func>(self, func: Func) -> Map<Self, F::Output, T, Func>
    where
        Func: FnOnce(F::Output) -> T,
    {
        Map {
            future: self,
            func: Some(func),
            _marker: PhantomData,
        }
    }
}
```

#### 实现 select

```rust
pub struct Select<A, B> {
    a: Option<A>,
    b: Option<B>,
}

impl<A, B> Future for Select<A, B>
where
    A: Future,
    B: Future,
{
    type Output = Either<(A::Output, B), (B::Output, A)>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.as_mut().get_unchecked_mut() };
        
        if let Some(a) = &mut this.a {
            if let Poll::Ready(val) = unsafe { Pin::new_unchecked(a).poll(cx) } {
                let b = this.b.take().unwrap();
                return Poll::Ready(Either::Left((val, b)));
            }
        }
        
        if let Some(b) = &mut this.b {
            if let Poll::Ready(val) = unsafe { Pin::new_unchecked(b).poll(cx) } {
                let a = this.a.take().unwrap();
                return Poll::Ready(Either::Right((val, a)));
            }
        }
        
        Poll::Pending
    }
}

pub async fn select<A, B>(a: A, b: B) -> Either<(A::Output, B), (B::Output, A)>
where
    A: Future,
    B: Future,
{
    Select { a: Some(a), b: Some(b) }.await
}
```

### 19.4 异步运行时原理

#### 简单执行器实现

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::task::{Waker, RawWaker, RawWakerVTable};

pub struct Executor {
    ready_queue: Arc<Mutex<VecDeque<Task>>>,
}

struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Task {
            future: Box::pin(future),
        };
        self.ready_queue.lock().unwrap().push_back(task);
    }
    
    pub fn run(&self) {
        loop {
            let task = self.ready_queue.lock().unwrap().pop_front();
            
            match task {
                Some(mut task) => {
                    let waker = self.create_waker();
                    let mut cx = Context::from_waker(&waker);
                    
                    match task.future.as_mut().poll(&mut cx) {
                        Poll::Ready(()) => {}
                        Poll::Pending => {
                            self.ready_queue.lock().unwrap().push_back(task);
                        }
                    }
                }
                None => break,
            }
        }
    }
    
    fn create_waker(&self) -> Waker {
        let ready_queue = self.ready_queue.clone();
        
        fn clone(ptr: *const ()) -> RawWaker {
            let ready_queue = unsafe { Arc::from_raw(ptr as *const Mutex<VecDeque<Task>>) };
            let cloned = ready_queue.clone();
            std::mem::forget(ready_queue);
            std::mem::forget(cloned.clone());
            RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
        }
        
        fn wake(ptr: *const ()) {
            unsafe { Arc::from_raw(ptr as *const Mutex<VecDeque<Task>>) };
        }
        
        fn wake_by_ref(ptr: *const ()) {
            unsafe { Arc::from_raw(ptr as *const Mutex<VecDeque<Task>>) };
        }
        
        fn drop(ptr: *const ()) {
            unsafe { Arc::from_raw(ptr as *const Mutex<VecDeque<Task>>) };
        }
        
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        
        let ptr = Arc::into_raw(ready_queue);
        unsafe { Waker::from_raw(RawWaker::new(ptr as *const (), &VTABLE)) }
    }
}
```

#### 工作窃取调度器

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::deque::{Injector, Stealer, Worker};

pub struct WorkStealingExecutor {
    global_queue: Arc<Injector<Task>>,
    workers: Vec<WorkerHandle>,
    num_workers: usize,
}

struct WorkerHandle {
    local_queue: Worker<Task>,
    stealer: Stealer<Task>,
    thread: Option<thread::JoinHandle<()>>,
}

impl WorkStealingExecutor {
    pub fn new(num_workers: usize) -> Self {
        let global_queue = Arc::new(Injector::new());
        let mut workers = Vec::with_capacity(num_workers);
        
        for _ in 0..num_workers {
            let local_queue = Worker::new_fifo();
            let stealer = local_queue.stealer();
            workers.push(WorkerHandle {
                local_queue,
                stealer,
                thread: None,
            });
        }
        
        Self {
            global_queue,
            workers,
            num_workers,
        }
    }
    
    pub fn spawn(&self, task: Task) {
        self.global_queue.push(task);
    }
    
    fn find_task(&self, worker: &Worker<Task>) -> Option<Task> {
        worker.pop().or_else(|| {
            std::iter::repeat_with(|| {
                self.global_queue
                    .steal_batch_and_pop(worker)
                    .or_else(|| {
                        self.workers
                            .iter()
                            .map(|w| w.stealer.steal())
                            .collect()
                    })
            })
            .find(|s| !s.is_retry())
            .map(|s| s.success())
        })
    }
}
```

---

## 20. FFI 与跨语言互操作

Rust 提供了强大的 FFI（Foreign Function Interface）支持，可以与 C/C++ 等语言无缝交互。

### 20.1 调用 C 代码

#### 基本调用

```rust
use std::os::raw::{c_int, c_char};

extern "C" {
    fn abs(x: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    unsafe {
        let result = abs(-42);
        println!("abs(-42) = {}", result);
        
        let s = b"hello\0";
        let len = strlen(s.as_ptr() as *const c_char);
        println!("strlen(\"hello\") = {}", len);
    }
}
```

#### 链接 C 库

```toml
[build-dependencies]
cc = "1.0"

[dependencies]
libc = "0.2"
```

```rust
fn main() {
    cc::Build::new()
        .file("src/native.c")
        .compile("native");
}
```

```c
#include <stdint.h>

int64_t add_numbers(int64_t a, int64_t b) {
    return a + b;
}
```

```rust
extern "C" {
    fn add_numbers(a: i64, b: i64) -> i64;
}

fn add(a: i64, b: i64) -> i64 {
    unsafe { add_numbers(a, b) }
}
```

#### 回调函数

```rust
type Callback = extern "C" fn(i32, i32) -> i32;

extern "C" {
    fn register_callback(cb: Callback);
    fn trigger_callback(a: i32, b: i32) -> i32;
}

extern "C" fn my_callback(a: i32, b: i32) -> i32 {
    println!("回调被调用: a={}, b={}", a, b);
    a + b
}

fn main() {
    unsafe {
        register_callback(my_callback);
        let result = trigger_callback(10, 20);
        println!("结果: {}", result);
    }
}
```

### 20.2 为 C 提供接口

#### 导出函数

```rust
use std::slice;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_string_length(s: *const c_char) -> usize {
    unsafe {
        if s.is_null() {
            return 0;
        }
        let c_str = std::ffi::CStr::from_ptr(s);
        c_str.to_bytes().len()
    }
}

#[no_mangle]
pub extern "C" fn rust_create_string() -> *mut c_char {
    let s = std::ffi::CString::new("Hello from Rust").unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = std::ffi::CString::from_raw(s);
        }
    }
}
```

#### 导出结构体

```rust
use std::os::raw::c_char;

#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

#[no_mangle]
pub extern "C" fn create_point(x: f64, y: f64) -> Point {
    Point { x, y }
}

#[no_mangle]
pub extern "C" fn point_distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}

#[no_mangle]
pub extern "C" fn rectangle_area(rect: &Rectangle) -> f64 {
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.bottom_right.y - rect.top_left.y;
    width * height
}
```

#### 不透明类型模式

```rust
pub struct Database {
    connections: Vec<Connection>,
    config: Config,
}

#[repr(C)]
pub struct DatabaseHandle {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn database_new(config_json: *const c_char) -> *mut DatabaseHandle {
    unsafe {
        let config_str = std::ffi::CStr::from_ptr(config_json);
        let config: Config = match serde_json::from_str(config_str.to_str().unwrap()) {
            Ok(c) => c,
            Err(_) => return std::ptr::null_mut(),
        };
        
        let db = Box::new(Database::new(config));
        Box::into_raw(db) as *mut DatabaseHandle
    }
}

#[no_mangle]
pub extern "C" fn database_free(handle: *mut DatabaseHandle) {
    unsafe {
        if !handle.is_null() {
            let _ = Box::from_raw(handle as *mut Database);
        }
    }
}

#[no_mangle]
pub extern "C" fn database_query(
    handle: *const DatabaseHandle,
    query: *const c_char,
) -> *mut c_char {
    unsafe {
        let db = &*(handle as *const Database);
        let query_str = std::ffi::CStr::from_ptr(query);
        
        match db.execute(query_str.to_str().unwrap()) {
            Ok(result) => {
                let c_result = std::ffi::CString::new(result).unwrap();
                c_result.into_raw()
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}
```

### 20.3 与 C++ 交互

#### C++ ABI 兼容

```rust
#[repr(C)]
pub struct CppVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

extern "C" {
    #[link_name = "_ZN3Math9normalizeERK8Vector3"]
    fn cpp_normalize(v: &CppVector3) -> CppVector3;
    
    #[link_name = "_ZN3Math3dotERK8Vector3S2_"]
    fn cpp_dot(a: &CppVector3, b: &CppVector3) -> f32;
}

pub fn normalize(v: CppVector3) -> CppVector3 {
    unsafe { cpp_normalize(&v) }
}

pub fn dot(a: CppVector3, b: CppVector3) -> f32 {
    unsafe { cpp_dot(&a, &b) }
}
```

#### RAII 包装

```rust
pub struct CppString {
    ptr: *mut std::ffi::c_void,
}

impl CppString {
    pub fn new(s: &str) -> Self {
        let c_str = std::ffi::CString::new(s).unwrap();
        let ptr = unsafe { cpp_string_new(c_str.as_ptr()) };
        Self { ptr }
    }
    
    pub fn as_str(&self) -> &str {
        unsafe {
            let ptr = cpp_string_c_str(self.ptr);
            let c_str = std::ffi::CStr::from_ptr(ptr);
            c_str.to_str().unwrap()
        }
    }
}

impl Drop for CppString {
    fn drop(&mut self) {
        unsafe {
            cpp_string_free(self.ptr);
        }
    }
}

extern "C" {
    fn cpp_string_new(s: *const c_char) -> *mut std::ffi::c_void;
    fn cpp_string_c_str(s: *mut std::ffi::c_void) -> *const c_char;
    fn cpp_string_free(s: *mut std::ffi::c_void);
}
```

### 20.4 bindgen 与 cbindgen

#### 使用 bindgen 自动生成绑定

```toml
[build-dependencies]
bindgen = "0.69"
```

```rust
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=mylib");
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("无法生成绑定");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("无法写入绑定");
}
```

```c
#include <mylib.h>
```

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

#### 使用 cbindgen 生成 C 头文件

```toml
[build-dependencies]
cbindgen = "0.26"
```

```rust
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("无法生成头文件")
        .write_to_file("target/mylib.h");
}
```

#### cbindgen.toml 配置

```toml
[parse]
parse_deps = true
include = ["my_crate"]

[export]
include = ["Point", "Rectangle", "database_new"]

[fn]
sort_by = "Name"

[struct]
rename_fields = "CamelCase"

[enum]
rename_variants = "ScreamingSnakeCase"
```

### 20.5 安全封装模式

#### 类型安全包装

```rust
use std::marker::PhantomData;

pub struct Owned;
pub struct Borrowed<'a>(PhantomData<&'a ()>);

pub struct CString<Ownership = Owned> {
    ptr: *mut c_char,
    _ownership: PhantomData<Ownership>,
}

impl CString<Owned> {
    pub fn new(s: &str) -> Result<Self, std::ffi::NulError> {
        let c_string = std::ffi::CString::new(s)?;
        Ok(Self {
            ptr: c_string.into_raw(),
            _ownership: PhantomData,
        })
    }
    
    pub fn as_ptr(&self) -> *const c_char {
        self.ptr
    }
    
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.ptr
    }
}

impl<'a> CString<Borrowed<'a>> {
    pub unsafe fn from_ptr(ptr: *const c_char) -> Self {
        Self {
            ptr: ptr as *mut c_char,
            _ownership: PhantomData,
        }
    }
    
    pub fn to_str(&self) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(self.ptr);
            c_str.to_str()
        }
    }
}

impl Drop for CString<Owned> {
    fn drop(&mut self) {
        unsafe {
            let _ = std::ffi::CString::from_raw(self.ptr);
        }
    }
}
```

#### 资源管理 RAII

```rust
pub struct FileDescriptor {
    fd: i32,
}

impl FileDescriptor {
    pub fn open(path: &str, flags: i32) -> std::io::Result<Self> {
        let c_path = std::ffi::CString::new(path)?;
        let fd = unsafe { libc::open(c_path.as_ptr(), flags) };
        
        if fd < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(Self { fd })
        }
    }
    
    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = unsafe {
            libc::read(
                self.fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len(),
            )
        };
        
        if n < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }
    
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = unsafe {
            libc::write(
                self.fd,
                buf.as_ptr() as *const libc::c_void,
                buf.len(),
            )
        };
        
        if n < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(n as usize)
        }
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
```

---

## 21. 类型系统进阶

Rust 的类型系统非常强大，支持高级类型级编程和抽象。

### 21.1 泛型关联类型 (GATs)

#### 基本用法

```rust
trait Container {
    type Item<'a> where Self: 'a;
    
    fn get<'a>(&'a self) -> Self::Item<'a>;
}

struct VecContainer<T> {
    data: Vec<T>,
}

impl<T: Clone> Container for VecContainer<T> {
    type Item<'a> = &'a T where Self: 'a;
    
    fn get<'a>(&'a self) -> Self::Item<'a> {
        self.data.first().unwrap()
    }
}

struct OptionContainer<T> {
    data: Option<T>,
}

impl<T> Container for OptionContainer<T> {
    type Item<'a> = Option<&'a T> where Self: 'a;
    
    fn get<'a>(&'a self) -> Self::Item<'a> {
        self.data.as_ref()
    }
}
```

#### 实现借用迭代器

```rust
trait Iterable {
    type Item<'a> where Self: 'a;
    type Iterator<'a>: Iterator<Item = Self::Item<'a>> where Self: 'a;
    
    fn iter(&self) -> Self::Iterator<'_>;
}

impl<T> Iterable for Vec<T> {
    type Item<'a> = &'a T where Self: 'a;
    type Iterator<'a> = std::slice::Iter<'a, T> where Self: 'a;
    
    fn iter(&self) -> Self::Iterator<'_> {
        self.as_slice().iter()
    }
}

impl<T> Iterable for VecDeque<T> {
    type Item<'a> = &'a T where Self: 'a;
    type Iterator<'a> = std::collections::vec_deque::Iter<'a, T> where Self: 'a;
    
    fn iter(&self) -> Self::Iterator<'_> {
        self.iter()
    }
}
```

#### 数据库连接池模式

```rust
trait DatabasePool {
    type Connection<'a> where Self: 'a;
    type Error: std::error::Error;
    
    fn get<'a>(&'a self) -> Result<Self::Connection<'a>, Self::Error>;
    
    async fn get_async<'a>(&'a self) -> Result<Self::Connection<'a>, Self::Error>;
}

struct PostgresPool {
    url: String,
    max_connections: usize,
}

struct PostgresConnection<'a> {
    pool: &'a PostgresPool,
    id: u64,
}

impl DatabasePool for PostgresPool {
    type Connection<'a> = PostgresConnection<'a> where Self: 'a;
    type Error = PostgresError;
    
    fn get<'a>(&'a self) -> Result<Self::Connection<'a>, Self::Error> {
        Ok(PostgresConnection {
            pool: self,
            id: rand::random(),
        })
    }
    
    async fn get_async<'a>(&'a self) -> Result<Self::Connection<'a>, Self::Error> {
        self.get()
    }
}
```

### 21.2 关联类型默认值

#### 带默认值的关联类型

```rust
trait Service {
    type Request;
    type Response;
    type Error = std::io::Error;
    type Future<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'a>>
    where
        Self: 'a;
    
    fn call(&self, req: Self::Request) -> Self::Future<'_>;
}

struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;
    
    fn call(&self, req: Self::Request) -> Self::Future<'_> {
        Box::pin(async move { Ok(req) })
    }
}

struct LoggingService<S> {
    inner: S,
}

impl<S: Service> Service for LoggingService<S> {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future<'a> = S::Future<'a> where Self: 'a;
    
    fn call(&self, req: Self::Request) -> Self::Future<'_> {
        println!("收到请求");
        self.inner.call(req)
    }
}
```

### 21.3 类型级编程

#### 类型级自然数

```rust
struct Zero;
struct Succ<N>(std::marker::PhantomData<N>);

type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;

trait Nat {
    const VALUE: usize;
}

impl Nat for Zero {
    const VALUE: usize = 0;
}

impl<N: Nat> Nat for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

fn print_nat<N: Nat>() {
    println!("值: {}", N::VALUE);
}

print_nat::<Zero>();
print_nat::<Three>();
```

#### 类型级列表

```rust
struct Nil;
struct Cons<H, T>(std::marker::PhantomData<(H, T)>);

trait HList {
    type Head;
    type Tail;
    const LEN: usize;
}

impl HList for Nil {
    type Head = ();
    type Tail = Nil;
    const LEN: usize = 0;
}

impl<H, T: HList> HList for Cons<H, T> {
    type Head = H;
    type Tail = T;
    const LEN: usize = 1 + T::LEN;
}

type MyList = Cons<i32, Cons<String, Cons<bool, Nil>>>;
```

#### 类型级布尔运算

```rust
struct True;
struct False;

trait Bool {
    type Not: Bool;
    type And<B: Bool>: Bool;
    type Or<B: Bool>: Bool;
}

impl Bool for True {
    type Not = False;
    type And<B: Bool> = B;
    type Or<B: Bool> = True;
}

impl Bool for False {
    type Not = True;
    type And<B: Bool> = False;
    type Or<B: Bool> = B;
}

type NotTrue = <True as Bool>::Not;
type TrueAndFalse = <True as Bool>::And<False>;
```

#### 类型级条件

```rust
trait If<Cond: Bool, Then, Else> {
    type Output;
}

impl<Then, Else> If<True, Then, Else> for () {
    type Output = Then;
}

impl<Then, Else> If<False, Then, Else> for () {
    type Output = Else;
}

type Result = <() as If<True, i32, String>>::Output;
```

### 21.4 高阶类型模拟

#### Functor 模拟

```rust
trait Functor<'a, A> {
    type Target<B>;
    
    fn map<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B;
}

impl<'a, A> Functor<'a, A> for Option<A> {
    type Target<B> = Option<B>;
    
    fn map<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A, E> Functor<'a, A> for Result<A, E> {
    type Target<B> = Result<B, E>;
    
    fn map<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}
```

#### Monad 模拟

```rust
trait Monad<'a, A>: Functor<'a, A> {
    fn pure(a: A) -> Self;
    
    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>;
}

impl<'a, A> Monad<'a, A> for Option<A> {
    fn pure(a: A) -> Self {
        Some(a)
    }
    
    fn bind<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> Self::Target<B>,
    {
        self.and_then(f)
    }
}

fn example_monad() {
    let result = Some(5)
        .bind(|x| Some(x * 2))
        .bind(|x| Some(x + 1));
    assert_eq!(result, Some(11));
}
```

### 21.5 幻影类型与类型状态

#### 幻影类型

```rust
use std::marker::PhantomData;

struct Meters(f64);
struct Feet(f64);

struct Distance<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl Distance<Meters> {
    fn from_meters(m: f64) -> Self {
        Self {
            value: m,
            _unit: PhantomData,
        }
    }
    
    fn to_feet(&self) -> Distance<Feet> {
        Distance {
            value: self.value * 3.28084,
            _unit: PhantomData,
        }
    }
}

impl Distance<Feet> {
    fn from_feet(f: f64) -> Self {
        Self {
            value: f,
            _unit: PhantomData,
        }
    }
    
    fn to_meters(&self) -> Distance<Meters> {
        Distance {
            value: self.value / 3.28084,
            _unit: PhantomData,
        }
    }
}

impl<Unit> std::ops::Add for Distance<Unit> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
            _unit: PhantomData,
        }
    }
}

let d1 = Distance::<Meters>::from_meters(100.0);
let d2 = Distance::<Meters>::from_meters(200.0);
let total = d1 + d2;

let d3 = Distance::<Feet>::from_feet(100.0);
// let error = d1 + d3; // 编译错误！类型不匹配
```

#### 类型状态模式

```rust
struct Uninitialized;
struct Initialized;
struct Running;
struct Stopped;

pub struct StateMachine<State> {
    data: Vec<u8>,
    _state: PhantomData<State>,
}

impl StateMachine<Uninitialized> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _state: PhantomData,
        }
    }
    
    pub fn initialize(mut self, initial_data: Vec<u8>) -> StateMachine<Initialized> {
        self.data = initial_data;
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl StateMachine<Initialized> {
    pub fn start(self) -> StateMachine<Running> {
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl StateMachine<Running> {
    pub fn process(&mut self) -> &mut [u8] {
        for byte in &mut self.data {
            *byte = byte.wrapping_add(1);
        }
        &mut self.data
    }
    
    pub fn stop(self) -> StateMachine<Stopped> {
        StateMachine {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl StateMachine<Stopped> {
    pub fn reset(self) -> StateMachine<Uninitialized> {
        StateMachine {
            data: Vec::new(),
            _state: PhantomData,
        }
    }
    
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

let machine = StateMachine::new();
let machine = machine.initialize(vec![1, 2, 3]);
let mut machine = machine.start();
machine.process();
let machine = machine.stop();
let data = machine.get_data();
```

#### 连接状态机

```rust
struct Disconnected;
struct Connecting;
struct Connected;
struct Error;

pub struct Connection<State> {
    stream: Option<TcpStream>,
    error: Option<std::io::Error>,
    _state: PhantomData<State>,
}

impl Connection<Disconnected> {
    pub fn new() -> Self {
        Self {
            stream: None,
            error: None,
            _state: PhantomData,
        }
    }
    
    pub async fn connect(self, addr: &str) -> Result<Connection<Connected>, Connection<Error>> {
        match TcpStream::connect(addr).await {
            Ok(stream) => Ok(Connection {
                stream: Some(stream),
                error: None,
                _state: PhantomData,
            }),
            Err(e) => Err(Connection {
                stream: None,
                error: Some(e),
                _state: PhantomData,
            }),
        }
    }
}

impl Connection<Connected> {
    pub async fn send(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.stream.as_mut().unwrap().write_all(data).await
    }
    
    pub async fn receive(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.as_mut().unwrap().read(buf).await
    }
    
    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection {
            stream: None,
            error: None,
            _state: PhantomData,
        }
    }
}

impl Connection<Error> {
    pub fn error(&self) -> &std::io::Error {
        self.error.as_ref().unwrap()
    }
    
    pub fn retry(self) -> Connection<Disconnected> {
        Connection {
            stream: None,
            error: None,
            _state: PhantomData,
        }
    }
}
```

---

## 22. 性能优化与基准测试

性能优化是 Rust 开发中的重要环节，本章介绍性能分析工具、基准测试方法和优化技巧。

### 22.1 性能分析工具

#### 使用 perf 进行性能分析

```bash
# 编译带调试信息的发布版本
RUSTFLAGS="-g" cargo build --release

# 运行 perf
perf record -g ./target/release/my_program
perf report

# 生成火焰图
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

#### 使用 cargo-flamegraph

```bash
cargo install flamegraph

# 直接生成火焰图
cargo flamegraph

# 分析特定命令
cargo flamegraph --root -- my_program --arg1 value
```

#### 使用 valgrind 分析内存

```bash
# 内存泄漏检测
valgrind --leak-check=full ./target/release/my_program

# 缓存分析
valgrind --tool=cachegrind ./target/release/my_program

# 堆分析
valgrind --tool=massif ./target/release/my_program
```

#### 使用 pprof

```rust
use pprof::protos::Message;

#[cfg(feature = "pprof")]
fn start_pprof() -> pprof::ProfilerGuard<'static> {
    pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap()
}

#[cfg(feature = "pprof")]
fn stop_pprof(guard: pprof::ProfilerGuard) {
    if let Ok(report) = guard.report().build() {
        let profile = report.pprof().unwrap();
        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        std::fs::write("profile.pb", content).unwrap();
    }
}
```

### 22.2 基准测试

#### 使用 criterion.rs

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    
    let mut group = c.benchmark_group("fibonacci");
    for size in [10, 20, 30, 40].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &s| {
            b.iter(|| fibonacci(black_box(s)));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

#### 比较不同实现

```rust
fn bench_string_operations(c: &mut Criterion) {
    let data = "hello world ".repeat(1000);
    
    let mut group = c.benchmark_group("string_concat");
    
    group.bench_function("push_str", |b| {
        b.iter(|| {
            let mut s = String::new();
            for _ in 0..100 {
                s.push_str(&data);
            }
            black_box(s)
        })
    });
    
    group.bench_function("format!", |b| {
        b.iter(|| {
            let mut s = String::new();
            for _ in 0..100 {
                s = format!("{}{}", s, data);
            }
            black_box(s)
        })
    });
    
    group.bench_function("extend", |b| {
        b.iter(|| {
            let mut s = String::new();
            for _ in 0..100 {
                s.extend(data.chars());
            }
            black_box(s)
        })
    });
    
    group.finish();
}
```

#### 异步基准测试

```rust
use tokio::runtime::Runtime;

fn bench_async_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("async_task", |b| {
        b.to_async(&rt).iter(|| async {
            tokio::time::sleep(std::time::Duration::from_micros(10)).await;
            42
        })
    });
}
```

### 22.3 编译优化选项

#### Cargo.toml 优化配置

```toml
[profile.dev]
opt-level = 0
debug = true
split-debuginfo = "packed"
debug-assertions = true
overflow-checks = true
lto = false
panic = "unwind"
incremental = true
codegen-units = 256

[profile.release]
opt-level = 3
debug = false
split-debuginfo = "packed"
debug-assertions = false
overflow-checks = false
lto = "fat"
panic = "abort"
incremental = false
codegen-units = 1
strip = true

[profile.release-with-debug]
inherits = "release"
debug = true
strip = false

[profile.release-size]
inherits = "release"
opt-level = "z"
lto = "thin"

[profile.release-speed]
inherits = "release"
opt-level = 3
lto = "fat"
codegen-units = 1
```

#### 链接时优化 (LTO)

```toml
[profile.release]
lto = "fat"
codegen-units = 1
```

```bash
# 使用 LTO 编译
RUSTFLAGS="-C lto=fat" cargo build --release
```

#### PGO (Profile-Guided Optimization)

```bash
# 步骤 1: 编译带插桩的版本
RUSTFLAGS="-C profile-generate=/path/to/pgo-data" \
    cargo build --release

# 步骤 2: 运行典型工作负载
./target/release/my_program

# 步骤 3: 使用收集的数据重新编译
RUSTFLAGS="-C profile-use=/path/to/pgo-data" \
    cargo build --release
```

### 22.4 内存布局优化

#### 使用 #[repr] 控制布局

```rust
#[repr(C)]
struct CLayout {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(C, packed)]
struct PackedLayout {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(align(16))]
struct AlignedLayout {
    data: [u8; 16],
}

#[repr(u8)]
enum SmallEnum {
    A = 0,
    B = 1,
    C = 2,
}
```

#### 字段排序优化

```rust
use std::mem::size_of;

struct Unoptimized {
    a: u8,
    b: u64,
    c: u8,
    d: u64,
    e: u8,
}

struct Optimized {
    b: u64,
    d: u64,
    a: u8,
    c: u8,
    e: u8,
}

assert_eq!(size_of::<Unoptimized>(), 32);
assert_eq!(size_of::<Optimized>(), 24);
```

#### 使用小型数组

```rust
use smallvec::SmallVec;

type SmallBuffer = SmallVec<[u8; 64]>;

fn process_data() -> SmallBuffer {
    let mut buf = SmallBuffer::new();
    for i in 0..50 {
        buf.push(i);
    }
    buf
}
```

#### 使用小型字符串

```rust
use smartstring::SmartString;

fn process_strings() {
    let s: SmartString<smartstring::LazyCompact> = "hello world".into();
    println!("{}", s);
}

use smallstr::SmallString;

type ShortString = SmallString<[u8; 23]>;
```

### 22.5 SIMD 向量化

#### 使用 std::simd

```rust
#![feature(portable_simd)]

use std::simd::*;

fn sum_simd(data: &[f32]) -> f32 {
    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();
    
    let mut sum = Simd::splat(0.0f32);
    
    for chunk in chunks {
        let vec = Simd::from_slice(chunk);
        sum += vec;
    }
    
    let mut total = sum.reduce_sum();
    for &val in remainder {
        total += val;
    }
    
    total
}
```

#### 使用 packed_simd_2

```rust
use packed_simd::*;

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    let chunks = a.chunks_exact(8);
    let remainder = chunks.remainder();
    
    let mut sum = f32x8::splat(0.0);
    
    for (chunk_a, chunk_b) in chunks.zip(b.chunks_exact(8)) {
        let va = f32x8::from_slice_unaligned(chunk_a);
        let vb = f32x8::from_slice_unaligned(chunk_b);
        sum += va * vb;
    }
    
    let mut total = sum.sum();
    for i in 0..remainder.len() {
        total += a[i] * b[i];
    }
    
    total
}
```

#### 手动向量化提示

```rust
fn vectorized_add(a: &[i32], b: &[i32], c: &mut [i32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());
    
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
}

#[target_feature(enable = "avx2")]
#[cfg(target_arch = "x86_64")]
unsafe fn vectorized_add_avx2(a: &[i32], b: &[i32], c: &mut [i32]) {
    use std::arch::x86_64::*;
    
    let chunks = a.chunks_exact(8);
    let remainder = chunks.remainder();
    
    for (i, (chunk_a, chunk_b)) in chunks.zip(b.chunks_exact(8)).enumerate() {
        let va = _mm256_loadu_si256(chunk_a.as_ptr() as *const _);
        let vb = _mm256_loadu_si256(chunk_b.as_ptr() as *const _);
        let vc = _mm256_add_epi32(va, vb);
        _mm256_storeu_si256(c[i * 8..].as_mut_ptr() as *mut _, vc);
    }
    
    for i in 0..remainder.len() {
        c[a.len() - remainder.len() + i] = a[i] + b[i];
    }
}
```

---

## 23. 生态系统与常用 Crate

Rust 生态系统中有许多高质量的第三方库，本章介绍最常用的 crate 及其最佳实践。

### 23.1 序列化：serde

#### 基本用法

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
        is_active: true,
        phone: None,
    };
    
    let json = serde_json::to_string_pretty(&user)?;
    println!("{}", json);
    
    let parsed: User = serde_json::from_str(&json)?;
    println!("{:?}", parsed);
    
    Ok(())
}
```

#### 自定义序列化

```rust
use serde::{Serializer, Deserializer, Serialize, Deserialize};

#[derive(Debug)]
struct PhoneNumber(String);

impl Serialize for PhoneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("+86-{}", self.0))
    }
}

impl<'de> Deserialize<'de> for PhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let number = s.strip_prefix("+86-")
            .ok_or_else(|| serde::de::Error::custom("invalid phone format"))?;
        Ok(PhoneNumber(number.to_string()))
    }
}
```

#### 枚举序列化

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum Message {
    Text(String),
    Number(i32),
    Binary(Vec<u8>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FlexibleValue {
    String(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Event {
    #[serde(rename = "user_created")]
    UserCreated { id: u64, name: String },
    #[serde(rename = "user_deleted")]
    UserDeleted { id: u64 },
}
```

### 23.2 错误处理：thiserror 与 anyhow

#### 使用 thiserror 定义错误

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("数据连接失败: {0}")]
    ConnectionError(String),
    
    #[error("记录未找到: {id}")]
    NotFound { id: u64 },
    
    #[error("无效输入: {field} = {value}")]
    InvalidInput { field: String, value: String },
    
    #[error("IO 错误")]
    Io(#[from] std::io::Error),
    
    #[error("JSON 解析错误")]
    Json(#[from] serde_json::Error),
    
    #[error("未知错误")]
    Unknown,
}

fn find_user(id: u64) -> Result<User, DataStoreError> {
    if id == 0 {
        return Err(DataStoreError::NotFound { id });
    }
    Ok(User { id, name: "Alice".to_string() })
}
```

#### 使用 anyhow 处理应用错误

```rust
use anyhow::{Context, Result, anyhow, bail};

fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取配置文件: {}", path))?;
    
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("配置文件格式错误: {}", path))?;
    
    if config.port == 0 {
        bail!("端口号不能为 0");
    }
    
    Ok(config)
}

fn main() -> Result<()> {
    let config = read_config("config.toml")?;
    
    if config.debug {
        println!("调试模式已启用");
    }
    
    Ok(())
}
```

#### 组合使用

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("认证失败")]
    Unauthorized,
    #[error("资源未找到: {0}")]
    NotFound(String),
    #[error("内部错误: {0}")]
    Internal(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

pub fn handle_request() -> Result<HttpResponse, ApiError> {
    let data = load_data()
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok(data))
}
```

### 23.3 异步运行时：tokio 深入

#### 运行时配置

```rust
use tokio::runtime::{self, Runtime};

fn create_runtime() -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .thread_name("my-tokio-worker")
        .thread_stack_size(3 * 1024 * 1024)
        .max_blocking_threads(32)
        .build()
        .unwrap()
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    println!("多线程运行时");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("单线程运行时");
}
```

#### 任务管理

```rust
use tokio::task::{self, JoinSet, yield_now};

async fn task_management() {
    let handle = task::spawn(async {
        println!("后台任务运行中");
        42
    });
    
    let result = handle.await.unwrap();
    println!("结果: {}", result);
    
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            i * 2
        });
    }
    
    while let Some(res) = set.join_next().await {
        println!("任务结果: {:?}", res);
    }
    
    task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("阻塞任务完成");
    }).await.unwrap();
}
```

#### 同步原语

```rust
use tokio::sync::{Mutex, RwLock, Semaphore, mpsc, broadcast};

async fn sync_primitives() {
    let mutex = Mutex::new(0);
    {
        let mut guard = mutex.lock().await;
        *guard += 1;
    }
    
    let rwlock = RwLock::new(0);
    {
        let read_guard = rwlock.read().await;
        println!("读取: {}", *read_guard);
    }
    {
        let mut write_guard = rwlock.write().await;
        *write_guard += 1;
    }
    
    let semaphore = Semaphore::new(3);
    let _permit = semaphore.acquire().await.unwrap();
    
    let (tx, mut rx) = mpsc::channel(100);
    tx.send("hello".to_string()).await.unwrap();
    let msg = rx.recv().await.unwrap();
    
    let (tx, mut rx) = broadcast::channel(16);
    tx.send("broadcast message").unwrap();
    let msg = rx.recv().await.unwrap();
}
```

### 23.4 数据库：sqlx 与 diesel

#### 使用 sqlx

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::{query, query_as};

#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn database_example() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/db").await?;
    
    let users = query_as::<_, User>(
        "SELECT id, name, email FROM users WHERE id > $1"
    )
    .bind(0)
    .fetch_all(&pool)
    .await?;
    
    let user = query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind("Alice")
    .bind("alice@example.com")
    .fetch_one(&pool)
    .await?;
    
    let count: (i64,) = query("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;
    
    Ok(())
}
```

#### 使用 diesel

```rust
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn diesel_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = SqliteConnection::establish("database.db")?;
    
    let users = users::table
        .filter(users::id.gt(0))
        .load::<User>(&mut conn)?;
    
    let new_user = User {
        id: 0,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };
    
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)?;
    
    diesel::update(users::table.find(1))
        .set(users::name.eq("Updated Name"))
        .execute(&mut conn)?;
    
    diesel::delete(users::table.filter(users::id.eq(1)))
        .execute(&mut conn)?;
    
    Ok(())
}
```

### 23.5 Web 框架：axum 与 actix-web

#### 使用 axum

```rust
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::{Path, State},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    db: Database,
}

async fn get_user(Path(id): Path<u64>) -> Result<Json<User>, StatusCode> {
    match find_user(id).await {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .with_state(AppState { db: Database::new() });
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

#### 使用 actix-web

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[get("/users/{id}")]
async fn get_user(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(User { id, name: "Alice".to_string() })
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

## 24. 大型项目架构设计

本章介绍大型 Rust 项目的架构设计原则和最佳实践。

### 24.1 模块化设计原则

#### 模块组织结构

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   └── validator.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── order.rs
│   │   └── product.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── database.rs
│   │   ├── cache.rs
│   │   └── messaging.rs
│   ├── application/
│   │   ├── mod.rs
│   │   ├── services.rs
│   │   └── handlers.rs
│   └── utils/
│       ├── mod.rs
│       └── helpers.rs
└── tests/
    ├── integration/
    └── e2e/
```

#### 模块可见性控制

```rust
pub mod api;
pub mod domain;
pub mod infrastructure;

mod internal;

pub use domain::User;
pub use infrastructure::Database;

pub(crate) fn internal_helper() {
    println!("仅在 crate 内部可见");
}

pub(super) fn parent_helper() {
    println!("仅在父模块可见");
}
```

#### 重导出模式

```rust
pub mod models {
    pub use super::domain::user::{User, UserId};
    pub use super::domain::order::{Order, OrderId};
    pub use super::domain::product::{Product, ProductId};
}

pub mod services {
    pub use super::application::user_service::UserService;
    pub use super::application::order_service::OrderService;
}
```

### 24.2 分层架构

#### 领域层

```rust
pub mod domain {
    use std::sync::Arc;
    
    pub struct UserId(u64);
    
    pub struct User {
        pub id: UserId,
        pub name: String,
        pub email: String,
    }
    
    pub trait UserRepository: Send + Sync {
        async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError>;
        async fn save(&self, user: &User) -> Result<(), DomainError>;
        async fn delete(&self, id: UserId) -> Result<(), DomainError>;
    }
    
    pub struct UserService {
        repository: Arc<dyn UserRepository>,
    }
    
    impl UserService {
        pub fn new(repository: Arc<dyn UserRepository>) -> Self {
            Self { repository }
        }
        
        pub async fn create_user(&self, name: String, email: String) -> Result<User, DomainError> {
            let user = User {
                id: UserId(rand::random()),
                name,
                email,
            };
            self.repository.save(&user).await?;
            Ok(user)
        }
        
        pub async fn get_user(&self, id: UserId) -> Result<Option<User>, DomainError> {
            self.repository.find_by_id(id).await
        }
    }
}
```

#### 基础设施层

```rust
pub mod infrastructure {
    use sqlx::PgPool;
    use async_trait::async_trait;
    
    pub struct PostgresUserRepository {
        pool: PgPool,
    }
    
    impl PostgresUserRepository {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }
    
    #[async_trait]
    impl UserRepository for PostgresUserRepository {
        async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError> {
            let row = sqlx::query!(
                "SELECT id, name, email FROM users WHERE id = $1",
                id.0 as i64
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))?;
            
            Ok(row.map(|r| User {
                id: UserId(r.id as u64),
                name: r.name,
                email: r.email,
            }))
        }
        
        async fn save(&self, user: &User) -> Result<(), DomainError> {
            sqlx::query!(
                "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)
                 ON CONFLICT (id) DO UPDATE SET name = $2, email = $3",
                user.id.0 as i64,
                user.name,
                user.email,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))?;
            
            Ok(())
        }
        
        async fn delete(&self, id: UserId) -> Result<(), DomainError> {
            sqlx::query!("DELETE FROM users WHERE id = $1", id.0 as i64)
                .execute(&self.pool)
                .await
                .map_err(|e| DomainError::Database(e.to_string()))?;
            
            Ok(())
        }
    }
}
```

#### 应用层

```rust
pub mod application {
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        Json,
    };
    
    pub struct AppState {
        pub user_service: Arc<UserService>,
        pub config: Arc<Config>,
    }
    
    pub async fn get_user_handler(
        State(state): State<Arc<AppState>>,
        Path(id): Path<u64>,
    ) -> Result<Json<User>, StatusCode> {
        let user = state.user_service
            .get_user(UserId(id))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        user.map(Json).ok_or(StatusCode::NOT_FOUND)
    }
    
    #[derive(Deserialize)]
    pub struct CreateUserRequest {
        pub name: String,
        pub email: String,
    }
    
    pub async fn create_user_handler(
        State(state): State<Arc<AppState>>,
        Json(req): Json<CreateUserRequest>,
    ) -> Result<Json<User>, StatusCode> {
        let user = state.user_service
            .create_user(req.name, req.email)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(user))
    }
}
```

### 24.3 依赖注入模式

#### 使用 trait 进行依赖注入

```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> std::time::Instant;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> std::time::Instant {
        std::time::Instant::now()
    }
}

pub struct MockClock {
    current: std::sync::atomic::AtomicU64,
}

impl Clock for MockClock {
    fn now(&self) -> std::time::Instant {
        let nanos = self.current.load(std::sync::atomic::Ordering::SeqCst);
        std::time::Instant::from_nanos(nanos)
    }
}

pub struct Scheduler {
    clock: Arc<dyn Clock>,
}

impl Scheduler {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }
}
```

#### 使用 Builder 模式

```rust
pub struct Application {
    db: PgPool,
    cache: RedisClient,
    user_service: Arc<UserService>,
    order_service: Arc<OrderService>,
}

pub struct ApplicationBuilder {
    db_url: Option<String>,
    redis_url: Option<String>,
    config: Option<Config>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self {
            db_url: None,
            redis_url: None,
            config: None,
        }
    }
    
    pub fn database(mut self, url: impl Into<String>) -> Self {
        self.db_url = Some(url.into());
        self
    }
    
    pub fn redis(mut self, url: impl Into<String>) -> Self {
        self.redis_url = Some(url.into());
        self
    }
    
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }
    
    pub async fn build(self) -> Result<Application, BuildError> {
        let db_url = self.db_url.ok_or(BuildError::MissingDatabase)?;
        let redis_url = self.redis_url.ok_or(BuildError::MissingRedis)?;
        
        let db = PgPool::connect(&db_url).await?;
        let cache = RedisClient::new(&redis_url)?;
        
        let user_repo = Arc::new(PostgresUserRepository::new(db.clone()));
        let user_service = Arc::new(UserService::new(user_repo));
        
        let order_repo = Arc::new(PostgresOrderRepository::new(db.clone()));
        let order_service = Arc::new(OrderService::new(order_repo));
        
        Ok(Application {
            db,
            cache,
            user_service,
            order_service,
        })
    }
}
```

### 24.4 错误处理策略

#### 分层错误处理

```rust
pub mod error {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum DomainError {
        #[error("实体未找到: {0}")]
        NotFound(String),
        #[error("验证失败: {0}")]
        Validation(String),
        #[error("业务规则违反: {0}")]
        BusinessRule(String),
    }
    
    #[derive(Error, Debug)]
    pub enum InfrastructureError {
        #[error("数据库错误: {0}")]
        Database(String),
        #[error("缓存错误: {0}")]
        Cache(String),
        #[error("网络错误: {0}")]
        Network(String),
    }
    
    #[derive(Error, Debug)]
    pub enum ApplicationError {
        #[error("领域错误: {0}")]
        Domain(#[from] DomainError),
        #[error("基础设施错误: {0}")]
        Infrastructure(#[from] InfrastructureError),
        #[error("未授权")]
        Unauthorized,
        #[error("禁止访问")]
        Forbidden,
    }
    
    impl From<sqlx::Error> for InfrastructureError {
        fn from(e: sqlx::Error) -> Self {
            InfrastructureError::Database(e.to_string())
        }
    }
}
```

#### 错误转换层

```rust
impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApplicationError::Domain(DomainError::NotFound(_)) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            ApplicationError::Domain(DomainError::Validation(_)) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            ApplicationError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "未授权".to_string())
            }
            ApplicationError::Infrastructure(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "内部错误".to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "未知错误".to_string()),
        };
        
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
```

### 24.5 可测试性设计

#### 使用 trait 隔离外部依赖

```rust
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError>;
}

pub struct SmtpEmailSender {
    client: SmtpClient,
}

#[async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
        self.client.send(to, subject, body).await
    }
}

pub struct MockEmailSender {
    pub sent: Arc<Mutex<Vec<EmailRecord>>>,
}

#[async_trait]
impl EmailSender for MockEmailSender {
    async fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
        self.sent.lock().await.push(EmailRecord {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        });
        Ok(())
    }
}

pub struct NotificationService {
    email_sender: Arc<dyn EmailSender>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_send_notification() {
        let mock_sender = Arc::new(MockEmailSender {
            sent: Arc::new(Mutex::new(Vec::new())),
        });
        
        let service = NotificationService::new(mock_sender.clone());
        service.notify("test@example.com", "Hello").await.unwrap();
        
        let sent = mock_sender.sent.lock().await;
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].to, "test@example.com");
    }
}
```

#### 测试替身模式

```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}

pub struct TestClock {
    frozen_time: chrono::DateTime<chrono::Utc>,
}

impl TestClock {
    pub fn new(time: chrono::DateTime<chrono::Utc>) -> Self {
        Self { frozen_time: time }
    }
    
    pub fn advance(&mut self, duration: chrono::Duration) {
        self.frozen_time = self.frozen_time + duration;
    }
}

impl Clock for TestClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.frozen_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expiration() {
        let clock = Arc::new(TestClock::new(chrono::Utc::now()));
        let service = TokenService::new(clock.clone());
        
        let token = service.create_token(Duration::hours(1));
        assert!(service.is_valid(&token));
        
        clock.advance(Duration::hours(2));
        assert!(!service.is_valid(&token));
    }
}
```

---

## 25. 嵌入式与 no_std 开发

Rust 在嵌入式开发领域表现出色，本章介绍 no_std 环境和嵌入式开发。

### 25.1 no_std 环境

#### 禁用标准库

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
```

#### Cargo.toml 配置

```toml
[package]
name = "embedded_project"
version = "0.1.0"
edition = "2021"

[dependencies]

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
panic = "abort"
```

#### 使用 core 替代 std

```rust
#![no_std]

use core::fmt::{self, Display, Write};
use core::slice;
use core::str;

struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            let ptr = 0x1000_0000 as *mut u8;
            for byte in s.bytes() {
                core::ptr::write_volatile(ptr, byte);
            }
        }
        Ok(())
    }
}

impl Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Console")
    }
}
```

### 25.2 嵌入式 HAL

#### 使用 embedded-hal

```rust
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::spi::Write;

struct Led<P: OutputPin> {
    pin: P,
}

impl<P: OutputPin> Led<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }
    
    pub fn on(&mut self) {
        self.pin.set_high().ok();
    }
    
    pub fn off(&mut self) {
        self.pin.set_low().ok();
    }
}

fn blink_led<P: OutputPin, D: DelayMs<u32>>(
    led: &mut Led<P>,
    delay: &mut D,
    times: u32,
) {
    for _ in 0..times {
        led.on();
        delay.delay_ms(500);
        led.off();
        delay.delay_ms(500);
    }
}
```

#### SPI 设备驱动

```rust
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

pub struct SpiDevice<SPI, CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin,
{
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> SpiDevice<SPI, CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
    
    pub fn read_register(&mut self, reg: u8) -> Result<u8, SPI::Error> {
        self.cs.set_low().ok();
        let mut buf = [reg, 0];
        self.spi.transfer(&mut buf)?;
        self.cs.set_high().ok();
        Ok(buf[1])
    }
    
    pub fn write_register(&mut self, reg: u8, value: u8) -> Result<(), SPI::Error> {
        self.cs.set_low().ok();
        let mut buf = [reg | 0x80, value];
        self.spi.transfer(&mut buf)?;
        self.cs.set_high().ok();
        Ok(())
    }
}
```

### 25.3 交叉编译

#### 配置目标

```bash
rustup target add thumbv7em-none-eabihf
rustup target add aarch64-unknown-none-softfloat

cargo build --target thumbv7em-none-eabihf --release
```

#### .cargo/config.toml

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = [
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=-nostartfiles",
]

[target.aarch64-unknown-none-softfloat]
rustflags = [
    "-C", "link-arg=-Taarch64.ld",
]
```

#### 内存布局脚本

```
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 256K
    RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

ENTRY(_start);

SECTIONS
{
    .text : {
        *(.text._start)
        *(.text*)
    } > FLASH
    
    .rodata : {
        *(.rodata*)
    } > FLASH
    
    .data : {
        _sdata = .;
        *(.data*)
        _edata = .;
    } > RAM AT > FLASH
    
    .bss : {
        _sbss = .;
        *(.bss*)
        _ebss = .;
    } > RAM
}
```

### 25.4 嵌入式异步

#### 使用 embassy

```rust
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_rp::gpio::{Level, Output};

#[embassy::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    let mut led = Output::new(p.PIN_25, Level::Low);
    
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
```

#### 异步驱动

```rust
use embassy::util::Unborrow;
use embassy_hal_common::unborrow;

pub struct AsyncUart<'d, T: Instance> {
    uart: Uart<'d, T>,
    rx_buf: &'d mut [u8],
}

impl<'d, T: Instance> AsyncUart<'d, T> {
    pub async fn read(&mut self) -> Result<&[u8], Error> {
        let len = self.uart.read_until_idle(self.rx_buf).await?;
        Ok(&self.rx_buf[..len])
    }
    
    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        self.uart.write(data).await;
        Ok(())
    }
}
```

---

## 26. WebAssembly 开发

Rust 是 WebAssembly 开发的首选语言之一，本章介绍 WASM 开发的完整流程。

### 26.1 编译到 WASM

#### 安装工具链

```bash
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

cargo install wasm-pack
cargo install wasm-bindgen-cli
```

#### Cargo.toml 配置

```toml
[package]
name = "wasm-demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console", "Document", "Element", "Window"] }

[profile.release]
opt-level = "s"
lto = true
```

#### 基本编译

```bash
cargo build --target wasm32-unknown-unknown --release

wasm-bindgen target/wasm32-unknown-unknown/release/wasm_demo.wasm \
    --out-dir pkg \
    --target web
```

### 26.2 wasm-bindgen

#### 导出函数

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    log(&format!("Adding {} and {}", a, b));
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub struct Counter {
    count: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { count: 0 }
    }
    
    pub fn increment(&mut self) {
        self.count += 1;
    }
    
    pub fn get(&self) -> i32 {
        self.count
    }
}
```

#### 处理复杂数据

```rust
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u32,
}

#[wasm_bindgen]
pub fn process_user(json: &str) -> Result<String, JsValue> {
    let user: User = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let processed = User {
        name: format!("Processed: {}", user.name),
        age: user.age + 1,
    };
    
    serde_json::to_string(&processed)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn create_array() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[wasm_bindgen]
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
```

### 26.3 与 JavaScript 交互

#### 调用 JavaScript 函数

```rust
use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Reflect};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
    
    type HTMLDocument;
    
    #[wasm_bindgen(js_namespace = window)]
    static document: HTMLDocument;
    
    #[wasm_bindgen(method, structural)]
    fn getElementById(this: &HTMLDocument, id: &str) -> Option<Element>;
    
    type Element;
    
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn update_element(id: &str, content: &str) {
    if let Some(element) = document.getElementById(id) {
        element.set_inner_html(content);
    }
}

#[wasm_bindgen]
pub fn call_js_callback(callback: &js_sys::Function) {
    let result = callback.call0(&JsValue::NULL);
    log(&format!("Callback result: {:?}", result));
}
```

#### 处理 Promise

```rust
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = fetch)]
    fn fetch(url: &str) -> Promise;
}

#[wasm_bindgen]
pub async fn fetch_data(url: &str) -> Result<String, JsValue> {
    let promise = fetch(url);
    let response = JsFuture::from(promise).await?;
    
    let json_promise = js_sys::Reflect::get(&response, &JsValue::from_str("json"))?
        .dyn_into::<js_sys::Function>()?
        .call0(&response)?;
    
    let json = JsFuture::from(json_promise).await?;
    Ok(js_sys::JSON::stringify(&json)?.as_string().unwrap())
}
```

#### 事件处理

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Event, EventTarget, HtmlElement, MouseEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn setup_event_handlers() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let button = document.get_element_by_id("myButton").unwrap();
    let button: HtmlElement = button.dyn_into().unwrap();
    
    let closure = Closure::<dyn FnMut(_)>::new(|event: MouseEvent| {
        log(&format!("Button clicked at ({}, {})", event.client_x(), event.client_y()));
    });
    
    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
```

### 26.4 WASM 性能优化

#### 内存管理

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_large_data(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    for chunk in data.chunks(1024) {
        let processed: Vec<u8> = chunk.iter().map(|b| b.wrapping_add(1)).collect();
        result.extend(processed);
    }
    result
}

#[wasm_bindgen]
pub struct Buffer {
    data: Vec<u8>,
}

#[wasm_bindgen]
impl Buffer {
    #[wasm_bindgen(constructor)]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    
    pub fn write(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }
    
    pub fn read(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
```

#### 减少边界检查

```rust
#[wasm_bindgen]
pub fn sum_unsafe(data: &[i32]) -> i32 {
    unsafe {
        let ptr = data.as_ptr();
        let len = data.len();
        let mut sum = 0;
        for i in 0..len {
            sum += *ptr.add(i);
        }
        sum
    }
}

#[wasm_bindgen]
pub fn sum_safe(data: &[i32]) -> i32 {
    data.iter().sum()
}
```

#### 使用 wee_alloc 减小体积

```toml
[dependencies]
wee_alloc = "0.4"
```

```rust
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### 构建优化

```bash
wasm-pack build --release --target web -- --features wee_alloc

wasm-opt -Oz -o output.wasm input.wasm
```

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

## 附录

#### 官方资源
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 编程语言](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

#### 社区资源
- [Rust 论坛](https://users.rust-lang.org/)
- [Rust 子reddit](https://www.reddit.com/r/rust/)

---

**文档版本**: 2.0  
**最后更新**: 2026-03-11  
**基于**: step01-step24 学习资料 + 高级进阶内容补充

**v2.0 更新内容**:
- 新增第18章：过程宏深入（派生宏、属性宏、函数式宏）
- 新增第19章：异步编程底层原理（Future、Pin/Unpin、运行时）
- 新增第20章：FFI 与跨语言互操作
- 新增第21章：类型系统进阶（GATs、类型级编程、幻影类型）
- 新增第22章：性能优化与基准测试
- 新增第23章：生态系统与常用 Crate 实践
- 新增第24章：大型项目架构设计
- 新增第25章：嵌入式与 no_std 开发
- 新增第26章：WebAssembly 开发
