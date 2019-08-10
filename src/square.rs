extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::pow::Pow;

#[path = "progress.rs"]
mod progress;
use progress::Progress;

pub fn calculate(limit: u64) {
	let progress = Progress::new(limit);
	let power: u8 = 2;
	for i in 0..limit {
		println!("{}", BigUint::from(i).pow(power));
		progress.inc();
	}

	progress.finish();
}
