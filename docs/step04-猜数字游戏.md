# 步骤 4：猜数字游戏实现

## 操作目标
通过实现一个猜数字游戏，学习 Rust 的基本输入输出、随机数生成和控制流。

## 执行方法

### 1. 创建项目

1. 打开终端
2. 执行以下命令创建一个新的 Rust 项目：
   ```bash
   cargo new guessing_game
   ```
3. 导航到项目目录：
   ```bash
   cd guessing_game
   ```

### 2. 添加依赖

1. 打开 `Cargo.toml` 文件
2. 在 `[dependencies]` 部分添加 rand 依赖：
   ```toml
   [dependencies]
   rand = "0.8.5"
   ```

### 3. 编写代码

1. 打开 `src/main.rs` 文件
2. 替换为以下代码：
   ```rust
   use std::io;
   use rand::Rng;
   
   fn main() {
       println!("猜数字游戏");
       
       let secret_number = rand::thread_rng().gen_range(1..=100);
       
       loop {
           println!("请输入你猜的数字（1-100）：");
           
           let mut guess = String::new();
           io::stdin()
               .read_line(&mut guess)
               .expect("读取输入失败");
           
           let guess: u32 = match guess.trim().parse() {
               Ok(num) => num,
               Err(_) => {
                   println!("请输入一个有效的数字！");
                   continue;
               }
           };
           
           println!("你猜的数字是：{}", guess);
           
           match guess.cmp(&secret_number) {
               std::cmp::Ordering::Less => println!("太小了！"),
               std::cmp::Ordering::Greater => println!("太大了！"),
               std::cmp::Ordering::Equal => {
                   println!("恭喜你，猜对了！");
                   break;
               }
           }
       }
   }
   ```

### 4. 构建和运行项目

1. 执行以下命令构建项目：
   ```bash
   cargo build
   ```
2. 执行以下命令运行项目：
   ```bash
   cargo run
   ```

### 5. 测试游戏

1. 运行游戏后，输入数字进行猜测
2. 根据提示调整猜测的数字
3. 直到猜对为止

## 所需资源
- 终端
- Rust 开发环境（已在步骤 1 中安装）
- rand 依赖库

## 时间节点
- **开始时间**：学习计划第 3 天
- **完成时间**：学习计划第 4 天

## 预期成果
- 成功实现一个功能完整的猜数字游戏
- 掌握 Rust 的基本输入输出操作
- 学习随机数生成的使用
- 理解 Rust 的控制流和错误处理

## 优先级
- **优先级**：高
- **理由**：通过实际项目学习 Rust 的核心概念，加深对语言的理解

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 依赖添加失败 | 检查网络连接，确保 Cargo 能够访问 crates.io |
| 编译错误 | 检查代码语法和依赖版本 |
| 运行时错误 | 检查输入处理逻辑 |

## 验证步骤

1. 执行 `cargo run` 运行游戏
2. 输入数字进行猜测
3. 确认游戏能够正确提示数字大小
4. 确认猜对后游戏结束

## 后续步骤

完成猜数字游戏实现后，继续学习 [步骤 5：核心概念学习](step05-核心概念.md)。
