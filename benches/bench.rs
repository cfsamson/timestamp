#![feature(test)]
extern crate test;
use test::Bencher;
use timestamp::TimeStamp;
mod chrono_ts;
mod timestamp2;
use chrono_ts::TimeStampChrono;
use timestamp2::TimeStamp2;
#[bench]
fn naive_timestamp(b: &mut Bencher) {
    b.iter(|| {
        let mut timestamps = vec![];
        let start = TimeStamp2::new(19, 1, 1, 1);
        for _ in 0..20 {
            for i in 0..30 {
                let (y, m, d, h) = start.as_ymdh();
                let ts = TimeStamp2::new(y, m, d + i, h);
                timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn custom_timestamp(b: &mut Bencher) {
    b.iter(|| {
        let mut timestamps = vec![];
        let start = TimeStamp::new(19, 1, 1, 1);
        for _ in 0..20 {
            for i in 0..30 {
                let (y, m, d, h) = start.as_ymdh();
                let ts = TimeStamp::new(y, m, d + i, h);
                timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn chrono_timestamp(b: &mut Bencher) {
    b.iter(|| {
        let mut timestamps = vec![];
        let start = TimeStampChrono::new(19, 1, 1, 1);
        for _ in 0..20 {
            for i in 0..30 {
                let (y, m, d, h) = start.as_ymdh();
                let ts = TimeStampChrono::new(y, m, d + i, h);
                timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn naive_timestamp_create(b: &mut Bencher) {
    
    b.iter(|| {
        let mut timestamps = vec![];
        for _ in 0..20 {
            for i in 0..30 {
                let ts = TimeStamp2::new(19, 1, 1 + i, 1);
                &timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn custom_timestamp_create(b: &mut Bencher) {
    
    b.iter(|| {
        let mut timestamps = vec![];
        for _ in 0..20 {
            for i in 0..30 {
                let ts = TimeStamp::new(19, 1, 1 + i, 1);
                &timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn chrono_timestamp_create(b: &mut Bencher) {
    
    b.iter(|| {
        let mut timestamps = vec![];
        for _ in 0..20 {
            for i in 0..30 {
                let ts = TimeStampChrono::new(19, 1, 1 + i, 1);
                &timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn naive_timestamp_read(b: &mut Bencher) {
    
    b.iter(|| {
        let tsr: Vec<TimeStamp2> = (0..30).map(|i| TimeStamp2::new(19, 1, 1 + i, 1)).collect();
        let mut timestamps = vec![];
        for _ in 0..20 {
            for ts in &tsr {
                &timestamps.push(ts.as_ymdh());
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn custom_timestamp_read(b: &mut Bencher) {
    
    b.iter(|| {
        let tsr: Vec<TimeStamp> = (0..30).map(|i| TimeStamp::new(19, 1, 1 + i, 1)).collect();
        let mut timestamps = vec![];
        for _ in 0..20 {
            for ts in &tsr {
                &timestamps.push(ts.as_ymdh());
            }
        }
        test::black_box(timestamps);
    })
}

#[bench]
fn chrono_timestamp_read(b: &mut Bencher) {
    
    b.iter(|| {
        let tsr: Vec<TimeStampChrono> = (0..30).map(|i| TimeStampChrono::new(19, 1, 1 + i, 1)).collect();
        let mut timestamps = vec![];
        for _ in 0..20 {
            for ts in &tsr {
                &timestamps.push(ts.as_ymdh());
            }
        }
        test::black_box(timestamps);
    })
}