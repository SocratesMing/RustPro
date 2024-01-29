use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use csv::Writer;

// 定义元组结构体表示OHLC数据

#[derive(Clone, Copy)]
struct OhlcRecord(i64, i64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);

impl OhlcRecord {
    // 创建一个新的OHLC记录
    fn new(time: i64, duration: i64, bid: f64, ask: f64) -> Self {
        let mid = (bid + ask) / 2.0;
        let start_time = time / duration * duration;
        OhlcRecord(start_time, start_time + duration, bid, bid, bid, bid, ask, ask, ask, ask, mid, mid, mid, mid)
    }

    // 更新OHLC记录
    fn update(&mut self, bid: f64, ask: f64) {
        // 更新现有OHLC记录
        self.3 = self.3.max(bid);
        self.4 = self.4.min(bid);
        self.5 = bid;
        self.7 = self.7.max(ask);
        self.8 = self.8.min(ask);
        self.9 = ask;
        let mid = (ask + bid) / 2.0;
        self.11 = self.11.max(mid);
        self.12 = self.12.min(mid);
        self.13 = mid;
    }
}


fn cal_bar2(path: &str, frame: i64) -> Result<(), Box<dyn Error>> {
    let dist_csv = format!("{}/{}N.csv", Path::new(path).parent().unwrap().to_str().unwrap(), frame / 60000);
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(path)?;
    let mut writer = csv::WriterBuilder::new().has_headers(false).from_path(dist_csv)?;
    let first_row = reader.records().next().unwrap()?;
    let tick_time = first_row[0].parse()?;

    let mut ohlc_record: OhlcRecord = {
        let tick_time: i64 = tick_time;
        let first_bid: f64 = first_row[1].parse()?;
        let first_ask: f64 = first_row[2].parse()?;
        OhlcRecord::new(tick_time, frame, first_bid, first_ask)
    };
    for row in reader.records() {
        let row = row?;
        let time_stamp: i64 = row[0].parse()?;
        let bid: f64 = row[1].parse()?;
        let ask: f64 = row[2].parse()?;
        if time_stamp >= ohlc_record.0 && time_stamp < ohlc_record.1 {
            ohlc_record.update(bid, ask);
        } else {
            write_row(&mut writer, ohlc_record).unwrap();
            ohlc_record = OhlcRecord::new(time_stamp, frame, bid, ask);
        }
    }
    // 写入最后一条OHLC记录
    write_row(&mut writer, ohlc_record)?;

    println!("执行完成");
    Ok(())
}

fn write_row(writer: &mut Writer<File>, ohlc_record: OhlcRecord) -> Result<(), Box<dyn Error>> {
    writer.write_record(vec![
        format!("{:.0}", ohlc_record.0),
        format!("{:.0}", ohlc_record.1),
        format!("{:.5}", ohlc_record.2),
        format!("{:.5}", ohlc_record.3),
        format!("{:.5}", ohlc_record.4),
        format!("{:.5}", ohlc_record.5),
        format!("{:.5}", ohlc_record.6),
        format!("{:.5}", ohlc_record.7),
        format!("{:.5}", ohlc_record.8),
        format!("{:.5}", ohlc_record.9),
        format!("{:.5}", ohlc_record.10),
        format!("{:.5}", ohlc_record.11),
        format!("{:.5}", ohlc_record.12),
        format!("{:.5}", ohlc_record.13),
    ].into_iter())?;
    Ok(())
}


#[test]
fn test01() {
    let path = r"D:\tick\EURUSD\dist\200005.csv";
    cal_bar2(&path, 60000).unwrap();
}

