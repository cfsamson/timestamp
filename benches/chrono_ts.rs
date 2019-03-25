use chrono::{NaiveDateTime, Datelike, Timelike, NaiveDate, NaiveTime};

#[derive(Debug)]
pub struct TimeStampChrono {
    ts: NaiveDateTime,
}

impl TimeStampChrono {
    pub fn new(year: u8, month: u8, day: u8, hour: u8) -> Self {
        TimeStampChrono {
            ts: NaiveDateTime::new(
                NaiveDate::from_ymd(year as i32, month as u32, day as u32), 
                NaiveTime::from_hms(hour as u32, 0, 0)
                ),
        }
    }

    pub fn as_ymdh(&self) -> (u8, u8, u8, u8) {
        
        (self.ts.year() as u8, self.ts.month() as u8, self.ts.day() as u8, self.ts.hour() as u8)
    }
}