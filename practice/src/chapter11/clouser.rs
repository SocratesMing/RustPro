#![allow(unused)]
fn main() {}

fn demo01() {
    use std::thread;
    use std::time::Duration;

    fn generate_workout(intensity: u32, random_number: u32) {
        // 注意这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，
        // 不是调用匿名函数的 返回值。
        // 现在函数只在一个地方被调用，并只会在需要结果的时候执行该代码。
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        if intensity < 25 {
            println!("Today, do {:?} pushups!", expensive_closure(intensity));
            println!("Next, do {:?} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {:?} minutes!", expensive_closure(intensity));
            }
        }
    }
}

fn demo_02() {}
// 闭包有一个 u32 的参数并返回一个 u32，
// 这样所指定的 trait bound 就是 Fn(u32) -> u32。
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, //这个field实现了闭包，因为它是类型T
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
