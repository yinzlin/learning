# 步骤 7：结构体学习

## 操作目标
掌握结构体的定义和使用，包括结构体的定义、实例化、方法实现等。

## 执行方法

### 1. 结构体定义和实例化

#### 学习内容
- 结构体的定义
- 结构体的实例化
- 结构体字段的访问
- 结构体的可变与不可变

#### 示例代码
```rust
// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 实例化结构体
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 访问结构体字段
println!("用户名: {}", user1.username);

// 可变结构体
let mut user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername456"),
    active: true,
    sign_in_count: 1,
};

// 修改可变结构体的字段
user2.email = String::from("updated@example.com");

// 结构体更新语法
let user3 = User {
    email: String::from("third@example.com"),
    ..user1
};
```

### 2. 元组结构体

#### 学习内容
- 元组结构体的定义
- 元组结构体的实例化
- 元组结构体字段的访问

#### 示例代码
```rust
// 定义元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 实例化元组结构体
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// 访问元组结构体字段
println!("黑色: R={}, G={}, B={}", black.0, black.1, black.2);
```

### 3. 方法实现

#### 学习内容
- 方法的定义
- 方法的调用
- `self` 参数
- 关联函数

#### 示例代码
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 带参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 关联函数（没有 self 参数）
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 使用方法
let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("矩形面积: {}", rect1.area());

let rect2 = Rectangle {
    width: 10,
    height: 40,
};

println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));

// 使用关联函数
let square = Rectangle::square(20);
println!("正方形面积: {}", square.area());
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 11 天
- **完成时间**：学习计划第 12 天

## 预期成果
- 能够定义和实例化结构体
- 掌握结构体字段的访问和修改
- 能够为结构体实现方法
- 理解关联函数的使用

## 优先级
- **优先级**：高
- **理由**：结构体是 Rust 中组织数据的重要方式，是面向对象编程的基础

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 结构体字段访问错误 | 确保使用正确的字段名和访问方式 |
| 方法实现错误 | 注意 `self` 参数的使用，确保方法签名正确 |
| 所有权问题 | 注意方法中对 `self` 的借用方式 |

## 验证步骤

1. 定义一个结构体并实例化
2. 实现结构体的方法
3. 调用结构体的方法和关联函数
4. 执行 `cargo check` 检查代码是否正确
5. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成结构体学习后，继续学习 [步骤 8：枚举与模式匹配](step08-枚举与模式匹配.md)。
