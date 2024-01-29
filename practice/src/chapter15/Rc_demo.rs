use crate::chapter15::Rc_demo::List::{Cons, Nil};

pub fn main() {
    demo01();
}

use std::rc::Rc;

//不在prelude中
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn demo01() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); //初始引用计数为1 a=1
    let b = Cons(2, Rc::clone(&a)); //增加a的引用计数
    println!("count after creating b = {}", Rc::strong_count(&a)); //调用 clone 计数会增加1  b = 2
    {
        let c = Cons(3, Rc::clone(&a)); //增加a的引用计数
        println!("count after creating b = {}", Rc::strong_count(&a)); // b = 3
                                                                       // 回收了
    }
    println!("count after creating b = {}", Rc::strong_count(&a)); // b = 2
}
