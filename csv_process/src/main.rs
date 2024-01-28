use std::time::Instant;

use clap::Parser;

use csv_process::convert_tick::contact::contact_csv;
use csv_process::convert_tick::convert::find_csv_threading;

#[warn(unused_variables)]
fn main() {
    let args = Args::parse();
    let start_time = Instant::now();

    if args.mode == 1 {
        find_csv_threading(&args.path, args.utc).unwrap();
    }
    if args.mode == 2 {
        contact_csv(&args.path, &args.symbol).unwrap();
    }

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("{:?}", elapsed_time);
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 1 转换tick  2 拼接csv
    #[arg(short, long)]
    mode: u8,

    /// CSV文件解压后所在的文件目录
    #[arg(short, long)]
    path: String,

    /// utc时区 一般为8
    #[arg(short, long, default_value_t = 8)]
    utc: u8,

    /// 拼接tick数据时生成的文件名，例如EURUSDSP
    #[arg(short, long)]
    symbol: String,
}