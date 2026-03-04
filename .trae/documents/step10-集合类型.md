# 步骤 10：集合类型学习

## 操作目标
掌握 Rust 中的集合类型，包括向量、字符串和哈希映射的使用。

## 执行方法

### 1. 向量（Vector）

#### 学习内容
- 向量的创建
- 向量的元素访问
- 向量的修改
- 向量的遍历

#### 示例代码
```rust
// 创建向量
let v1: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];

// 添加元素
let mut v3 = Vec::new();
v3.push(1);
v3.push(2);
v3.push(3);

// 访问元素
let v4 = vec![1, 2, 3, 4, 5];
let third: &i32 = &v4[2];
println!("第三个元素: {}", third);

// 使用 get 方法访问
match v4.get(2) {
    Some(third) => println!("第三个元素: {}", third),
    None => println!("索引超出范围"),
}

// 遍历向量
for i in &v4 {
    println!("{}", i);
}

// 遍历并修改
let mut v5 = vec![100, 32, 57];
for i in &mut v5 {
    *i += 50;
    println!("{}", i);
}
```

### 2. 字符串（String）

#### 学习内容
- 字符串的创建
- 字符串的修改
- 字符串的拼接
- 字符串的遍历

#### 示例代码
```rust
// 创建字符串
let s1 = String::new();
let s2 = "initial contents".to_string();
let s3 = String::from("initial contents");

// 修改字符串
let mut s4 = String::from("hello");
s4.push_str(", world");
s4.push('!');

// 字符串拼接
let s5 = String::from("hello");
let s6 = String::from("world");
let s7 = s5 + &s6; // s5 被移动，不能再使用

// 使用 format! 宏
let s8 = String::from("hello");
let s9 = String::from("world");
let s10 = format!("{} {}", s8, s9); // s8 和 s9 仍然可用

// 遍历字符串
for c in "hello".chars() {
    println!("{}", c);
}

for b in "hello".bytes() {
    println!("{}", b);
}
```

### 3. 哈希映射（HashMap）

#### 学习内容
- 哈希映射的创建
- 哈希映射的插入和访问
- 哈希映射的修改
- 哈希映射的遍历

#### 示例代码
```rust
use std::collections::HashMap;

// 创建哈希映射
let mut scores = HashMap::new();

// 插入键值对
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 访问值
let team_name = String::from("Blue");
let score = scores.get(&team_name);

match score {
    Some(s) => println!("分数: {}", s),
    None => println!("没有找到该队伍的分数"),
}

// 遍历哈希映射
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新哈希映射
// 覆盖现有值
scores.insert(String::from("Blue"), 25);

// 仅在键不存在时插入
scores.entry(String::from("Red")).or_insert(30);

// 根据旧值更新
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 18 天
- **完成时间**：学习计划第 20 天

## 预期成果
- 能够创建和使用向量
- 掌握字符串的操作和修改
- 能够使用哈希映射存储键值对
- 理解集合类型的使用场景

## 优先级
- **优先级**：高
- **理由**：集合类型是 Rust 中存储和管理数据的重要工具，是编写实际应用程序的基础

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 向量索引越界 | 使用 get 方法或检查索引范围 |
| 字符串所有权问题 | 注意字符串操作中的所有权转移 |
| 哈希映射性能问题 | 选择合适的哈希函数，避免过度使用 |

## 验证步骤

1. 编写使用向量、字符串和哈希映射的示例代码
2. 执行 `cargo check` 检查代码是否正确
3. 执行 `cargo run` 运行代码，确认输出正确
4. 测试各种集合操作的边界情况

## 后续步骤

完成集合类型学习后，继续学习 [步骤 11：错误处理学习](step11-错误处理.md)。
