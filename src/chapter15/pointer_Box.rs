use crate::chapter15::pointer_Box::List::{Cons, Nil};
use std::ops::Deref;

pub(crate) fn main() {
    demo01();
    demo02();
    demo03();
    demo04();
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn demo04() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn demo01() {
    let b = Box::new(5);
    println!("box is {}", b);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为box实现deref  trait 并重写deref方法
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn demo03() {
    let x = 5;
    let y = &5;
    let y2 = MyBox::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y2);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn demo02() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List is {:?}", list);
}
