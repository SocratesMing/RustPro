fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 定义结构体泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 在为结构体和枚举实现方法时（像第 5 章那样），一样也可以用泛型。
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn struct_type() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
pub(crate) fn main() {
    struct_type();
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    //
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // assert_eq!(result, 100);
    //
    // let char_list = vec![16.3, 15.3, 14.2, 4.3];
    //
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);
    // assert_eq!(result, 'y');
}
