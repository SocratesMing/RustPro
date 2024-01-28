#![allow(unused)]

pub(crate) fn main() {
    trait_demo();
}

use std::fmt::{Debug, Display};

// 一般的写法
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound的写法
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 通过 + 指定多个trait bound
pub fn notify3<T: Summary + Display>(item: T) {}

// 通过where简化  多个trait bound + 起来的情况
fn notify4<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    20
}

// 返回实现了 trait 的类型
// 只适用于返回单一类型！！！
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

#[test]
fn trait_demo() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // println!("1 new tweet: {}", tweet.summarize2());
}

// 定义trait
pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize2(&self) -> String {
        // 默认实现
        String::from("(Read more...)")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//  为 NewsArticle 实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        println!("{:?}", self);
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//  为 Tweet 实现 trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        println!("{:?}", self);
        format!("{}: {}", self.username, self.content)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn str_test(s: &[String]) {}
#[test]
fn largest_test() {
    let number_list = vec!["34".to_string(), "50".to_string(), "25".to_string()];
    let pp = &number_list;
    let string1 = String::from("abcd");
    string1.as_str();
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
