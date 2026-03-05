/// 数据结构示例模块
/// 涵盖：结构体、枚举、集合类型

/// 演示结构体
fn demo_structs() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 结构体                          │");
    println!("└─────────────────────────────────────┘");

    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    impl User {
        fn new(username: String, email: String) -> Self {
            Self {
                username,
                email,
                sign_in_count: 1,
                active: true,
            }
        }

        fn describe(&self) {
            println!("用户：{} ({})", self.username, self.email);
        }
    }

    println!("\n创建结构体实例：");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1 = {:?}", user1);

    println!("\n使用关联函数创建：");
    let user2 = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );
    user2.describe();

    println!("\n结构体更新语法：");
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user3 = {:?}", user3);

    println!("\n元组结构体：");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color({}, {}, {})", black.0, black.1, black.2);
    println!("Point({}, {}, {})", origin.0, origin.1, origin.2);

    println!("\n单元结构体：");
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    println!("\n方法示例：");
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(20);

    println!("rect1 = {:?}", rect1);
    println!("rect1 面积 = {}", rect1.area());
    println!("rect2 = {:?}", rect2);
    println!("rect1 能容纳 rect2？{}", rect1.can_hold(&rect2));
}

/// 演示枚举
fn demo_enums() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 枚举                            │");
    println!("└─────────────────────────────────────┘");

    println!("\n简单枚举：");
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:?}", four, six);

    println!("\n带数据的枚举：");
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);

    println!("\nOption 枚举：");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("absent_number = {:?}", absent_number);

    println!("\nmatch 表达式：");
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("幸运便士！");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("{:?} 的价值是 {} 美分", Coin::Quarter, value);

    println!("\n匹配 Option<T>：");
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("plus_one(Some(5)) = {:?}", six);
    println!("plus_one(None) = {:?}", none);

    println!("\n通配模式：");
    let dice_roll = 9;
    match dice_roll {
        3 => println!("掷出 3"),
        7 => println!("掷出 7"),
        _ => println!("掷出其他数字：{}", dice_roll),
    }

    println!("\nif let 简化：");
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("值是 3");
    } else {
        println!("值不是 3");
    }

    println!("\nlet...else 语法：");
    let maybe_value: Option<i32> = Some(42);
    if let Some(x) = maybe_value {
        println!("值是：{}", x);
    } else {
        println!("没有值");
    }
}

/// 演示集合类型
fn demo_collections() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 集合类型                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n向量（Vector）：");
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("动态创建：{:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("宏创建：{:?}", v2);

    println!("\n访问元素：");
    let third = &v2[2];
    println!("索引访问 v2[2] = {}", third);

    match v2.get(2) {
        Some(third) => println!("get 方法 v2.get(2) = {}", third),
        None => println!("没有第三个元素"),
    }

    println!("\n遍历向量：");
    for i in &v2 {
        print!("{} ", i);
    }
    println!();

    println!("\n遍历并修改：");
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }
    println!("修改后：{:?}", v3);

    println!("\n字符串（String）：");
    let s1 = String::new();
    println!("空字符串：'{}'", s1);

    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    println!("to_string：'{}'", s2);
    println!("String::from：'{}'", s3);

    let mut s4 = String::from("hello");
    s4.push_str(", world");
    s4.push('!');
    println!("修改后：'{}'", s4);

    println!("\n字符串拼接：");
    let s5 = String::from("hello");
    let s6 = String::from("world");
    let s7 = s5 + " " + &s6;
    println!("+ 运算符：'{}'", s7);

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("format! 宏：'{}'", s11);

    println!("\n遍历字符串：");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!("(按字符)");

    for b in "hello".bytes() {
        print!("{} ", b);
    }
    println!("(按字节)");

    println!("\n哈希映射（HashMap）：");
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);

    println!("\n访问值：");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue 队得分：{:?}", score);

    println!("\n遍历：");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("\n更新值：");
    scores.insert(String::from("Blue"), 25);
    println!("覆盖后：{:?}", scores);

    println!("\n只在键不存在时插入：");
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("entry 后：{:?}", scores);

    println!("\n根据旧值更新：");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数：{:?}", map);
}

/// 公开的演示函数
pub fn demo_data_structures() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块三：数据结构                    ║");
    println!("╚═════════════════════════════════════════╝");

    demo_structs();
    demo_enums();
    demo_collections();
}
