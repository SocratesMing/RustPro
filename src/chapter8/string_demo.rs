use std::pin::pin;

pub fn main() {
    create_String();
    update_String();
    display_split();
}

fn display_split() {
    // String 存储的是 Vec<u8> 因此len显示的是字节byte的个数
    let s1 = String::from("hola");
    let s2 = String::from("你好"); //一个中文对应三个字节
    println!("s1-len:{} s2-len:{}", s1.len(), s2.len());

    let s3 = "你好啊";

    // println!("split {}", &s3[..2]);   // 会报错，因为String是bytes存储的，没有到字符的边界

    for x in s2.chars() {
        println!("chars {}", x);
    }

    println!("--------");

    for m in s2.bytes() {
        println!("bytes: {}", m)
    }
}

fn create_String() {
    let s1 = String::from("hello");

    let s2 = "world".to_string();
    println!("{} {}", s1, s2);
}

fn update_String() {
    let mut s1 = String::from("first");
    s1.push_str(" second "); // 拼接字符串
    s1.push('C'); // 拼接单个字符

    println!("s1 {}", s1);

    let s2 = String::from(" next");
    let s3 = s1 + &s2; //使用+连接

    println!("s2 {}", s2);
    println!("s3 {}", s3);
    // println!("s1 {}", s1); //S1的发生了移动，所有权转移了

    // format!不会获得所有权，而且会返回字符串
    let s4 = format!("s3{} s2{}", s3, s2);
    println!("s4:{}", s4)
}
