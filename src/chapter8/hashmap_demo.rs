use std::collections::HashMap;

pub fn main() {
    create_hashmap();
    update_hashmap();
}

fn create_hashmap() {
    // 通过new函数创建
    let mut map = HashMap::new();

    map.insert(String::from("yellow"), 20);
    map.insert(String::from("blue"), 10);
    let red = String::from("red");
    let value = 33;
    map.insert(red, value);
    // println!("red {}", red);  // 报错，red发生了移动

    // 通过collect创建
    let teams = vec![String::from("yellow"), String::from("blue")];
    let scores = vec![20, 10];
    let map_zip: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    // hashmap获取值
    let yellow = String::from("yellow");
    let val = map_zip.get(&yellow);

    match val {
        None => {}
        Some(s) => {
            println!("{}", s)
        }
    }

    // 遍历hashmap
    for (k, v) in &map_zip {
        println!("{} - {}", k, v)
    }
}

fn update_hashmap() {
    let mut map = HashMap::new();
    map.insert(String::from("yellow"), 20);
    map.entry(String::from("blue")).or_insert(10);
    let mut new_val = map.entry(String::from("yellow")).or_insert(30);
    println!("new_val: {}", new_val);
    println!("{:?}", map);

    let string = String::from("hello world wonderful world");
    let mut map2 = HashMap::new();
    for word in string.split_ascii_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
}
