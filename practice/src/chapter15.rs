mod Rc_demo;
mod pointer_Box;

pub fn main() {
    // pointer_Box::main();
    Rc_demo::main();
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[test]
fn test_01() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
struct Data {
    value: i32,
}

#[test]
fn test_02() {
    // 创建一个包含数据的 Rc 智能指针
    let shared_data = Rc::new(Data { value: 42 });
    println!("{}", shared_data.value);
    // 克隆 Rc 智能指针，增加引用计数
    println!(
        "Reference Count after cloning: {}",
        Rc::strong_count(&shared_data)
    );
    let rc1 = Rc::clone(&shared_data);
    println!(
        "Reference Count after cloning: {}",
        Rc::strong_count(&shared_data)
    );
    let rc2 = Rc::clone(&shared_data);

    // 打印引用计数
    println!(
        "Reference Count after cloning: {}",
        Rc::strong_count(&shared_data)
    );

    // 通过 Rc 智能指针访问数据
    println!("Value from rc1: {:?}", rc1);
    println!("Value from rc2: {:?}", rc2);

    // 当 rc1 和 rc2 超出作用域时，引用计数减少
    // 当引用计数为零时，数据将被释放
}

#[test]
fn test_03() {
    let person = Human;
    Pilot::fly(&person); //指定调用Pilot的 fly 方法
    Wizard::fly(&person); //指定调用Wizard的 fly 方法
    person.fly(); //直接调用Human的fly方法

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
}

#[test]
fn test04() {
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

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
