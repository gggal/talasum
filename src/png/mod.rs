// Magi Pseudo-random number generator

// A class for which each object accepts
// a number in its constructor and uses it to
// create the internal generator object

use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg64Xsh32, Pcg32};
pub mod prandomizer;
pub mod skewed_prandomizer;

// pub struct PRandStream {
//     generator: Pcg32,
// }

// impl PRandStream {
//     pub fn new(seed: u64) -> Self {
//         Self {
//             generator: Pcg32::seed_from_u64(seed),
//         }
//     }

//     pub fn get(&mut self) -> u32 {
//         self.generator.gen()
//     }
// }

// // impl Stream for PRandStream {
// //     type Item = u32;

// //     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
// //         Poll::Ready(Some(self.get()))
// //     }
// // }

// impl Iterator for PRandStream {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.get())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::PRandStream;

//     #[test]
//     fn pransom_generator_is_deterministic() {
//         let mut gen1 : PRandStream = PRandStream::new(123);
//         let mut gen2 : PRandStream = PRandStream::new(123);
//         assert_eq!(gen1.get(), gen2.get());
//     }

//     #[test]
//     fn pransom_generator_is_seedable() {
//         let mut gen1 : PRandStream = PRandStream::new(123);
//         let mut gen2 : PRandStream = PRandStream::new(124);
//         assert_ne!(gen1.get(), gen2.get());
//     }

//     #[test]
//     fn zero_is_valid_seed_for_pransom_generator() {
//         let mut gen : PRandStream = PRandStream::new(0);
//         assert_ne!(gen.get(), 0);
//     }

// }
