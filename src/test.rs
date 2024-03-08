#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn bench_workload(b: &mut Bencher) {
    b.iter(|| workload());
}
