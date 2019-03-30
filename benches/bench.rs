#![feature(test)]
extern crate test;
use test::Bencher;
use timestamp::TimeStamp;
mod chrono_ts;
mod timestamp2;
mod timestamp3;
use chrono_ts::TimeStampChrono;
use timestamp2::TimeStamp2;
use timestamp3::TimeStamp3;

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

#[bench]
fn naive_timestamp3_read(b: &mut Bencher) {
    
    b.iter(|| {
        let tsr: Vec<TimeStamp3> = (0..30).map(|i| TimeStamp3::new(19, 1, 1 + i, 1)).collect();
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
fn naive_timestamp3_create(b: &mut Bencher) {
    
    b.iter(|| {
        let mut timestamps = vec![];
        for _ in 0..20 {
            for i in 0..30 {
                let ts = TimeStamp3::new(19, 1, 1 + i, 1);
                &timestamps.push(ts);
            }
        }
        test::black_box(timestamps);
    })
}