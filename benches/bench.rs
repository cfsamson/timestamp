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
