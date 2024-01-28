use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub fn contact_csv(path: &str, csv_name: &str) -> Result<(), Box<dyn Error>> {
    let csv_file = csv_name.to_owned() + ".csv";
    let csv_files: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |e| e == "csv"))
        .collect();


    let handles: Vec<_> = csv_files
        .into_iter()
        .map(|entry| {
            contact(&entry.path(), &csv_file).expect("TODO: 拼接异常");
        })
        .collect();

    // tx.send(format!("存储到文件 {:?}", dist_csv)).unwrap();
    println!("存储到文件 {:?}", csv_file);
    Ok(())
}

fn contact(src_path: &PathBuf, dist_name: &str) -> Result<(), Box<dyn Error>> {

    let parent_path = src_path.parent().unwrap();
    let dist_csv = parent_path.join(dist_name);

    let file = OpenOptions::new().create(true).append(true).open(dist_csv)?;

    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(src_path)?;
    let mut writer = csv::Writer::from_writer(file);

    for record in reader.records() {
        let row = record?;
        writer.write_record(&[
            row[0].to_string(),
            row[1].to_string(),
            row[2].to_string(),
        ])?;
    }
    println!("已拼接 {:?}", src_path.to_string_lossy().to_string());
    Ok(())
}

#[test]
fn test01() {
    println!("开始构建");
    contact_csv(r"D:\DataTick\EURUSD\dist", "EURUSD").unwrap();
}