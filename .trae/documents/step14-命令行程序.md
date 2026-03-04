# 步骤 14：命令行程序开发

## 操作目标
构建一个完整的命令行程序，学习处理命令行参数、读取文件、错误处理等核心功能。

## 执行方法

### 1. 接受命令行参数

#### 学习内容
- 命令行参数的获取
- 命令行参数的解析
- 使用 clap 库处理命令行参数

#### 示例代码
```rust
// 基本命令行参数获取
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("命令行参数: {:?}", args);
    
    if args.len() < 2 {
        println!("使用方法: {} <参数>", args[0]);
        return;
    }
    
    let query = &args[1];
    let file_path = &args[2];
    
    println!("搜索: {}", query);
    println!("在文件: {} 中搜索", file_path);
}

// 使用 clap 库
// Cargo.toml
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    query: String,
    
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    
    println!("搜索: {}", args.query);
    println!("在文件: {} 中搜索", args.file);
}
```

### 2. 读取文件

#### 学习内容
- 文件的打开和读取
- 文件内容的处理
- 错误处理

#### 示例代码
```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];
    
    println!("搜索: {}", query);
    println!("在文件: {} 中搜索", file_path);
    
    let contents = match File::open(file_path) {
        Ok(file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => contents,
                Err(e) => {
                    println!("读取文件错误: {}", e);
                    return;
                }
            }
        },
        Err(e) => {
            println!("打开文件错误: {}", e);
            return;
        }
    };
    
    println!("文件内容:\n{}", contents);
}

// 使用 ? 操作符简化
fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### 3. 重构与错误处理

#### 学习内容
- 代码重构
- 错误处理的改进
- 模块划分

#### 示例代码
```rust
// main.rs
use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        println!("参数错误: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = lib::run(config) {
        println!("应用错误: {}", e);
        process::exit(1);
    }
}

// lib.rs
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        
        assert_eq!(vec!["Rust:\nsafe, fast, productive."], search(query, contents));
    }
}
```

### 4. 测试驱动开发

#### 学习内容
- 测试驱动开发的流程
- 编写测试用例
- 实现功能并验证

#### 示例代码
```rust
// 首先编写测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        
        assert_eq!(
            vec!["Rust:\nsafe, fast, productive.", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// 然后实现功能
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}
```

### 5. 环境变量

#### 学习内容
- 环境变量的读取
- 环境变量的使用场景

#### 示例代码
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("参数错误: {}", err);
        process::exit(1);
    });
    
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
    if let Err(e) = run(config, case_sensitive) {
        println!("应用错误: {}", e);
        process::exit(1);
    }
}

pub fn run(config: Config, case_sensitive: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}
```

### 6. 错误重定向

#### 学习内容
- 标准输出和标准错误
- 错误重定向

#### 示例代码
```rust
use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(io::stderr(), "参数错误: {}", err).unwrap();
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        writeln!(io::stderr(), "应用错误: {}", e).unwrap();
        process::exit(1);
    }
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端
- clap 库（可选）

## 时间节点
- **开始时间**：学习计划第 29 天
- **完成时间**：学习计划第 35 天

## 预期成果
- 能够构建功能完整的命令行程序
- 掌握命令行参数的处理
- 理解文件读取和错误处理
- 学会使用测试驱动开发
- 能够处理环境变量和错误重定向

## 优先级
- **优先级**：高
- **理由**：命令行程序是 Rust 的重要应用场景，通过实际项目学习能够巩固之前的知识

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 错误处理不当 | 确保正确处理所有可能的错误情况 |
| 代码结构混乱 | 合理划分模块，保持代码清晰 |
| 测试覆盖不足 | 编写充分的测试用例 |

## 验证步骤

1. 创建命令行程序项目
2. 实现命令行参数处理
3. 实现文件读取功能
4. 编写测试用例
5. 处理环境变量和错误重定向
6. 执行 `cargo run` 运行程序，确认功能正常
7. 执行 `cargo test` 运行测试，确认测试通过

## 后续步骤

完成命令行程序开发后，继续学习 [步骤 15：函数式编程学习](step15-函数式编程.md)。
