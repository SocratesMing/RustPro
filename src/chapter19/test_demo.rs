pub fn main() {}

#[test]

fn test01() {
    let mut v = vec![1, 2, 3, 5];
}
#[test]
fn test02() {
    let x = 4;
    let x1 = 8;
    let mut x2 = 8;
    let mut y = &x;
    println!("{:p} {}", y, y);

    let z = &mut y;
    let z = &mut x2;
    let y = &x1;
    println!("{:p} {}", y, y);
    println!("{:p} {}", y, *y);
}
