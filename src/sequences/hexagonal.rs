use crate::{progress::Progress, sequences::Sequence};
use gmp::mpz::Mpz;

pub struct Hexagonal;
impl Sequence for Hexagonal {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);

		// Base case
		println!("{}", 0);
		let two = Mpz::from_str_radix("2", 10).unwrap();
		for i in 1..limit {
			let n = Mpz::from(i);
			let two_n: Mpz = n.clone() * two.clone();
			let result: Mpz = n * (two_n - Mpz::one());

			println!("{}", result);
		}

		progress.finish();
	}
}
