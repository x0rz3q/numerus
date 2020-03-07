use gmp::mpz::Mpz;
use std::mem::replace;

use crate::{progress::Progress, Sequence};

pub struct Tribonacci;
impl Sequence for Tribonacci {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);
		let mut first = Mpz::zero();
		let mut second = Mpz::one();
		let mut third = Mpz::one();

		println!("{}", first);
		println!("{}", second);
		println!("{}", third);

		for _ in 3..limit {
			let fourth = first + second.clone() + third.clone();
			println!("{}", fourth);

			first = replace(&mut second, third);
			third = fourth;

			progress.inc();
		}

		progress.finish();
	}
}
