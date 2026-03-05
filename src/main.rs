/// IP 地址枚举，演示带数据的枚举变体
enum IpAddr {
    /// IPv4 地址，包含 4 个 u8 值
    V4(u8, u8, u8, u8),
    /// IPv6 地址，包含一个 String
    V6(String),
}

impl IpAddr {
    /// 显示 IP 地址
    fn display(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("IPv6: {}", addr),
        }
    }
}

/// 硬币枚举，演示简单枚举变体
#[derive(Debug)]
enum Coin {
    Penny,   // 1 美分
    Nickel,  // 5 美分
    Dime,    // 10 美分
    Quarter, // 25 美分
}

/// 根据硬币类型返回价值（美分）
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/// 将 Option<i32> 中的值加一
/// 演示 Option 的模式匹配
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // ========== 1. 枚举定义与使用 ==========
    println!("=== 1. 枚举定义与使用 ===");

    // 创建 IPv4 和 IPv6 地址实例
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 调用枚举方法
    home.display();
    loopback.display();

    // ========== 2. Option 枚举 ==========
    println!("\n=== 2. Option 枚举 ===");

    // Option 是标准库中的枚举，用于表示可能存在的值
    let some_number = Some(5);           // 有值
    let absent_number: Option<i32> = None; // 无值

    // 对 Option 进行操作
    let six = plus_one(some_number);
    let none = plus_one(absent_number);

    println!("Some(5) + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);

    // ========== 3. match 控制流 ==========
    println!("\n=== 3. match 控制流 ===");

    // match 必须穷举所有可能
    let coin = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("{:?} 的价值是 {} 美分", Coin::Quarter, value);

    // 通配模式：使用 other 捕获其他所有值
    let dice_roll = 9;
    match dice_roll {
        3 => println!("掷出 3，添加帽子"),
        7 => println!("掷出 7，移除帽子"),
        other => println!("掷出 {}，移动玩家", other), // other 捕获其他所有情况
    }

    // ========== 4. if let 简化 ==========
    println!("\n=== 4. if let 简化 ===");

    // 当只关心一种模式时，可以用 if let 简化
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("值是 3");
    } else {
        println!("值不是 3");
    }

    // ========== 5. let...else ==========
    println!("\n=== 5. let...else ===");

    // let...else 语法：匹配成功则绑定变量，否则执行 else 分支
    let maybe_value = Some(42);
    let Some(x) = maybe_value else {
        // 如果不是 Some，执行这里并返回
        println!("没有值");
        return;
    };
    // 如果是 Some，继续执行这里
    println!("值是: {}", x);
}
