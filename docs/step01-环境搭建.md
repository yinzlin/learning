# 步骤 1：环境搭建

## 操作目标
安装并配置 Rust 开发环境，确保能够正常编译和运行 Rust 程序。

## 执行方法

### 1. 安装 Rust

#### Linux/macOS
1. 打开终端
2. 执行以下命令：
   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```
3. 按照提示完成安装过程

#### Windows
1. 访问 [Rust 官网](https://www.rust-lang.org) 下载安装程序
2. 运行安装程序并按照提示完成安装
3. 注意：Windows 系统可能需要安装 Visual Studio 提供的链接器

### 2. 验证安装

1. 打开终端
2. 执行以下命令：
   ```bash
   rustc --version
   ```
3. 如果安装成功，将显示 Rust 的版本信息，例如：
   ```
   rustc 1.90.0 (xxxxxxxxxxxx 2025-09-18)
   ```

### 3. 更新 Rust

如果需要更新 Rust 到最新版本，执行以下命令：

```bash
rustup update
```

### 4. 查看本地文档

安装 Rust 时会自动附带本地文档，通过以下命令可在浏览器中打开离线文档：

```bash
rustup doc --book
```

## 所需资源
- 互联网连接
- 操作系统权限
- 足够的磁盘空间（约 1-2GB）

## 时间节点
- **开始时间**：学习计划第 1 天
- **完成时间**：学习计划第 1 天

## 预期成果
- 成功安装 Rust 1.90.0 或更高版本
- 能够通过命令行验证 Rust 安装状态
- 能够访问本地 Rust 文档

## 优先级
- **优先级**：高
- **理由**：环境搭建是后续所有学习和实践的基础

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| Windows 系统缺少 Visual Studio 链接器 | 按照安装提示安装 Visual Studio Build Tools 或 Visual Studio |
| 网络连接问题导致安装失败 | 检查网络连接，或使用离线安装包 |
| 权限不足导致安装失败 | 以管理员身份运行安装程序 |

## 验证步骤

1. 打开终端
2. 执行 `rustc --version`，确认显示版本信息
3. 执行 `cargo --version`，确认 Cargo 包管理器已安装
4. 执行 `rustup doc --book`，确认能够打开本地文档

## 后续步骤

完成环境搭建后，继续学习 [步骤 2：Hello World 实践](step02-hello-world.md)。
