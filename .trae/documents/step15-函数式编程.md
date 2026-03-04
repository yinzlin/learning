# 步骤 15：函数式编程学习

## 操作目标
掌握 Rust 的函数式编程特性，包括闭包、迭代器和函数式风格的代码编写。

## 执行方法

### 1. 闭包

#### 学习内容
- 闭包的定义和使用
- 闭包的类型推断
- 闭包的捕获方式
- 闭包作为参数和返回值

#### 示例代码
```rust
// 闭包的定义和使用
let add_one = |x| x + 1;
let result = add_one(5);
println!("结果: {}", result);

// 多参数闭包
let add = |a, b| a + b;
let result = add(5, 3);
println!("结果: {}", result);

// 带类型标注的闭包
let add: fn(i32, i32) -> i32 = |a, b| a + b;

// 闭包捕获环境变量
let x = 5;
let add_x = |y| x + y;
let result = add_x(3);
println!("结果: {}", result);

// 可变闭包
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};

println!("{}", increment()); // 1
println!("{}", increment()); // 2

// 闭包作为参数
fn apply<F>(f: F) where F: FnOnce() {
    f();
}

let message = String::from("Hello");
apply(|| println!("{}", message));

// 闭包作为返回值
fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

let mut counter = make_counter();
println!("{}", counter()); // 1
println!("{}", counter()); // 2
```

### 2. 迭代器

#### 学习内容
- 迭代器的创建和使用
- 迭代器适配器
- 消费适配器
- 迭代器的性能

#### 示例代码
```rust
// 基本迭代器
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("值: {}", val);
}

// 消费适配器
let v1 = vec![1, 2, 3];
let total: i32 = v1.iter().sum();
println!("总和: {}", total);

// 迭代器适配器
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
println!("v2: {:?}", v2);

// 链式调用
let v1 = vec![1, 2, 3, 4, 5];
let v2: Vec<_> = v1.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
println!("v2: {:?}", v2);

// 创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
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

let mut counter = Counter::new();
while let Some(count) = counter.next() {
    println!("计数: {}", count);
}

// 迭代器组合
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
println!("总和: {}", sum);
```

### 3. 优化 I/O 项目

#### 学习内容
- 使用迭代器优化命令行程序
- 函数式风格的代码重构

#### 示例代码
```rust
// 优化搜索函数
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// 优化配置解析
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}

// 使用迭代器解析命令行参数
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // 跳过程序名
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少查询参数"),
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少文件路径参数"),
        };
        
        Ok(Config { query, file_path })
    }
}
```

### 4. 性能比较

#### 学习内容
- 循环与迭代器的性能比较
- 迭代器的优势

#### 示例代码
```rust
// 循环版本
fn sum_loop(v: &[i32]) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

// 迭代器版本
fn sum_iterator(v: &[i32]) -> i32 {
    v.iter().sum()
}

// 性能测试
fn main() {
    let v = vec![1; 1000000];
    
    let start = std::time::Instant::now();
    let sum1 = sum_loop(&v);
    let duration = start.elapsed();
    println!("循环版本: {:?}", duration);
    
    let start = std::time::Instant::now();
    let sum2 = sum_iterator(&v);
    let duration = start.elapsed();
    println!("迭代器版本: {:?}", duration);
    
    assert_eq!(sum1, sum2);
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 36 天
- **完成时间**：学习计划第 38 天

## 预期成果
- 掌握闭包的定义和使用
- 理解迭代器的工作原理
- 能够使用迭代器和闭包编写函数式风格的代码
- 了解迭代器的性能特性
- 能够使用函数式编程思想优化代码

## 优先级
- **优先级**：中
- **理由**：函数式编程是 Rust 的重要特性，能够使代码更简洁、更可维护

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 闭包捕获变量导致的所有权问题 | 理解闭包的捕获规则，正确处理所有权 |
| 迭代器性能误解 | 了解 Rust 迭代器的实现，知道何时使用迭代器 |
| 函数式代码可读性问题 | 保持代码简洁，避免过度使用链式调用 |

## 验证步骤

1. 编写使用闭包的代码
2. 编写使用迭代器的代码
3. 优化现有代码，使用函数式风格
4. 比较循环和迭代器的性能
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成函数式编程学习后，继续学习 [步骤 16：Cargo 高级功能学习](step16-cargo高级功能.md)。
