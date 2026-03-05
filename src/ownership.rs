/// 所有权系统示例模块
/// 涵盖：所有权概念、引用与借用、切片类型

/// 演示所有权概念
fn demo_ownership_concepts() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 所有权概念                      │");
    println!("└─────────────────────────────────────┘");

    println!("\n所有权转移（Move）：");
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // 错误：s1 的值已被移动
    println!("s1 的所有权转移给 s2，s2 = {}", s2);

    println!("\n复制语义（Copy）：");
    let x = 5;
    let y = x;
    println!("基本类型实现 Copy trait：x = {}, y = {}", x, y);

    println!("\n克隆语义（Clone）：");
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("显式克隆：s3 = {}, s4 = {}", s3, s4);

    println!("\n所有权与函数：");
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // 错误：s 的值已被移动

    let x = 5;
    makes_copy(x);
    println!("x 仍然有效：{}", x);
}

fn takes_ownership(some_string: String) {
    println!("函数获取所有权：{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("函数复制值：{}", some_integer);
}

/// 演示引用与借用
fn demo_references() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 引用与借用                      │");
    println!("└─────────────────────────────────────┘");

    println!("\n不可变引用：");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("字符串 '{}' 的长度是 {}", s1, len);

    println!("\n可变引用：");
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后的字符串：{}", s2);

    println!("\n引用的作用域：");
    let mut s3 = String::from("hello");
    let r1 = &s3;
    let r2 = &s3;
    println!("多个不可变引用：r1 = {}, r2 = {}", r1, r2);

    let r3 = &mut s3;
    r3.push_str(" world");
    println!("可变引用：r3 = {}", r3);

    println!("\n悬垂引用预防：");
    let string = no_dangle();
    println!("安全返回：{}", string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

/// 演示切片类型
fn demo_slices() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 切片类型                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n字符串切片：");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("切片：hello = '{}', world = '{}'", hello, world);

    println!("\n切片语法糖：");
    let slice1 = &s[0..5];
    let _slice2 = &s[..5];
    println!("&s[0..5] = &s[..5] = '{}'", slice1);

    let len = s.len();
    let slice3 = &s[6..len];
    let _slice4 = &s[6..];
    println!("&s[6..len] = &s[6..] = '{}'", slice3);

    let full_slice = &s[..];
    println!("完整切片 &s[..] = '{}'", full_slice);

    println!("\n数组切片：");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片 [1..3] = {:?}", slice);

    println!("\n实际应用：查找第一个单词");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词：'{}'", word);

    let s2 = String::from("hello");
    let word2 = first_word(&s2);
    println!("第一个单词：'{}'", word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 公开的演示函数
pub fn demo_ownership() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块二：所有权系统                  ║");
    println!("╚═════════════════════════════════════════╝");

    demo_ownership_concepts();
    demo_references();
    demo_slices();
}
