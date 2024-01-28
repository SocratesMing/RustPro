use std::error::Error;
use chrono::{DateTime, FixedOffset, naive, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Asia::Shanghai;

pub fn datetime_to_timestamp(datetime: &str, offset: i64) -> Result<i64, Box<dyn Error>> {
    let utc_datetime = NaiveDateTime::parse_from_str(datetime, "%Y%m%d %H%M%S%f").unwrap();
    let timestamp_beijing = utc_datetime.timestamp_millis() - offset * 3600000; //价格时区默认应该是u
    Ok(timestamp_beijing)
}

pub fn timestamp_to_datetime(timestamp: i64) -> Result<String, Box<dyn Error>> {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).expect("错误");
    let format = Shanghai.from_utc_datetime(&datetime).format("%Y%m%d %H%M%S%3f").to_string();
    Ok(format)
}

#[cfg(test)]
mod test {
    use crate::utils::time_utils::datetime_to_timestamp;

    #[test]
    fn test_01() {
        let offset: i64 = 8;
        let i = datetime_to_timestamp("20090315 170014000", 8 as i64).unwrap();
        assert_eq!(i, 1237107614000);
    }
}