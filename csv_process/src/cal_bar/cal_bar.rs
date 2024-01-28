//
//
// fn cal_bar_by_csv() -> Result<()> {
//     // 从CSV文件中读取数据
//     let df = CsvReader::from_path("your_data_file.csv")?
//         .has_header(false)
//         .finish()?;
//
//     // 将时间戳列转换为DateTime类型
//     let df = df.with_column(
//         df.column(0)
//             .cast(DataType::Date64)
//             .unwrap()
//             .alias("timestamp"),
//     );
//
//     // 对数据进行重新采样，计算1分钟、5分钟、15分钟的OHLC
//     let ohlc_1min = resample_ohlc(&df, "1T");
//     let ohlc_5min = resample_ohlc(&df, "5T");
//     let ohlc_15min = resample_ohlc(&df, "15T");
//
//     // 打印结果
//     println!("1-Minute OHLC:\n{:?}", ohlc_1min);
//     println!("5-Minute OHLC:\n{:?}", ohlc_5min);
//     println!("15-Minute OHLC:\n{:?}", ohlc_15min);
//
//     Ok(())
// }
//
// // 辅助函数：重新采样并计算OHLC
// fn resample_ohlc(df: &DataFrame, freq: &str) -> Result<DataFrame> {
//     df.resample(freq, Some(agg_list().agg_first()))?
//         .agg(agg_list().agg_max())
//         .agg(agg_list().agg_min())
//         .agg(agg_list().agg_last())
// }
//
// // 辅助函数：创建聚合函数列表
// fn agg_list() -> AggList {
//     let mut agg_list = AggList::new();
//     agg_list.add("open", |s| Ok(s.first().unwrap().unwrap()));
//     agg_list.add("high", |s| Ok(s.max().unwrap().unwrap()));
//     agg_list.add("low", |s| Ok(s.min().unwrap().unwrap()));
//     agg_list.add("close", |s| Ok(s.last().unwrap().unwrap()));
//     agg_list
// }
