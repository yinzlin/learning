/// 并发编程示例模块
/// 涵盖：线程、消息传递、共享状态、同步原语

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 演示线程
fn demo_threads() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 线程                            │");
    println!("└─────────────────────────────────────┘");

    println!("\n创建线程：");
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程计数：{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=3 {
        println!("主线程计数：{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!("线程执行完成");

    println!("\n使用 move 闭包：");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("从线程中访问向量：{:?}", v);
    });

    handle.join().unwrap();

    println!("\n线程返回值：");
    let handle = thread::spawn(|| {
        let result = 1 + 2 + 3 + 4 + 5;
        result
    });

    let result = handle.join().unwrap();
    println!("线程返回值：{}", result);

    println!("\n线程构建器：");
    let handle = thread::Builder::new()
        .name("my-thread".to_string())
        .spawn(|| {
            println!(
                "线程名称：{:?}",
                thread::current().name()
            );
        })
        .unwrap();

    handle.join().unwrap();
}

/// 演示消息传递
fn demo_channels() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 消息传递                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本通道：");
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("收到消息：{}", received);

    println!("\n发送多个消息：");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("你好"),
            String::from("来自"),
            String::from("线程"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("收到：{}", received);
    }

    println!("\n多发送者：");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx1.send(String::from("来自线程 1")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("来自线程 2")).unwrap();
    });

    drop(tx);

    for received in rx {
        println!("收到：{}", received);
    }

    println!("\n通道类型：");
    println!("mpsc：多生产者，单消费者");
    println!("spmc：单生产者，多消费者（需要外部库）");
    println!("mpmc：多生产者，多消费者（需要外部库）");
}

/// 演示共享状态
fn demo_shared_state() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 共享状态                        │");
    println!("└─────────────────────────────────────┘");

    println!("\nMutex<T> 互斥锁：");
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

    println!("计数器结果：{}", *counter.lock().unwrap());

    println!("\nMutex 的使用模式：");
    println!("1. 使用 Arc 包装以实现多线程共享");
    println!("2. lock() 获取锁");
    println!("3. unwrap() 处理可能的错误");
    println!("4. 离开作用域自动释放锁");

    println!("\n死锁预防：");
    println!("1. 避免嵌套锁");
    println!("2. 按固定顺序获取锁");
    println!("3. 使用 try_lock 避免阻塞");
    println!("4. 限制锁的持有时间");
}

/// 演示同步原语
fn demo_sync_primitives() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 同步原语                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n原子类型：");
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("原子计数器结果：{}", counter.load(Ordering::SeqCst));

    println!("\n内存顺序：");
    println!("Relaxed：最宽松，只保证原子性");
    println!("Acquire：读操作，防止后续操作重排到前面");
    println!("Release：写操作，防止前面操作重排到后面");
    println!("SeqCst：最严格，保证所有线程看到相同顺序");

    println!("\nBarrier 屏障：");
    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(3));

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            println!("线程 {} 到达屏障", i);
            barrier.wait();
            println!("线程 {} 继续执行", i);
        });
    }

    thread::sleep(Duration::from_millis(100));

    println!("\nOnce 一次性执行：");
    use std::sync::Once;

    static INIT: Once = Once::new();
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = thread::spawn(|| {
            INIT.call_once(|| {
                println!("只执行一次的初始化");
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 演示并发最佳实践
fn demo_best_practices() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. 并发最佳实践                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n1. 优先使用消息传递而非共享状态");
    println!("   - 使用 channel 进行线程间通信");
    println!("   - 避免复杂的锁机制");

    println!("\n2. 合理使用锁");
    println!("   - 限制锁的作用域");
    println!("   - 避免嵌套锁");
    println!("   - 考虑使用 RwLock 替代 Mutex");

    println!("\n3. 避免数据竞争");
    println!("   - 使用 Arc 包装共享数据");
    println!("   - 使用 Mutex 或 RwLock 保护可变数据");
    println!("   - 使用原子类型进行简单操作");

    println!("\n4. 错误处理");
    println!("   - 处理线程 panic");
    println!("   - 使用 join 等待线程完成");
    println!("   - 考虑使用线程池");

    println!("\n5. 性能考虑");
    println!("   - 避免过度线程化");
    println!("   - 使用线程池管理线程");
    println!("   - 考虑使用异步编程替代");
}

/// 公开的演示函数
pub fn demo_concurrency() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块八：并发编程                    ║");
    println!("╚═════════════════════════════════════════╝");

    demo_threads();
    demo_channels();
    demo_shared_state();
    demo_sync_primitives();
    demo_best_practices();
}
