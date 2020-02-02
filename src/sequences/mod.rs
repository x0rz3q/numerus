pub mod fibonacci;
pub mod pentagonal;
pub mod power;
pub mod triangular;
pub mod tribonacci;
pub mod hexagonal;

pub trait Sequence {
	fn calculate(limit: u64);
}
