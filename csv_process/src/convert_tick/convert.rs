use crate::utils::time_utils::datetime_to_timestamp;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::{fs, io, thread};

pub fn find_csv_threading(path: &str, offset: u8) -> Result<(), io::Error> {
    let path = path;

    let dist_path = Path::new(path).join("dist");

    if !dist_path.exists() {
        println!("创建目录 {:?}", dist_path);
        fs::create_dir_all(&dist_path)?;
    }
    // 创建channel
    // let (tx, rx) = mpsc::channel();

    let csv_files: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |e| e == "csv"))
        .collect();

    let handles: Vec<_> = csv_files
        .into_iter()
        .map(|entry| {
            // let tx = tx.clone();
            println!("entry = {:?}", &entry.path());
            println!("dist_path = {:?}", &dist_path);
            let dist_path = dist_path.clone();
            thread::spawn(move || {
                if let Err(e) = process_csv_thead(&entry.path(), &dist_path, offset) {
                    println!("Error processing file: {}", e);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    // drop(tx); // Close the channel to signal the receiver to stop
    //
    // let results: Vec<_> = rx.iter().collect();
    // for result in results {
    //     println!("{}", result);
    // }

    Ok(())
}

fn process_csv_thead(src_path: &PathBuf, dist_path: &PathBuf, offset: u8) -> Result<(), Box<dyn Error>> {
    let filename = src_path.file_name().ok_or(io::Error::new(io::ErrorKind::Other, "Invalid filename"))?;
    let binding = filename.to_string_lossy();
    let dist_name = binding.split("_").last().unwrap_or_default();
    let dist_csv = dist_path.join(dist_name);

    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(src_path)?;
    let mut writer = csv::Writer::from_path(&dist_csv)?;

    for record in reader.records() {
        let row = record?;
        let timestamp = datetime_to_timestamp(&row[0], offset as i64)?;
        writer.write_record(&[
            timestamp.to_string(),
            row[1].to_string(),
            row[2].to_string(),
        ])?;
    }

    // tx.send(format!("存储到文件 {:?}", dist_csv)).unwrap();
    println!("存储到文件 {:?}", dist_csv);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::convert_tick::convert::find_csv_threading;
    use std::time::Instant;

    #[test]
    fn test_01() {
        let start_time = Instant::now();
        find_csv_threading(r"D:\DataTick\USDJPY", 8).unwrap();
        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);
        println!("{:?}", elapsed_time);
    }
}
