pub(crate) fn main() {
    struct_demo();
    method_demo();
}
// 定义结构体
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct User2<'a> {
    active: bool,
    username: &'a str,
    email: String,
    sign_in_count: u32,
}

#[test]
fn struct_demo2() {
    let user2 = User2 {
        active: true,
        username: "suzhengm",
        email: String::from("test@email.com"),
        sign_in_count: 22,
    };
    println!("user2 {:?}", user2)
}
#[test]
fn struct_demo() {
    // 实例化结构体
    let mut user1 = User {
        // String类型没有实现copy
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 修改结构体的属性
    user1.email = String::from("anotheremail@example.com");
    println!("user1: {:#?}", user1);
    // 同名化参数用法
    let email = String::from("samename@example.com");
    let username = String::from("samename@");
    let user2 = build_user(email, username);
    println!("user2: {:#?}", user2);

    let user3 = User {
        //user1中的email和name发生了移动，因此user3不再有效
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user3: {:#?}", user3);

    let user4 = User {
        email: String::from("another@example.com"),
        ..user3 //user3中的email和name发生了移动，因此user3不再有效
    };

    println!("user4: {:#?}", user4);
}

// 字段初始化写法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn print(&self) {
        println!("{:?}", self);
    }
}
#[test]
fn method_demo() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    rect1.print();
}
