extern crate chrono;
use chrono::*;

pub fn after(start : DateTime<UTC>) -> DateTime<UTC> {
    start + Duration::seconds(1e9 as i64)
}
