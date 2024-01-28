pub fn main() {
    demo_01();
}

fn demo_01() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longeset(string1.as_str(), string2);
    println!("the longest string is {}", result);
}

fn longeset<'a, 'b>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// 结构体内部使用生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法上使用生命周期
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn demo_02() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

use std::fmt::Display;

//  生命周期结合使用泛型
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
