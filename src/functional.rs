/// 函数式编程示例模块
/// 涵盖：闭包、迭代器

/// 演示闭包
fn demo_closures() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 闭包                            │");
    println!("└─────────────────────────────────────┘");

    println!("\n闭包定义：");
    let add_one = |x| x + 1;
    let result = add_one(5);
    println!("add_one(5) = {}", result);

    println!("\n多参数闭包：");
    let add = |a, b| a + b;
    let result = add(5, 3);
    println!("add(5, 3) = {}", result);

    println!("\n带类型标注的闭包：");
    let add_typed = |a: i32, b: i32| -> i32 { a + b };
    println!("add_typed(10, 20) = {}", add_typed(10, 20));

    println!("\n捕获环境变量：");
    let x = 5;
    let add_x = |y| x + y;
    println!("x = {}, add_x(3) = {}", x, add_x(3));

    println!("\n可变闭包：");
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("increment() = {}", increment());
    println!("increment() = {}", increment());
    println!("increment() = {}", increment());

    println!("\n闭包作为参数：");
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    println!("apply(double, 5) = {}", apply(double, 5));

    println!("\n闭包 trait：");
    println!("FnOnce：消耗捕获的变量，只能调用一次");
    println!("FnMut：可变借用捕获的变量");
    println!("Fn：不可变借用捕获的变量");

    println!("\nFnOnce 示例：");
    fn consume<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let s = String::from("hello");
    let closure = move || println!("消耗：{}", s);
    consume(closure);
    // println!("{}", s); // 错误：s 已被移动

    println!("\nFnMut 示例：");
    fn mutate<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
        f();
    }

    let mut count = 0;
    mutate(|| {
        count += 1;
        println!("count = {}", count);
    });

    println!("\n返回闭包：");
    fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    let add_5 = make_adder(5);
    println!("add_5(10) = {}", add_5(10));
}

/// 演示迭代器
fn demo_iterators() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 迭代器                          │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本迭代器：");
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("值：{}", val);
    }

    println!("\nIterator trait：");
    println!("trait Iterator {{");
    println!("    type Item;");
    println!("    fn next(&mut self) -> Option<Self::Item>;");
    println!("}}");

    println!("\n手动调用 next：");
    let v2 = vec![1, 2, 3];
    let mut iter = v2.iter();
    println!("next() = {:?}", iter.next());
    println!("next() = {:?}", iter.next());
    println!("next() = {:?}", iter.next());
    println!("next() = {:?}", iter.next());

    println!("\n消费适配器：");
    let v3 = vec![1, 2, 3, 4, 5];
    let total: i32 = v3.iter().sum();
    println!("总和：{}", total);

    println!("\n迭代器适配器：");
    let v4: Vec<i32> = vec![1, 2, 3];
    let v5: Vec<i32> = v4.iter().map(|x| x + 1).collect();
    println!("map 后：{:?}", v5);

    println!("\n链式调用：");
    let v6 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = v6
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .take(3)
        .collect();
    println!("过滤偶数 -> 乘以 2 -> 取前 3：{:?}", result);

    println!("\n常用迭代器方法：");
    let v7 = vec![1, 2, 3, 4, 5];

    let sum: i32 = v7.iter().sum();
    println!("sum：{}", sum);

    let product: i32 = v7.iter().product();
    println!("product：{}", product);

    let any = v7.iter().any(|x| *x > 3);
    println!("any(>3)：{}", any);

    let all = v7.iter().all(|x| *x > 0);
    println!("all(>0)：{}", all);

    let find = v7.iter().find(|x| **x == 3);
    println!("find(3)：{:?}", find);

    let position = v7.iter().position(|x| *x == 3);
    println!("position(3)：{:?}", position);

    let count = v7.iter().count();
    println!("count：{}", count);

    let (even, odd): (Vec<i32>, Vec<i32>) = v7.iter().partition(|x| *x % 2 == 0);
    println!("partition：偶数 {:?}, 奇数 {:?}", even, odd);

    println!("\n自定义迭代器：");
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new(5);
    for num in counter {
        print!("{} ", num);
    }
    println!();

    println!("\n迭代器组合：");
    let sum: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("复杂组合结果：{}", sum);

    println!("\n迭代器性能：");
    println!("迭代器是零成本抽象，编译后与手写循环性能相当");
}

/// 公开的演示函数
pub fn demo_functional() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块六：函数式编程                  ║");
    println!("╚═════════════════════════════════════════╝");

    demo_closures();
    demo_iterators();
}
