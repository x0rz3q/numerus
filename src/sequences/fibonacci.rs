use std::mem::replace;
use gmp::mpz::Mpz;

use crate::{progress::Progress, sequences::Sequence};

pub struct Fibonacci;
impl Sequence for Fibonacci {
	fn calculate(limit: u64) {
		let progress = Progress::new(limit);
		let mut first = Mpz::from_str_radix("0", 10).unwrap(); 
		let mut second = Mpz::from_str_radix("1", 10).unwrap();

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
