# 步骤 17：智能指针学习

## 操作目标
掌握 Rust 的智能指针，包括 Box<T>、Rc<T>、RefCell<T> 等的使用。

## 执行方法

### 1. Box<T>

#### 学习内容
- Box<T> 的定义和使用
- Box<T> 的适用场景
- Box<T> 的内存布局

#### 示例代码
```rust
// 使用 Box<T> 存储堆上的数据
let b = Box::new(5);
println!("b = {}", b);

// Box<T> 自动释放内存
fn main() {
    {
        let b = Box::new(5);
        println!("b = {}", b);
    } // b 离开作用域，内存被自动释放
}

// Box<T> 用于递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))));
    println!("{:?}", list);
}

// Box<T> 实现了 Deref trait
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}
```

### 2. Drop 特性

#### 学习内容
- Drop 特性的实现
- 自动释放资源
- Drop 的执行时机

#### 示例代码
```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("释放 CustomSmartPointer，数据: {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("创建了 CustomSmartPointer");
} // c 和 d 离开作用域，drop 方法被自动调用

// 手动调用 drop
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    println!("创建了 CustomSmartPointer");
    drop(c); // 手动调用 drop
    println!("在 main 结束前");
} // c 已经被手动 drop，不会再次调用
```

### 3. Rc<T>

#### 学习内容
- Rc<T> 的定义和使用
- 引用计数
- 克隆和共享所有权

#### 示例代码
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    
    println!("引用计数: {}", Rc::strong_count(&a)); // 3
}

// 递归类型使用 Rc<T>
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("创建 a 后的引用计数: {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("创建 b 后的引用计数: {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("创建 c 后的引用计数: {}", Rc::strong_count(&a));
    }
    
    println!("c 离开作用域后的引用计数: {}", Rc::strong_count(&a));
}
```

### 4. RefCell<T>

#### 学习内容
- RefCell<T> 的定义和使用
- 内部可变性
- 运行时借用检查
- 与 Rc<T> 结合使用

#### 示例代码
```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    
    { 
        let mut y = x.borrow_mut();
        *y += 1;
    } // y 离开作用域，可变借用结束
    
    println!("x = {:?}", x.borrow());
}

// 与 Rc<T> 结合使用
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Person {
    name: String,
    age: RefCell<u32>,
}

fn main() {
    let person = Rc::new(Person {
        name: String::from("Alice"),
        age: RefCell::new(30),
    });
    
    let person1 = Rc::clone(&person);
    let person2 = Rc::clone(&person);
    
    *person1.age.borrow_mut() += 1;
    println!("{} 的年龄: {:?}", person2.name, person2.age.borrow());
}

// 递归类型使用 Rc<T> 和 RefCell<T>
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;
    
    println!("a 中的值: {:?}", a);
    println!("b 中的值: {:?}", b);
    println!("c 中的值: {:?}", c);
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 42 天
- **完成时间**：学习计划第 44 天

## 预期成果
- 掌握 Box<T> 的使用
- 理解 Drop 特性的工作原理
- 能够使用 Rc<T> 实现共享所有权
- 能够使用 RefCell<T> 实现内部可变性
- 理解智能指针的适用场景

## 优先级
- **优先级**：中
- **理由**：智能指针是 Rust 中管理内存和实现复杂数据结构的重要工具

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 引用循环导致内存泄漏 | 避免创建引用循环，使用 Weak<T> 打破循环 |
| 运行时借用冲突 | 确保在适当的作用域内使用借用 |
| 过度使用内部可变性 | 仅在必要时使用 RefCell<T>，优先考虑不可变性 |

## 验证步骤

1. 编写使用 Box<T> 的代码
2. 实现 Drop 特性
3. 编写使用 Rc<T> 的代码
4. 编写使用 RefCell<T> 的代码
5. 尝试 Rc<T> 和 RefCell<T> 的组合使用
6. 执行 `cargo check` 检查代码是否正确
7. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成智能指针学习后，继续学习 [步骤 18：并发编程学习](step18-并发编程.md)。
