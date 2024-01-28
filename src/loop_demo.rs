pub(crate) fn main() {
    println!("----------循环demo---------");
    // demo01();
    // demo02();
    demo03();
    demo04();
    demo05();
}

fn demo04() {
    println!("----------while---------");

    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        println!("x: {x}");
    }
    println!("x: {x}");
}

fn demo05() {
    println!("----------用于创建无限循环---------");

    let mut x = 10;
    loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        if x == 1 {
            break;
        }
        println!("x: {x}");
    }
}
fn demo03() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }

    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}

fn demo01() {
    println!("----------for循环---------");

    let array = [10, 20, 30];
    print!("直接打印数组:");
    println!("array {array:?}");
    print!("直接遍历数组:");

    for x in array {
        print!(" {x}")
    }
    println!(); //这个打印有换行符号

    print!("通过索引遍历数组:");

    for i in 0..array.len() {
        print!("{} ", array[i]);
    }
}

fn demo02() {
    let matrix = [
        [101, 102, 103], // <-- 这个注释会让 rust fmt 添加一个新行
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);
    let transposed = transpose(matrix);
    println!("matrix:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    unimplemented!()
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    unimplemented!()
}
