use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    // demo_01();
    // demo_02();
    // demo_03();
    // demo_04();
    demo_05();
}

fn demo_01() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1000));
    }
    handle.join().unwrap(); // join 会阻塞当前线程直到 handle 所代表的线程结束。
}

fn demo_02() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn demo_03() {
    // mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。
    // 由于历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("string has exist {}", val);  //已经被借用了
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn demo_04() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); //复制一个生产者继续发送

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn demo_05() {
    // 创建一个 Mutex<T>
    let m = Mutex::new(5);

    {
        // 通过lock 方法获取所锁
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // 多线程下所有权会被移动到线程中
    // 智能指针 Rc<T> 来创建引用计数的值，以便拥有多所有者，但是Rc<T> 并不能安全的在线程间共享
    // Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型，atomic 原子并发

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
