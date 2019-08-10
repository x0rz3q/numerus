extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

pub fn calculate(limit: u64) {
	let mut first: BigUint = Zero::zero();
	let mut second: BigUint = One::one();
	let mut third: BigUint = One::one();

	println!("{}", first);
	println!("{}", second);
	println!("{}", third);

	for _ in 3..limit {
		let fourth = first + &second + &third;
		println!("{}", fourth);
		first = replace(&mut second, third);
		third = fourth;
	}
}