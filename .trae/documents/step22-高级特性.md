# 步骤 22：高级特性学习

## 操作目标
掌握 Rust 的高级特性，包括不安全 Rust、高级特质、高级类型、高级函数和闭包、宏等。

## 执行方法

### 1. 不安全 Rust

#### 学习内容
- unsafe 关键字的使用
- 不安全操作的类型
- 原始指针
- 不安全函数和方法
- 不安全代码块

#### 示例代码
```rust
// 原始指针
fn main() {
    let mut num = 5;
    
    // 不可变原始指针
    let r1 = &num as *const i32;
    // 可变原始指针
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1: {:?}", *r1);
        println!("r2: {:?}", *r2);
    }
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

// 不安全代码块中的操作
unsafe {
    // 解引用原始指针
    // 调用不安全函数
    // 访问或修改可变静态变量
    // 实现不安全特质
}

// 外部函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("-3 的绝对值: {}", abs(-3));
    }
}
```

### 2. 高级特质

#### 学习内容
- 关联类型
- 默认方法实现
- 特质继承
- 泛型特质
- 特质对象

#### 示例代码
```rust
// 关联类型
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

// 默认方法实现
trait Animal {
    fn name(&self) -> String;
    
    fn make_sound(&self) {
        println!("{} 发出声音", self.name());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn make_sound(&self) {
        println!("{} 汪汪汪", self.name());
    }
}

// 特质继承
trait Pet: Animal {
    fn owner(&self) -> String;
}

struct Cat {
    name: String,
    owner: String,
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn owner(&self) -> String {
        self.owner.clone()
    }
}

// 泛型特质
trait Container<T> {
    fn contains(&self, item: &T) -> bool;
}

impl<T: PartialEq> Container<T> for Vec<T> {
    fn contains(&self, item: &T) -> bool {
        self.iter().any(|x| x == item)
    }
}
```

### 3. 高级类型

#### 学习内容
- 新类型模式
- 类型别名
- 从未类型 (!)
- 动态大小类型

#### 示例代码
```rust
// 新类型模式
struct Years(u64);
struct Days(u64);

impl Years {
    fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

// 类型别名
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: Kilometers = 5;
    println!("距离: {} 公里", x);
    
    let f: Thunk = Box::new(|| println!("Hello"));
    f();
}

// 从未类型
fn never_returns() -> ! {
    panic!("这个函数永远不会返回");
}

// 动态大小类型
fn main() {
    // &str 是动态大小类型的引用
    let s: &str = "Hello";
    println!("字符串: {}", s);
    
    // [T] 是动态大小类型
    let arr: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &arr;
    println!("切片: {:?}", slice);
}
```

### 4. 高级函数和闭包

#### 学习内容
- 函数指针
- 返回闭包
- 闭包的类型推断
- 闭包的捕获方式

#### 示例代码
```rust
// 函数指针
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("答案: {}", answer);
}

// 返回闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// 闭包的类型推断
fn main() {
    let plus_one = |x| x + 1;
    let result = plus_one(5);
    println!("结果: {}", result);
}

// 闭包的捕获方式
fn main() {
    // 不可变捕获
    let x = 5;
    let equal_to_x = |z| z == x;
    let z = 5;
    println!("5 == 5: {}", equal_to_x(z));
    
    // 可变捕获
    let mut y = 5;
    let mut add_to_y = |z| y += z;
    add_to_y(5);
    println!("y: {}", y);
    
    // 移动捕获
    let s = String::from("hello");
    let consume = move || println!("{}", s);
    // 不能再使用 s，因为所有权已经移动到闭包中
    consume();
}
```

### 5. 宏

#### 学习内容
- 声明式宏
- 过程宏
- 宏的使用场景

#### 示例代码
```rust
// 声明式宏
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

// 自定义宏
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

// 过程宏（需要在单独的 crate 中实现）
// #[derive(Debug)] 就是一个过程宏
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 56 天
- **完成时间**：学习计划第 59 天

## 预期成果
- 掌握不安全 Rust 的使用
- 理解高级特质的实现
- 能够使用高级类型
- 掌握高级函数和闭包的使用
- 能够编写和使用宏

## 优先级
- **优先级**：低
- **理由**：高级特性是 Rust 的进阶内容，在基本应用中可能不会经常使用

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 不安全代码的风险 | 尽量避免使用不安全代码，只在必要时使用，并确保安全 |
| 宏的复杂性 | 保持宏的简洁性，避免过度使用宏 |
| 类型系统的复杂性 | 理解类型系统的工作原理，避免过度复杂的类型设计 |

## 验证步骤

1. 编写使用不安全 Rust 的代码
2. 实现高级特质
3. 使用高级类型
4. 编写和使用宏
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成高级特性学习后，继续学习 [步骤 23：多线程 web 服务器开发](step23-web服务器.md)。
