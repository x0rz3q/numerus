use gmp::mpz::Mpz;
use std::ops::Div;

use crate::progress::Progress;

pub fn calculate(limit: u64) {
	let progress = Progress::new(limit);

	for n in 0..limit {
		let i = Mpz::from(n);
		let j = Mpz::from(n + 1);
		let result = i * j;
		println!("{}", result.div(Mpz::from(2)));

		progress.inc();
	}

	progress.finish();
}
