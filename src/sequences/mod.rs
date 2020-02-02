pub mod fibonacci;
pub mod hexagonal;
pub mod pentagonal;
pub mod power;
pub mod triangular;
pub mod tribonacci;

pub trait Sequence {
	fn calculate(limit: u64);
}
