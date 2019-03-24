#[derive(Debug)]
pub struct TimeStamp {
    ts: u32,
}

impl TimeStamp {
    pub fn new(y: u8, m: u8, d: u8, h: u8) -> TimeStamp {
        let mut base: u32 = 0x00000000;
        let mut ts = base | ((y as u32) << 24);
        ts = ts | ((m as u32) << 16);
        ts = ts | ((d as u32) << 8);
        ts = ts | ((h as u32) << 0);

        TimeStamp { ts }
    }

    pub fn as_ymdh(&self) -> (u8, u8, u8, u8) {
        let year: u8 = ((self.ts & 0xFF000000) >> 24) as u8;
        let month: u8 = ((self.ts & 0x00FF0000) >> 16) as u8;
        let day: u8 = ((self.ts & 0x0000FF00) >> 8) as u8;
        let hour: u8 = self.ts as u8;
        (year, month, day, hour)
    }
}

use std::fmt;
impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y,m,d,h) = self.as_ymdh();
        write!(f, "year: {}, month: {}, day: {}, hour: {}", y,m,d,h)
    }
}