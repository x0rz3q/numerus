extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::pow::Pow;
use std::ops::{Div, Mul, Sub};

#[path = "../progress.rs"]
mod progress;
use progress::Progress;

pub fn calculate(limit: u64) {
	let progress = Progress::new(limit);

	for i in 0..limit {
		let n: BigUint = BigUint::from(i);
		println!("{}", n.pow(2 as u8).mul(3 as u8).sub(n).div(2 as u8));

		progress.inc();
	}

	progress.finish();
}
