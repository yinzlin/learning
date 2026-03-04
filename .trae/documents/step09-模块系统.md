# 步骤 9：模块系统学习

## 操作目标
掌握 Rust 的模块系统，包括包、crate、模块的定义和使用。

## 执行方法

### 1. 包和 Crate

#### 学习内容
- 包（Package）的概念
- Crate 的概念
- Crate 的类型（二进制 crate 和库 crate）
- Cargo 项目结构

#### 示例代码
```rust
// Cargo.toml 文件定义包
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"

[dependencies]
```

### 2. 模块和作用域

#### 学习内容
- 模块的定义
- 模块的作用域
- 模块的可见性（pub 关键字）
- 模块的嵌套

#### 示例代码
```rust
// 定义模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
        
        fn seat_at_table() {
            println!("安排座位");
        }
    }
    
    mod serving {
        fn take_order() {
            println!("点餐");
        }
        
        fn serve_order() {
            println!("上菜");
        }
    }
}

// 使用模块中的函数
fn main() {
    front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::seat_at_table(); // 编译错误，因为 seat_at_table 不是 pub
    // front_of_house::serving::take_order(); // 编译错误，因为 serving 不是 pub
}
```

### 3. 路径引用

#### 学习内容
- 绝对路径
- 相对路径
- 路径的使用

#### 示例代码
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
    }
}

// 使用绝对路径
fn main() {
    crate::front_of_house::hosting::add_to_waitlist();
}

// 使用相对路径
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    
    fn cook_order() {
        println!("烹饪订单");
    }
}

fn serve_order() {
    println!("上菜");
}
```

### 4. use 关键字

#### 学习内容
- use 关键字的使用
- 简化路径
- 重命名导入
- 嵌套导入

#### 示例代码
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
    }
}

// 使用 use 关键字简化路径
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}

// 重命名导入
use crate::front_of_house::hosting as h;

fn main2() {
    h::add_to_waitlist();
}

// 嵌套导入
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
```

### 5. 多文件模块

#### 学习内容
- 模块文件结构
- 模块的分离
- 模块的重新导出

#### 示例代码
```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

// src/front_of_house.rs
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {
    println!("添加到等待列表");
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 15 天
- **完成时间**：学习计划第 17 天

## 预期成果
- 理解包和 crate 的概念
- 能够定义和使用模块
- 掌握模块的可见性控制
- 能够组织多文件模块结构

## 优先级
- **优先级**：中
- **理由**：模块系统是组织大型 Rust 项目的重要工具，对于构建复杂应用程序至关重要

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 路径引用错误 | 仔细检查路径，使用绝对路径或相对路径时确保正确性 |
| 可见性问题 | 确保使用 pub 关键字正确设置模块和函数的可见性 |
| 模块文件结构错误 | 遵循 Rust 的模块文件结构约定 |

## 验证步骤

1. 创建一个包含多个模块的项目
2. 定义模块和子模块
3. 设置适当的可见性
4. 使用 use 关键字简化路径
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成模块系统学习后，继续学习 [步骤 10：集合类型学习](step10-集合类型.md)。
