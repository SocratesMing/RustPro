pub fn main() {
    demo01();
    demo02();
}

unsafe fn dangerous() {}

fn demo01() {
    #![allow(unused)]
    let mut num = 5;
    // 使用 as 将不可变和可变引用强转为对应的裸指针类型
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。
    // 因此访问的代码块需要用unsafe代码块包围
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous(); // 在unsafe代码块中调用unsafe函数
    }
}

// 定义静态变量
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        // 修改静态变量
        COUNTER += inc;
    }
}

fn demo02() {
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);

    unsafe {
        // 访问静态变量
        println!("COUNTER: {}", COUNTER);
    }
}
