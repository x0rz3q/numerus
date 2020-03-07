use gmp::mpz::Mpz;
use std::ops::{Div, Mul, Sub};

use crate::{progress::Progress, sequences::Sequence};

pub struct Pengagonal;
impl Sequence for Pengagonal {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);

		for i in 0..limit {
			let n: Mpz = Mpz::from(i);
			println!("{}", n.pow(2).mul(Mpz::from(3)).sub(n).div(Mpz::from(2)));

			progress.inc();
		}

		progress.finish();
	}
}
