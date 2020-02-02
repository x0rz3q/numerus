extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

use crate::{progress::Progress, sequences::Sequence};

pub struct Fibonacci;
impl Sequence for Fibonacci {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);
		let mut first: BigUint = Zero::zero();
		let mut second: BigUint = One::one();

		println!("{}", first);
		println!("{}", second);

		for _ in 2..limit {
			let third = first + &second;
			println!("{}", third);
			first = replace(&mut second, third);

			progress.inc();
		}

		progress.finish();
	}
}
