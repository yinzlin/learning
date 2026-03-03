use std::io;

fn main() {
    println!("猜数字游戏");
    let secret_number = 7;
    println!("请输入你猜的数字（1-100）：");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("请输入一个数字");
    println!("你猜的数字是：{}", guess);
    if guess == secret_number {
        println!("恭喜你，猜对了！");
    } else {
        println!("很遗憾，猜错了。正确答案是：{}", secret_number);
    }
}
