#[derive(Debug)]
pub struct TimeStamp2 {
    ts: u32,
}

impl TimeStamp2 {
    pub fn new(year: u8, month: u8, day: u8, hour: u8) -> Self {
        let year = year as u32 * 100_00_00;
        let month = month as u32 * 100_00;
        let day = day as u32 * 100;
        let hour = hour as u32;

        TimeStamp2 {
            ts: year + month + day + hour,
        }
    }

    pub fn as_ymdh(&self) -> (u8, u8, u8, u8) {
        let rep = self.ts;
        let year = rep / 100_00_00;
        let month = rep / 100_00 - year * 100;
        let day = rep / 100 - (year * 100_00 + month * 100);
        let hour = rep - (year * 100_00_00 + month * 100_00 + day * 100);
        (year as u8, month as u8, day as u8, hour as u8)
    }
}
