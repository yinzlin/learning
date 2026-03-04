# 步骤 19：异步编程学习

## 操作目标
掌握 Rust 的异步编程，包括 async/await 语法、Futures、Streams 等。

## 执行方法

### 1. Futures 和 async 语法

#### 学习内容
- async 函数的定义和使用
- await 操作符的使用
- Future trait 的理解
- 执行器的使用

#### 示例代码
```rust
// 基本 async 函数
async fn say_world() {
    println!("world");
}

async fn hello_world() {
    println!("hello");
    say_world().await;
}

// 使用 tokio 执行器
// Cargo.toml
// [dependencies]
// tokio = { version = "1.0", features = ["full"] }

#[tokio::main]
async fn main() {
    hello_world().await;
}

// 异步函数返回 Future
async fn get_hello() -> String {
    String::from("Hello")
}

#[tokio::main]
async fn main() {
    let hello = get_hello().await;
    println!("{}", hello);
}
```

### 2. 异步并发

#### 学习内容
- 并发执行多个异步任务
- join! 宏的使用
- select! 宏的使用
- 超时处理

#### 示例代码
```rust
// 并发执行多个任务
use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 完成");
}

async fn task2() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 完成");
}

#[tokio::main]
async fn main() {
    tokio::join!(task1(), task2());
    println!("所有任务完成");
}

// select! 宏
use tokio::time::{sleep, Duration};
use tokio::select;

async fn task_a() {
    sleep(Duration::from_secs(1)).await;
    "Task A"
}

async fn task_b() {
    sleep(Duration::from_secs(2)).await;
    "Task B"
}

#[tokio::main]
async fn main() {
    let result = select! {
        a = task_a() => a,
        b = task_b() => b,
    };
    println!("完成的任务: {}", result);
}

// 超时处理
use tokio::time::{sleep, Duration, timeout};

async fn long_running_task() {
    sleep(Duration::from_secs(5)).await;
    println!("长时间任务完成");
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(2), long_running_task()).await {
        Ok(_) => println!("任务在超时前完成"),
        Err(_) => println!("任务超时"),
    }
}
```

### 3. Streams

#### 学习内容
- Stream trait 的理解
- 流的创建和使用
- 流的转换和组合
- 流的消费

#### 示例代码
```rust
// 基本 Stream
use tokio::stream::StreamExt;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let mut stream = tokio::stream::iter(vec![1, 2, 3, 4, 5]);
    
    while let Some(item) = stream.next().await {
        println!("Item: {}", item);
        time::sleep(Duration::from_millis(500)).await;
    }
}

// 流的转换
use tokio::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let stream = stream::iter(vec![1, 2, 3, 4, 5])
        .map(|x| x * 2)
        .filter(|x| *x > 5);
    
    stream.for_each(|x| async move {
        println!("过滤后的项目: {}", x);
    }).await;
}

// 异步流
use tokio::stream::{self, StreamExt};
use tokio::time::{self, Duration};

async fn generate_numbers() -> impl stream::Stream<Item = u32> {
    stream::unfold(0, |state| async move {
        time::sleep(Duration::from_secs(1)).await;
        Some((state, state + 1))
    })
}

#[tokio::main]
async fn main() {
    let mut stream = generate_numbers();
    
    for _ in 0..5 {
        if let Some(number) = stream.next().await {
            println!("生成的数字: {}", number);
        }
    }
}
```

### 4. 异步 I/O

#### 学习内容
- 异步文件操作
- 异步网络操作
- 异步 HTTP 客户端
- 异步 Web 服务器

#### 示例代码
```rust
// 异步文件操作
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 写入文件
    let mut file = File::create("hello.txt").await?;
    file.write_all(b"Hello, world!").await?;
    
    // 读取文件
    let mut file = File::open("hello.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("文件内容: {}", contents);
    
    Ok(())
}

// 异步网络操作
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            
            loop {
                let n = socket.read(&mut buffer).await.expect("读取失败");
                
                if n == 0 {
                    break;
                }
                
                socket.write_all(&buffer[0..n]).await.expect("写入失败");
            }
        });
    }
}

// 异步 HTTP 客户端
// Cargo.toml
// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端
- tokio 库
- reqwest 库（可选）

## 时间节点
- **开始时间**：学习计划第 48 天
- **完成时间**：学习计划第 50 天

## 预期成果
- 掌握 async/await 语法
- 理解 Future 和 Stream 的概念
- 能够并发执行多个异步任务
- 能够使用异步 I/O 操作
- 理解异步编程的优势和使用场景

## 优先级
- **优先级**：中
- **理由**：异步编程是现代 Rust 开发的重要组成部分，特别适合 I/O 密集型应用

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 阻塞操作 | 确保在异步代码中不使用阻塞操作，使用异步版本的 API |
| 死锁 | 避免在持有锁的情况下 await |
| 内存泄漏 | 确保所有任务都能正常完成或被取消 |
| 错误处理 | 正确处理异步错误，使用 Result 和 ? 操作符 |

## 验证步骤

1. 编写使用 async/await 的代码
2. 实现并发执行多个异步任务
3. 使用 Stream 处理数据流
4. 尝试异步 I/O 操作
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成异步编程学习后，继续学习 [步骤 20：面向对象编程学习](step20-面向对象编程.md)。
