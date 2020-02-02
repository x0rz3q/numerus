extern crate num_bigint;
use num_bigint::BigUint;

use crate::{progress::Progress, sequences::Sequence};

pub struct Hexagonal;
impl Sequence for Hexagonal {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);

		// Base case
		println!("{}", 0);
		let two = BigUint::from(2 as u8);
		for i in 1..limit {
			let n = BigUint::from(i);
			let two_n: BigUint = n.clone() * &two;
			let result: BigUint = n * (&two_n - (1 as u8));

			println!("{}", result);
		}

		progress.finish();
	}
}
