# 步骤 20：面向对象编程学习

## 操作目标
掌握 Rust 的面向对象编程特性，包括特质对象、继承和多态。

## 执行方法

### 1. 面向对象特性

#### 学习内容
- Rust 中的面向对象编程概念
- 封装
- 继承
- 多态

#### 示例代码
```rust
// 封装
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 使用
fn main() {
    let rect = Rectangle::new(10, 20);
    println!("面积: {}", rect.area());
}
```

### 2. 特质对象

#### 学习内容
- 特质对象的定义和使用
- 动态分发
- 特质对象的限制

#### 示例代码
```rust
// 定义特质
pub trait Draw {
    fn draw(&self);
}

// 实现特质
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: {}, {}x{}", self.label, self.width, self.height);
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("绘制文本框: '{}', {}x{}", self.text, self.width, self.height);
    }
}

// 使用特质对象
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

// 测试
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 50,
                label: String::from("点击我"),
            }),
            Box::new(TextField {
                width: 200,
                height: 30,
                text: String::from("输入文本"),
            }),
        ],
    };
    
    screen.run();
}
```

### 3. 实现设计模式

#### 学习内容
- 工厂模式
- 观察者模式
- 策略模式

#### 示例代码
```rust
// 工厂模式
pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub enum ShapeType {
    Circle(f64),
    Rectangle(f64, f64),
}

pub fn create_shape(shape_type: ShapeType) -> Box<dyn Shape> {
    match shape_type {
        ShapeType::Circle(radius) => Box::new(Circle { radius }),
        ShapeType::Rectangle(width, height) => Box::new(Rectangle { width, height }),
    }
}

// 观察者模式
use std::sync::mpsc;
use std::thread;

pub trait Observer {
    fn update(&self, message: &str);
}

pub struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
    
    pub fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
    
    pub fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

pub struct ConcreteObserver {
    name: String,
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("{} 收到消息: {}", self.name, message);
    }
}

// 策略模式
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

pub struct PayPalPayment {
    email: String,
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) {
        println!("使用 PayPal {} 支付 {} 元", self.email, amount);
    }
}

pub struct ShoppingCart {
    items: Vec<f64>,
    payment_strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    pub fn new(payment_strategy: Box<dyn PaymentStrategy>) -> Self {
        Self {
            items: Vec::new(),
            payment_strategy,
        }
    }
    
    pub fn add_item(&mut self, price: f64) {
        self.items.push(price);
    }
    
    pub fn checkout(&self) {
        let total = self.items.iter().sum::<f64>();
        self.payment_strategy.pay(total);
    }
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 51 天
- **完成时间**：学习计划第 53 天

## 预期成果
- 理解 Rust 中的面向对象编程概念
- 掌握特质对象的使用
- 能够实现常见的设计模式
- 理解 Rust 与传统面向对象语言的区别

## 优先级
- **优先级**：中
- **理由**：面向对象编程是一种重要的编程范式，Rust 提供了独特的实现方式

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 特质对象的性能开销 | 仅在需要动态分发时使用特质对象，否则使用泛型 |
| 设计模式过度使用 | 只在必要时使用设计模式，避免过度设计 |
| 与传统 OOP 的思维差异 | 理解 Rust 的所有权系统和特质系统，适应 Rust 的设计理念 |

## 验证步骤

1. 编写使用封装的代码
2. 实现特质对象
3. 实现常见的设计模式
4. 执行 `cargo check` 检查代码是否正确
5. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成面向对象编程学习后，继续学习 [步骤 21：模式匹配学习](step21-模式匹配.md)。
