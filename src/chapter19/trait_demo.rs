#![allow(unused)]
pub fn main() {
    demo02();
    demo03();
}

pub trait Iterator {
    // 关联类型（associated types）
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// 默认类型参数（default type parameters
// trait Add<RHS=Self> {
//     type Output;
//     // 如果实现 Add trait 时不指定 RHS 的具体类型，
//     // RHS 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。
//     fn add(self, rhs: RHS) -> Self::Output;
// }

use std::ops::Add;

struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn demo02() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn demo03() {
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
