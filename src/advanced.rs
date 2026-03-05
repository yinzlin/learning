/// 高级特性示例模块
/// 涵盖：不安全 Rust、高级类型、高级函数和闭包、宏

/// 演示不安全 Rust
fn demo_unsafe() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 不安全 Rust                     │");
    println!("└─────────────────────────────────────┘");

    println!("\n原始指针：");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 指向的值：{}", *r1);
        println!("r2 指向的值：{}", *r2);
    }

    println!("\n不安全函数：");
    unsafe fn dangerous() {
        println!("执行不安全操作");
    }

    unsafe {
        dangerous();
    }

    println!("\n不安全代码块的操作：");
    println!("1. 解引用原始指针");
    println!("2. 调用不安全函数");
    println!("3. 访问或修改可变静态变量");
    println!("4. 实现不安全 trait");
    println!("5. 访问 union 字段");

    println!("\n静态变量（使用原子类型）：");
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);

    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("COUNTER: {}", COUNTER.load(Ordering::SeqCst));

    println!("\n不安全 trait：");
    unsafe trait Foo {
        fn method(&self);
    }

    unsafe impl Foo for i32 {
        fn method(&self) {
            println!("Foo method: {}", self);
        }
    }

    42.method();

    println!("\n何时使用 unsafe：");
    println!("1. 与 C 代码交互（FFI）");
    println!("2. 实现底层抽象");
    println!("3. 性能优化");
    println!("4. 访问硬件");
}

/// 演示高级类型
fn demo_advanced_types() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 高级类型                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n新类型模式（Newtype）：");
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

    let years = Years(10);
    let days = years.to_days();
    println!("{} 年 = {} 天", years.0, days.0);

    println!("\n类型别名：");
    type Kilometers = i32;
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let x: Kilometers = 5;
    println!("距离：{} 公里", x);

    let f: Thunk = Box::new(|| println!("Hello"));
    f();

    println!("\n从不类型（Never type）：");
    fn never_returns() -> ! {
        panic!("这个函数永远不会返回");
    }

    fn diverges_or_returns(flag: bool) -> i32 {
        if flag {
            42
        } else {
            loop {}
        }
    }

    println!("diverges_or_returns(true) = {}", diverges_or_returns(true));

    println!("\n动态大小类型（DST）：");
    let s: &str = "Hello";
    println!("字符串切片：{}", s);

    let arr: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &arr;
    println!("数组切片：{:?}", slice);

    println!("\nSized trait：");
    println!("所有编译时已知大小的类型都实现了 Sized");
    println!("泛型默认要求 Sized：fn foo<T>() 等价于 fn foo<T: Sized>()");
    println!("使用 ?Sized 放宽限制：fn foo<T: ?Sized>(t: &T)");
}

/// 演示高级函数和闭包
fn demo_advanced_functions() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 高级函数和闭包                  │");
    println!("└─────────────────────────────────────┘");

    println!("\n函数指针：");
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("do_twice(add_one, 5) = {}", answer);

    println!("\n函数作为参数：");
    fn apply_to_list(list: Vec<i32>, f: fn(i32) -> i32) -> Vec<i32> {
        list.into_iter().map(f).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = apply_to_list(numbers, |x| x * 2);
    println!("应用函数后：{:?}", doubled);

    println!("\n返回闭包：");
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    let add_one = returns_closure();
    println!("returns_closure()(5) = {}", add_one(5));

    println!("\n闭包捕获模式：");
    let x = 5;
    let capture_ref = || println!("捕获引用：{}", x);
    capture_ref();

    let mut y = 5;
    let mut capture_mut = || {
        y += 1;
        println!("捕获可变引用：{}", y);
    };
    capture_mut();

    let s = String::from("hello");
    let capture_move = move || println!("移动捕获：{}", s);
    capture_move();

    println!("\n闭包类型推断：");
    let closure = |x| x + 1;
    println!("closure(5) = {}", closure(5));
    // println!("closure(5.0) = {}", closure(5.0)); // 错误：类型已推断为 i32
}

/// 演示宏
fn demo_macros() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 宏                              │");
    println!("└─────────────────────────────────────┘");

    println!("\n声明式宏：");
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $( temp_vec.push($x); )*
                temp_vec
            }
        };
    }

    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec! 宏创建：{:?}", v);

    println!("\n多模式宏：");
    macro_rules! say_hello {
        () => {
            println!("Hello!");
        };
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
        ($name:expr, $greeting:expr) => {
            println!("{}, {}!", $greeting, $name);
        };
    }

    say_hello!();
    say_hello!("World");
    say_hello!("Rust", "Welcome");

    println!("\n重复模式：");
    macro_rules! build_string {
        ( $( $part:expr ),+ ) => {
            {
                let mut s = String::new();
                $(
                    s.push_str($part);
                )+
                s
            }
        };
    }

    let s = build_string!("Hello", " ", "World", "!");
    println!("build_string! 结果：{}", s);

    println!("\n宏 vs 函数：");
    println!("宏的优点：");
    println!("1. 可变参数");
    println!("2. 编译时代码生成");
    println!("3. 可以操作语法结构");

    println!("\n函数的优点：");
    println!("1. 更简单直观");
    println!("2. 更好的错误信息");
    println!("3. 更好的 IDE 支持");

    println!("\n过程宏（概念）：");
    println!("1. 派生宏：#[derive(Debug)]");
    println!("2. 属性宏：#[route(GET, \"/\")]");
    println!("3. 函数宏：sql!(SELECT * FROM users)");

    println!("\n常用内置宏：");
    println!("vec!：创建向量");
    println!("println!：打印输出");
    println!("format!：格式化字符串");
    println!("panic!：触发 panic");
    println!("assert!：断言");
    println!("todo!：标记未实现代码");
    println!("unimplemented!：标记未实现代码");
}

/// 演示模式匹配高级用法
fn demo_advanced_patterns() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. 高级模式匹配                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n匹配守卫：");
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("小于 5：{}", x),
        Some(x) => println!("大于等于 5：{}", x),
        None => println!("没有值"),
    }

    println!("\n绑定模式：");
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 1..=7 } => println!("ID 在 1-7 之间：{}", id),
        Message::Hello { id: 10..=12 } => println!("ID 在 10-12 之间"),
        Message::Hello { id } => println!("其他 ID：{}", id),
    }

    println!("\n解构复杂类型：");
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y } => println!("在点 ({}, {})", x, y),
    }

    println!("\n嵌套解构：");
    let ((a, b), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("a={}, b={}, x={}, y={}", a, b, x, y);

    println!("\n忽略值：");
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("部分值：{}, {}, {}", first, third, fifth);
        }
    }

    println!("\n使用 .. 忽略剩余部分：");
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x 坐标：{}", x),
    }
}

/// 公开的演示函数
pub fn demo_advanced() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块九：高级特性                    ║");
    println!("╚═════════════════════════════════════════╝");

    demo_unsafe();
    demo_advanced_types();
    demo_advanced_functions();
    demo_macros();
    demo_advanced_patterns();
}
