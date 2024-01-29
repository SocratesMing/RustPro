use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::net::IpAddr;
use std::{io, result};

pub fn main() {
    // demo01();
    // demo02();
    // upwarp_expect();
    // propagate_panic();
    // read_username_from_file2();
}

#[test]
fn demo_3() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
}
#[test]
fn demo01() {
    let file = File::open("hello.txt");
    let f = match file {
        Ok(file) => {
            file;
            println!("文件存在");
        }
        Err(errr) => {
            println!("不文件存在");
            panic!("Error opening file [{:?}]", errr);
        }
    };
}

fn demo02() {
    let file = File::open("hello.txt");

    // 匹配不同的错误
    let f = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => {
                    panic!("Error creating file [{:?}]", error);
                }
            },
            other_error => panic!("Error opening file [{:?}]", other_error),
        },
    };
}

fn upwarp_expect() {
    // upwarp不可以指定错误信息
    // let file = File::open("hello.txt").unwrap();
    let file = File::open("hello.txt").expect("文件不存在");
}

fn propagate_panic() {
    let s = read_username_from_file();
    match s {
        Ok(s) => {
            println!("{}", s);
        }
        Err(error) => {
            panic!("error execute funtion")
        }
    }

    let s = read_username_from_file2();
    match s {
        Ok(s) => {
            println!("{}", s);
        }
        Err(error) => {
            panic!("error execute funtion")
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("hello.txt");

    let f = match file {
        Ok(_) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.unwrap().read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    } //此处没有分号，表示结果直接返回
}

fn read_username_from_file2() -> Result<String, io::Error> {
    //  ? 如果OK继续执行，如果Err直接返回Err
    let mut file = File::open("hello.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    //  还可以链式调用
    // File::open("hello.txt")?.read_to_string(&mut  s)?;
    Ok(s)
}
