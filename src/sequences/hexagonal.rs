extern crate num_bigint;
use num_bigint::BigUint;
use num_integer::Integer;

use crate::{progress::Progress, sequences::Sequence};

pub struct Hexagonal;
impl Sequence for Hexagonal {
	fn calculate(limit: u64) {
        let progress = Progress::new(limit);

        //Base case
        println!("{}", 0);
        let two = BigUint::from(2 as u8);
        for i in 1..limit {
            let two_n: BigUint = BigUint::from(i) * &two;
            let upper: BigUint = two_n.clone() * (&two_n - (1 as u8));

            if upper.is_even() {
                println!("{}", upper.div_floor(&two));
            }
        }

		progress.finish();
	}
}