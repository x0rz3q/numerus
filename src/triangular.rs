extern crate num_bigint;
use num_bigint::BigUint;
use std::ops::Div;

#[path = "progress.rs"]
mod progress;

pub fn calculate(limit: u64) {
	let bar = progress::get_bar(limit);

	for n in 0..limit {
		let i = BigUint::from(n);
		let j = BigUint::from(n + 1);
		let result = i * &j;
		println!("{}", result.div(2 as u8));

		bar.inc(1);
	}

	bar.finish();
}
