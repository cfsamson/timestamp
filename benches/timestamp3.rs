#[derive(Debug)]
pub struct TimeStamp3 {
    ts: [u8; 4],
}

impl TimeStamp3 {
    pub fn new(year: u8, month: u8, day: u8, hour: u8) -> Self {
        

        TimeStamp3 {
            ts: [year, month, day, hour]
        }
    }

    pub fn as_ymdh(&self) -> (u8, u8, u8, u8) {
        (self.ts[0], self.ts[1], self.ts[2], self.ts[3])
    }
}
