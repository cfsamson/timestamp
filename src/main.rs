mod timestamp;
use timestamp::*;
fn main() {
    let c: CahceItem<i32> = CahceItem {
        item: 4,
        timestamp: TimeStamp::new(19, 3, 23, 22),
    };

    println!("{}", c.timestamp);
    println!("{}", c.is_valid());

}

#[derive(Debug)]
struct Cache<T> {
    cached_data: Vec<CahceItem<T>>,
}

#[derive(Debug)]
struct CahceItem<T> {
    item: T,
    timestamp: TimeStamp,
}

impl<T> CahceItem<T> {
    fn is_valid(&self) -> bool {
        if self.timestamp > TimeStamp::new(19, 3, 24, 22) {
            true
        } else {
            false
        }
    }
}