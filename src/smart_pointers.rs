/// 智能指针示例模块
/// 涵盖：Box<T>、Drop 特性、Rc<T>、RefCell<T>

use std::cell::RefCell;
use std::rc::Rc;

/// 演示 Box<T>
fn demo_box() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. Box<T> 堆分配                   │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本使用：");
    let b = Box::new(5);
    println!("Box 中的值：{}", b);

    println!("\n递归数据结构：");
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表：{:?}", list);

    println!("\nDeref trait：");
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("Box 实现了 Deref，可以像引用一样使用");

    println!("\n大类型存储：");
    struct LargeData {
        data: [u8; 1000],
    }

    let _large = Box::new(LargeData { data: [0u8; 1000] });
    println!("大类型存储在堆上，栈上只保留指针");
}

/// 演示 Drop 特性
fn demo_drop() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. Drop 特性                       │");
    println!("└─────────────────────────────────────┘");

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("释放 CustomSmartPointer，数据：{}", self.data);
        }
    }

    println!("\n自动调用 drop：");
    {
        let _c = CustomSmartPointer {
            data: String::from("我的数据"),
        };
        let _d = CustomSmartPointer {
            data: String::from("其他数据"),
        };
        println!("创建两个智能指针");
    }
    println!("离开作用域，自动调用 drop");

    println!("\n手动调用 drop：");
    let e = CustomSmartPointer {
        data: String::from("手动释放"),
    };
    println!("创建智能指针");
    drop(e);
    println!("手动释放后");

    println!("\nDrop 的用途：");
    println!("1. 释放内存");
    println!("2. 关闭文件句柄");
    println!("3. 释放网络连接");
    println!("4. 执行清理逻辑");
}

/// 演示 Rc<T>
fn demo_rc() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. Rc<T> 引用计数                  │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本使用：");
    let a = Rc::new(String::from("hello"));
    println!("创建 a，引用计数：{}", Rc::strong_count(&a));

    let _b = Rc::clone(&a);
    println!("克隆 b，引用计数：{}", Rc::strong_count(&a));

    {
        let _c = Rc::clone(&a);
        println!("克隆 c，引用计数：{}", Rc::strong_count(&a));
    }

    println!("c 离开作用域，引用计数：{}", Rc::strong_count(&a));

    println!("\n共享数据：");
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("创建 a，引用计数：{}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("创建 b，引用计数：{}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("创建 c，引用计数：{}", Rc::strong_count(&a));

    println!("b = {:?}", b);
    println!("c = {:?}", c);

    println!("\nRc<T> 的限制：");
    println!("1. 只能用于单线程");
    println!("2. 不可变引用");
    println!("3. 可能造成循环引用");
}

/// 演示 RefCell<T>
fn demo_refcell() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. RefCell<T> 内部可变性           │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本使用：");
    let x = RefCell::new(5);

    {
        let mut y = x.borrow_mut();
        *y += 1;
    }

    println!("修改后的值：{}", x.borrow());

    println!("\n内部可变性模式：");
    trait Messenger {
        fn send(&self, msg: &str);
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    let mock = MockMessenger {
        sent_messages: RefCell::new(vec![]),
    };
    mock.send("消息 1");
    mock.send("消息 2");
    println!("发送的消息：{:?}", mock.sent_messages.borrow());

    println!("\nRc<RefCell<T>> 组合：");
    #[derive(Debug)]
    struct Person {
        name: String,
        age: RefCell<u32>,
    }

    let person = Rc::new(Person {
        name: String::from("Alice"),
        age: RefCell::new(30),
    });

    let person1 = Rc::clone(&person);
    let person2 = Rc::clone(&person);

    *person1.age.borrow_mut() += 1;
    println!("{} 的年龄：{}", person2.name, person2.age.borrow());

    println!("\nRefCell<T> 的规则：");
    println!("1. 运行时检查借用规则");
    println!("2. borrow() 返回不可变引用");
    println!("3. borrow_mut() 返回可变引用");
    println!("4. 违反规则会 panic");
}

/// 演示智能指针组合
fn demo_combinations() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. 智能指针组合                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n常见组合模式：");
    println!("Box<T>：独占所有权，堆分配");
    println!("Rc<T>：共享所有权，引用计数");
    println!("RefCell<T>：内部可变性，运行时检查");
    println!("Rc<RefCell<T>>：共享可变数据");

    println!("\n选择指南：");
    println!("1. 需要堆分配？使用 Box<T>");
    println!("2. 需要共享所有权？使用 Rc<T>");
    println!("3. 需要内部可变性？使用 RefCell<T>");
    println!("4. 需要共享可变数据？使用 Rc<RefCell<T>>");
}

/// 公开的演示函数
pub fn demo_smart_pointers() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块七：智能指针                    ║");
    println!("╚═════════════════════════════════════════╝");

    demo_box();
    demo_drop();
    demo_rc();
    demo_refcell();
    demo_combinations();
}
