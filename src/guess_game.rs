use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Read;

pub(crate) fn main() {
    println!("猜数游戏");
    let number = rand::thread_rng().gen_range(1..101);

    println!("请输入一个数字：");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是：{}", guess);
        match guess.cmp(&number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        };
    }
}
