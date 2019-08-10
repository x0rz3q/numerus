extern crate num_bigint;
use num_bigint::BigUint;
use num_traits::pow::Pow;

pub fn calculate(limit: u64) {
	let power: u8 = 2;
	for i in 0..limit {
		println!("{}", BigUint::from(i).pow(power));
	}
}
