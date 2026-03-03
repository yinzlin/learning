# 步骤 18：并发编程学习

## 操作目标
掌握 Rust 的并发编程，包括线程、消息传递、共享状态和同步原语。

## 执行方法

### 1. 线程

#### 学习内容
- 线程的创建和管理
- 线程的等待
- 线程的错误处理

#### 示例代码
```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("线程中: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // 主线程
    for i in 1..5 {
        println!("主线程中: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // 等待线程结束
    handle.join().unwrap();
}

// 传递数据给线程
fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("向量: {:?}", v);
    });
    
    handle.join().unwrap();
    // 不能再使用 v，因为所有权已经转移到线程中
}
```

### 2. 消息传递

#### 学习内容
- 通道的创建和使用
- 消息的发送和接收
- 通道的关闭

#### 示例代码
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建通道
    let (tx, rx) = mpsc::channel();
    
    // 创建发送线程
    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
        // 不能再使用 val，因为所有权已经转移
    });
    
    // 接收消息
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}

// 多消息发送
fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("你好"),
            String::from("来自"),
            String::from("另一个"),
            String::from("线程"),
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // 接收所有消息
    for received in rx {
        println!("收到: {}", received);
    }
}

// 多发送者
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("来自线程 1: 你好"),
            String::from("来自线程 1: 你好吗？"),
        ];
        
        for msg in messages {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    thread::spawn(move || {
        let messages = vec![
            String::from("来自线程 2: 嗨"),
            String::from("来自线程 2: 再见"),
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
}
```

### 3. 共享状态

#### 学习内容
- 互斥锁（Mutex）
- 原子类型
- 死锁的避免

#### 示例代码
```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // 创建互斥锁
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("结果: {}", *counter.lock().unwrap());
}

// 原子类型
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = AtomicUsize::new(0);
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("结果: {}", counter.load(Ordering::SeqCst));
}
```

### 4. 同步原语

#### 学习内容
- 条件变量
- 屏障
- 信号量

#### 示例代码
```rust
// 条件变量
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });
    
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    
    println!("线程已启动");
}

// 屏障
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(3));
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            println!("线程 {} 正在等待", i);
            barrier.wait();
            println!("线程 {} 继续执行", i);
        });
    }
}
```

## 所需资源
- Rust 文档
- 文本编辑器
- 终端

## 时间节点
- **开始时间**：学习计划第 45 天
- **完成时间**：学习计划第 47 天

## 预期成果
- 掌握线程的创建和管理
- 理解消息传递的并发模型
- 能够使用共享状态进行并发编程
- 了解同步原语的使用
- 能够避免常见的并发问题

## 优先级
- **优先级**：中
- **理由**：并发编程是现代软件的重要组成部分，Rust 提供了安全的并发编程模型

## 风险点与应对措施

| 风险点 | 应对措施 |
|-------|--------|
| 数据竞争 | 使用互斥锁或消息传递确保线程安全 |
| 死锁 | 避免循环依赖，使用超时机制 |
| 线程恐慌 | 正确处理线程错误，使用 join() 等待线程结束 |
| 性能问题 | 合理使用并发，避免过度线程化 |

## 验证步骤

1. 编写使用线程的代码
2. 实现消息传递的并发模型
3. 使用共享状态进行并发编程
4. 尝试使用不同的同步原语
5. 执行 `cargo check` 检查代码是否正确
6. 执行 `cargo run` 运行代码，确认输出正确

## 后续步骤

完成并发编程学习后，继续学习 [步骤 19：异步编程学习](step19-异步编程.md)。
