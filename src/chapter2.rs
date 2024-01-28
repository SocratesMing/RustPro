pub fn main() {
    tuple_test();
    for_demo();
}
#[test]
fn tuple_test() {
    println!("-------- tuple demo --------");
    //定义
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //解构元组
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    //访问数组
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} {} {}", five_hundred, six_point_four, one)
}

#[test]
fn for_demo() {
    //打印123
    for number in 1..4 {
        println!("{}!", number);
    }
    //打印321
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
