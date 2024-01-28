pub(crate) fn main() {
    println!("---------- 块demo---------");
    block_demo();
    if_demo();
}

fn if_demo() {
    /// if语句中不需要括号
    ///
    ///
    ///
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }

    println!("x : {x}");
    let dish = ("Ham", "Eggs");

    // this body will be skipped because the pattern is refuted
    if let ("Bacon", b) = dish {
        println!("Bacon is served with {}", b);
    } else {
        // This block is evaluated instead.
        println!("No bacon will be served");
    }

    // this body will execute
    if let ("Ham", b) = dish {
        println!("Ham is served with {}", b);
    }

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }

    let x = Some(3);
    let a = if let Some(1) = x {
        1
    } else if x == Some(2) {
        2
    } else if let Some(y) = x {
        y
    } else {
        -1
    };
    assert_eq!(a, 3);
    println!("a: {}", a);
}

fn block_demo() {
    // 块的类型和值是最后一个表达式的值及对应的类型
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = { 3 + 4 };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}
