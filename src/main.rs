fn main() {
   let today = TimeStamp::new(17, 03, 24, 19);
   let yesterday = TimeStamp::new(17, 03, 23, 18);
   let ly = TimeStamp::new(16, 03, 23, 18);
   println!("{:?}", today);
   println!("{}", today);
   println!("{:?}", yesterday);
   println!("{}", yesterday);
   println!("{:?}", ly);
   println!("{}", ly);
}

mod timestamp;
use timestamp::*;

use chrono::{NaiveDateTime, Local, Duration, Datelike};
struct Cache<T> {
    cached_data: Vec<CahceItem<T>>,
}

struct CahceItem<T> {
    item: T,
    timestamp: u64,
}

impl<T> CahceItem<T> {
    // fn is_valid(&self) -> bool {
    //     let now = Local::now().naive_local();
    //     if self.timestamp > now - Duration::days(1) {
    //         return true;
    //     }

    //     if self.
    //     unimplemented!()
    // }
}