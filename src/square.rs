extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::pow::Pow;

#[path = "progress.rs"]
mod progress;

pub fn calculate(limit: u64) {
	let bar = progress::get_bar(limit);

	let power: u8 = 2;
	for i in 0..limit {
		println!("{}", BigUint::from(i).pow(power));
		bar.inc(1);
	}

	bar.finish();
}
