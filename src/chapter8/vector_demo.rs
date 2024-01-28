use std::net::IpAddr::V4;
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
pub fn main() {
    create_and_iter();
    push_members();
    get_members();
}
fn vec_enum() {
    // 使用可附加数据的enum类型，使得vec可以存储多种类型
    let row = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(32.1),
        SpreadsheetCell::Text(String::from("text")),
    ];
}
fn create_and_iter() {
    // 创建vector 使用new函数
    let v: Vec<i32> = Vec::new();

    //v小写，适用类型推断直接创建
    let mut v2 = vec![1, 2, 3];
    let first = &v2[0];
    v2.push(4); //新版不会报错了
    println!("v2 is {:?}", v2);

    for x in &mut v2 {
        *x *= 2; //解引用v2
    }

    for x in &v2 {
        println!("{}", x);
    }
}

fn push_members() {
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    v3.push(4);

    println!("v3:{:?}", v3)
}

fn get_members() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; //超出索引会panic
    println!("th third element is {}", third);

    match v.get(2) {
        //get不会panic
        None => {
            println!("there is no third element");
        }
        Some(third) => {
            println!("th third element is {}", third);
        }
    }
}
