# 步骤 6：所有权系统学习

## 操作目标
理解 Rust 独特的所有权系统，掌握所有权、借用和切片的概念。

## 执行方法

### 1. 所有权概念

#### 学习内容
- 所有权的基本原则
- 移动（Move）语义
- 复制（Copy）语义
- 克隆（Clone）语义

#### 示例代码
```rust
// 所有权转移
let s1 = String::from("hello");
let s2 = s1; // s1 的所有权转移给 s2
// println!("{}", s1); // 这里会编译错误，因为 s1 已经没有所有权

// 复制语义（对于基本类型）
let x = 5;
let y = x; // x 的值被复制给 y，x 仍然有效
println!("x = {}, y = {}", x, y);

// 克隆语义
let s3 = String::from("hello");
let s4 = s3.clone(); // 显式克隆，s3 仍然有效
println!("s3 = {}, s4 = {}", s3, s4);
```

### 2. 引用与借用

#### 学习内容
- 不可变引用
- 可变引用
- 借用规则

#### 示例代码
```rust
// 不可变引用
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("字符串 '{}' 的长度是 {}", s1, len);

// 可变引用
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s2 = String::from("hello");
change(&mut s2);
println!("{}", s2);

// 借用规则：同一时间只能有一个可变引用或多个不可变引用
let mut s3 = String::from("hello");
let r1 = &s3; // 不可变引用
let r2 = &s3; // 另一个不可变引用，允许
// let r3 = &mut s3; // 可变引用，编译错误
println!("r1 = {}, r2 = {}", r1, r2);
```

### 3. 切片类型

#### 学习内容
- 字符串切片
- 数组切片
- 切片的不可变性

#### 示例代码
```rust
// 字符串切片
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
println!("{} {}", hello, world);

// 数组切片
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
println!("切片元素: {:?}", slice);

// 函数参数使用切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

let my_string = String::from("hello world");
let word = first_word(&my_string[..]);
println!("第一个单词: {}", word);
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 8 天
- **完成时间**：学习计划第 10 天

## 预期成果
- 理解 Rust 的所有权系统
- 掌握所有权、借用和切片的概念
- 能够正确使用引用和借用
- 避免常见的所有权错误

## 优先级
- **优先级**：高
- **理由**：所有权系统是 Rust 的核心特性，是理解 Rust 内存管理的关键

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 所有权概念理解困难 | 多编写示例代码，参考官方文档和社区资源 |
| 借用规则违反 | 仔细检查代码，遵循借用规则 |
| 生命周期错误 | 注意引用的生命周期，使用适当的生命周期标注 |

## 验证步骤

1. 编写包含所有权转移、引用和切片的示例代码
2. 执行 `cargo check` 检查代码是否正确
3. 执行 `cargo run` 运行代码，确认输出正确
4. 尝试违反借用规则，观察编译错误

## 后续步骤

完成所有权系统学习后，继续学习 [步骤 7：结构体学习](step07-结构体.md)。
