pub fn main() {
    demo01();
    loop_demo();
}

fn demo01() {
    // 例子1: 检查两个条件是否都为真
    let x = 5;
    let y = 10;
    if x > 0 && y > 0 {
        println!("Both x and y are greater than 0");
    } else {
        println!("At least one of x or y is not greater than 0");
    }
}

fn loop_demo() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; //退出内部循环
            }
            if count == 2 {
                break 'counting_up; //退出外部循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn refer_demo() {
    let x = "test";
}
