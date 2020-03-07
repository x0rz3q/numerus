use gmp::mpz::Mpz;
use crate::{progress::Progress, sequences::Sequence};

fn calculate_power(limit: u64, power: u32) {
	let progress = Progress::new(limit);
	for i in 0..limit {
		println!("{}", Mpz::from(i).pow(power));
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
