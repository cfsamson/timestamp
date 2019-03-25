use std::fmt;
/// Stores a representation of the current day, month, year, hour as a u32.
/// 
/// This is achieved by packing the u32 with 4 u8 by rewriting the bits. 
/// It's easy to seralize and deserialize since its only one u32.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeStamp {
    ts: u32,
}

impl TimeStamp {
    pub fn new(y: u8, m: u8, d: u8, h: u8) -> TimeStamp {
        let mut ts = (y as u32) << 24;
        ts |= (m as u32) << 16;
        ts |= (d as u32) << 8;
        ts |= h as u32;

        TimeStamp { ts }
    }

    pub fn as_ymdh(&self) -> (u8, u8, u8, u8) {
        let year: u8 = ((self.ts & 0xFF_00_00_00) >> 24) as u8;
        let month: u8 = ((self.ts & 0x00_FF_00_00) >> 16) as u8;
        let day: u8 = ((self.ts & 0x00_00_FF_00) >> 8) as u8;
        let hour: u8 = self.ts as u8;
        (year, month, day, hour)
    }
}


impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y,m,d,h) = self.as_ymdh();
        write!(f, "year: {}, month: {}, day: {}, hour: {}", y,m,d,h)
    }
}