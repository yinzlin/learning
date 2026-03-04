# 步骤 23：多线程 web 服务器开发

## 操作目标
构建一个多线程 web 服务器，学习网络编程、多线程处理和优雅关闭等高级功能。

## 执行方法

### 1. 单线程服务器

#### 学习内容
- 基本的 TCP 服务器
- HTTP 请求处理
- 响应生成
- 基本的路由

#### 示例代码
```rust
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### 2. 多线程服务器

#### 学习内容
- 线程池的实现
- 任务分配
- 并发处理请求
- 线程安全

#### 示例代码
```rust
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers,
            sender,
        }
    }
    
    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("向所有工作线程发送终止消息");
        
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        println!("等待所有工作线程结束");
        
        for worker in &mut self.workers {
            println!("关闭工作线程 {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("工作线程 {} 收到新任务", id);
                    job();
                }
                Message::Terminate => {
                    println!("工作线程 {} 收到终止消息", id);
                    break;
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 与单线程版本相同
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let contents = std::fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### 3. 优雅关闭

#### 学习内容
- 信号处理
- 优雅关闭服务器
- 清理资源

#### 示例代码
```rust
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use ctrlc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    // 创建一个原子布尔值来控制服务器是否继续运行
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // 设置信号处理
    ctrlc::set_handler(move || {
        println!("收到终止信号，正在关闭服务器...");
        running_clone.store(false, Ordering::SeqCst);
    }).expect("设置信号处理失败");
    
    // 接受连接的线程
    thread::spawn(move || {
        for stream in listener.incoming() {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    });
    
    // 等待终止信号
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
    
    // 当离开作用域时，ThreadPool 的 drop 方法会被调用，从而关闭所有工作线程
    println!("服务器已关闭");
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端
- ctrlc 库（可选，用于信号处理）

## 时间节点
- **开始时间**：学习计划第 60 天
- **完成时间**：学习计划第 66 天

## 预期成果
- 能够构建基本的单线程 web 服务器
- 实现多线程服务器以提高性能
- 实现优雅关闭功能
- 理解网络编程和多线程处理的基本原理

## 优先级
- **优先级**：高
- **理由**：多线程 web 服务器是一个综合性项目，能够巩固之前学习的各种 Rust 知识

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 线程安全问题 | 使用适当的同步原语，如互斥锁和原子类型 |
| 资源泄漏 | 确保所有资源都能正确释放，使用 Drop trait |
| 性能问题 | 合理设置线程池大小，避免过度线程化 |
| 错误处理 | 正确处理网络错误和系统错误 |

## 验证步骤

1. 创建单线程 web 服务器
2. 测试基本的 HTTP 请求处理
3. 实现多线程服务器
4. 测试并发请求处理
5. 实现优雅关闭功能
6. 测试服务器的启动和关闭
7. 执行 `cargo check` 检查代码是否正确
8. 执行 `cargo run` 运行服务器，确认功能正常

## 后续步骤

完成多线程 web 服务器开发后，继续学习 [步骤 24：项目总结与反思](step24-总结与反思.md)。
