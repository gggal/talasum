// use rand::{thread_rng, Rng};
// use bitvec::{Bits, store::BitStore};

pub mod boolean;
pub mod null;
pub mod number;
pub mod string;
use crate::randomization;

pub const IDENTITY: fn(u32, String) -> String = |_, input| input;
