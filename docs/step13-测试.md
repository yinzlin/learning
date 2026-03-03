# 步骤 13：测试学习

## 操作目标
掌握 Rust 的测试系统，包括编写测试、控制测试运行和测试组织。

## 执行方法

### 1. 编写测试

#### 学习内容
- 测试函数的定义
- 断言宏的使用
- 测试失败的处理

#### 示例代码
```rust
// 测试函数
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn another() {
        panic!("测试失败");
    }
}

// 使用 assert! 宏
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        
        assert!(!smaller.can_hold(&larger));
    }
}

// 使用 assert_eq! 和 assert_ne! 宏
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// 使用 assert!(...), assert_eq!(...), assert_ne!(...)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

### 2. 控制测试运行

#### 学习内容
- 运行特定测试
- 并行与串行测试
- 显示测试输出
- 测试失败时的行为

#### 示例代码
```rust
// 运行特定测试
// 命令: cargo test it_works

// 运行多个测试
// 命令: cargo test works

// 忽略测试
#[test]
#[ignore]
fn expensive_test() {
    // 执行耗时操作
}

// 运行忽略的测试
// 命令: cargo test -- --ignored

// 串行测试
#[test]
#[serial]
fn test_with_shared_resource() {
    // 测试代码
}
```

### 3. 测试组织

#### 学习内容
- 单元测试
- 集成测试
- 文档测试

#### 示例代码
```rust
// 单元测试 (通常放在 src 目录中)
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// 集成测试 (放在 tests 目录中)
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// 文档测试
/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let result = adder::add_two(2);
/// assert_eq!(result, 4);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 27 天
- **完成时间**：学习计划第 28 天

## 预期成果
- 能够编写基本的测试函数
- 掌握断言宏的使用
- 理解如何控制测试运行
- 能够组织不同类型的测试

## 优先级
- **优先级**：中
- **理由**：测试是确保代码质量的重要手段，Rust 提供了强大的测试系统

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 测试覆盖不足 | 确保测试覆盖主要功能和边界情况 |
| 测试代码质量差 | 保持测试代码的清晰和可维护性 |
| 测试运行缓慢 | 合理组织测试，使用忽略属性标记耗时测试 |

## 验证步骤

1. 编写单元测试
2. 运行测试并观察结果
3. 编写集成测试
4. 尝试不同的测试运行命令
5. 执行 `cargo test` 运行所有测试

## 后续步骤

完成测试学习后，继续学习 [步骤 14：命令行程序开发](step14-命令行程序.md)。
