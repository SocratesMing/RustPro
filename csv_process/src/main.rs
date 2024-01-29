use std::collections::HashMap;
use std::time::Instant;

use clap::{Arg, Command, value_parser};

use csv_process::cal_bar::cal_bar::calculate_bar;
use csv_process::convert_tick::contact::contact_csv;
use csv_process::convert_tick::convert::find_csv_threading;

fn main() {
    let mut map = HashMap::new();
    map.insert("1N", 60000);
    map.insert("5N", 60000 * 5);
    map.insert("15N", 60000 * 15);
    map.insert("30N", 60000 * 30);
    map.insert("1H", 60000 * 1);
    let matches = Command::new("fast-tools")
        .version("1.0")
        .author("sututu")
        .about("行情处理CLI工具")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .required(true)
                .value_parser(["1", "2", "3"])
                .default_value("1")
                .help("1转tick 2拼接tick 3计算bar (1, 2, or 3)"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .required(true)
                .help("文件路径或文件夹路径"),
        ).arg(
        Arg::new("offset")
            .short('o')
            .long("offset")
            .value_parser(value_parser!(u8))
            .default_value("8")
            .help("UTC 偏移量参数(hour) (mode为1有效)"),
    )
        .arg(
            Arg::new("symbol")
                .short('s')
                .long("symbol")
                .required_if_eq("mode", "2")
                .help("合约参数 (mode为2有效)"),
        )
        .arg(
            Arg::new("frame")
                .short('f')
                .long("frame")
                .required_if_eq("mode", "3")
                .value_parser(["1N", "5N", "15N", "30N", "1H"])
                .help("时间框架合约参数 (mode为3有效)"),
        ).get_matches();

    // 获取参数值
    let mode = matches.get_one::<String>("mode").unwrap();
    let path = matches.get_one::<String>("path").unwrap();
    let start_time = Instant::now();
    // 在这里根据参数值执行相应的逻辑
    match mode.as_str() {
        "1" => {
            let offset = matches.get_one::<u8>("offset").unwrap();

            println!("开始执行tick数据格式转换,UTC 偏移量为 {} hour", offset);
            // 在这里执行 Mode 1 的逻辑
            find_csv_threading(&path, *offset);
        }
        "2" => {
            let symbol = matches.get_one::<String>("symbol").unwrap();

            println!("开始执行路径 {} 下的csv文件拼接任务", path);
            // 在这里执行 Mode 2 的逻辑
            contact_csv(&path, symbol);
        }
        "3" => {
            let frame = matches.get_one::<String>("frame").unwrap();

            println!("开始执行BAR数据计算,时间框架为{:?}", frame);
            // 在这里执行 Mode 3 的逻辑
            calculate_bar(&path, *map.get(frame.as_str()).unwrap());
        }
        _ => {
            println!("Invalid mode");
        }
    }
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("程序运行耗时 {:?}", elapsed_time);
}

