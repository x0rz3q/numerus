use gmp::mpz::Mpz;
use std::mem::replace;

use crate::{progress::Progress, sequences::Sequence};

pub struct Fibonacci;
impl Sequence for Fibonacci {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);
		let mut first = Mpz::zero();
		let mut second = Mpz::one();

		println!("{}", first);
		println!("{}", second);

		for _ in 2..limit {
			let third = first + second.clone();
			println!("{}", third);
			first = replace(&mut second, third);

			progress.inc();
		}

		progress.finish();
	}
}
