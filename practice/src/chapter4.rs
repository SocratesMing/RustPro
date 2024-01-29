pub(crate) fn main() {
    // move_demo();
    // slice_demo();
    slice_demo2();
}
#[test]
fn move_demo() {
    let x = 5;
    let y = x;

    println!("x:{} y{}", x, y)
}
#[test]
fn slice_demo() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    println!("{} {} {}", hello, world, whole);
}
#[test]
fn slice_demo2() {
    let my_string = String::from("hello world");

    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值**就是**字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", &s[..i]);
            return &s[..i];
        }
    }
    println!("{}", &s[..]);
    &s[..]
}
#[test]
fn slice_demo5() {
    let a = [1, 2, 3, 4, 5];
    let b = &a[..3];
    println!("{:?}", b);
}
