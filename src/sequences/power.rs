extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::pow::Pow;

use crate::{progress::Progress, sequences::Sequence};

fn calculate_power(limit: u64, power: u64) {
	let progress = Progress::new(limit);
	for i in 0..limit {
		println!("{}", BigUint::from(i).pow(power));
		progress.inc();
	}

	progress.finish();
}

pub struct Square;
impl Sequence for Square {
	fn calculate(limit: u64) {
		calculate_power(limit, 2);
	}
}

pub struct Cube;
impl Sequence for Cube {
	fn calculate(limit: u64) {
		calculate_power(limit, 3);
	}
}
