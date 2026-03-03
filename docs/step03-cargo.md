# 步骤 3：Cargo 项目管理

## 操作目标
学习使用 Cargo 管理 Rust 项目，了解 Cargo 的基本命令和项目结构。

## 执行方法

### 1. 创建新项目

1. 打开终端
2. 执行以下命令创建一个新的 Rust 项目：
   ```bash
   cargo new hello_cargo
   ```
3. 这将创建一个名为 `hello_cargo` 的目录，包含基本的项目结构。

### 2. 查看项目结构

1. 导航到项目目录：
   ```bash
   cd hello_cargo
   ```
2. 查看目录内容：
   ```bash
   ls -la
   ```
3. 项目结构应该如下：
   ```
   hello_cargo/
   ├── Cargo.toml
   └── src/
       └── main.rs
   ```

### 3. 查看 Cargo.toml 文件

`Cargo.toml` 是 Cargo 的配置文件，包含项目的元数据和依赖信息：

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### 4. 查看 src/main.rs 文件

`src/main.rs` 是项目的主源文件：

```rust
fn main() {
    println!("Hello, world!");
}
```

### 5. 构建项目

执行以下命令构建项目：

```bash
cargo build
```

这将在 `target/debug/` 目录中生成可执行文件。

### 6. 运行项目

执行以下命令运行项目：

```bash
cargo run
```

这将编译并运行程序，输出 "Hello, world!"。

### 7. 检查项目

执行以下命令检查项目是否有编译错误（不生成可执行文件）：

```bash
cargo check
```

## 所需资源
- 终端
- Rust 开发环境（已在步骤 1 中安装）

## 时间节点
- **开始时间**：学习计划第 2 天
- **完成时间**：学习计划第 2 天

## 预期成果
- 能够使用 Cargo 创建新项目
- 理解 Cargo 项目的基本结构
- 掌握 Cargo 的基本命令（build、run、check）
- 能够构建和运行 Cargo 项目

## 优先级
- **优先级**：高
- **理由**：Cargo 是 Rust 的官方包管理器和构建工具，是开发 Rust 项目的必备工具

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 依赖解析失败 | 检查网络连接，确保 Cargo 能够访问 crates.io |
| 构建错误 | 检查代码语法和依赖配置 |
| 权限问题 | 确保有项目目录的读写权限 |

## 验证步骤

1. 执行 `cargo new hello_cargo` 创建新项目
2. 执行 `cd hello_cargo` 进入项目目录
3. 执行 `cargo build` 构建项目
4. 执行 `cargo run` 运行项目
5. 确认终端显示 "Hello, world!"

## 后续步骤

完成 Cargo 项目管理学习后，继续学习 [步骤 4：猜数字游戏实现](step04-猜数字游戏.md)。
