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
        let base: u32 = 0x0000_0000;
        let mut ts = base | (u32::from(y) << 24);
        ts |= u32::from(m) << 16;
        ts |= u32::from(d) << 8;
        ts |= u32::from(h);

        TimeStamp { ts }
    }

    pub fn as_ymdh(&self) -> (u8, u8, u8, u8) {
        let year: u8 = ((self.ts & 0xFF00_0000) >> 24) as u8;
        let month: u8 = ((self.ts & 0x00FF_0000) >> 16) as u8;
        let day: u8 = ((self.ts & 0x0000_FF00) >> 8) as u8;
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