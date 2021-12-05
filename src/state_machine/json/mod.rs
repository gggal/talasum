// use rand::{thread_rng, Rng};
// use bitvec::{Bits, store::BitStore};

pub mod boolean;
pub mod null;
pub mod number;
pub mod string;

pub const IDENTITY: fn(String) -> String = |input| input;
