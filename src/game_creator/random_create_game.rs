//! Create a game using a random number generator.

use rand::{Rng, thread_rng, SeedableRng};
use rand::rngs::ThreadRng;
use rand_pcg::Pcg64;
use crate::game_creator::GameCreator;
use crate::GameBoard;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Contains a random generator to create a board
pub struct RandomLoader<R : Rng> {
    rng: R,
}

impl RandomLoader<ThreadRng> {
    /// Creates a new random generator to create a board
    pub fn new() -> Self {
        RandomLoader {
            rng: thread_rng()
        }
    }
}

impl RandomLoader<Pcg64> {

    /// use a preset seed for the rng
    pub fn from_seed(seed: u64) -> Self {
        RandomLoader { rng: Pcg64::seed_from_u64(seed) }
    }


}

#[derive(Debug)]
pub struct RandomCreatorError;

impl Display for RandomCreatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RandomCreatorError { }

impl<R : Rng> GameCreator for RandomLoader<R> {
    type Error = RandomCreatorError;

    fn into_game(self) -> Result<GameBoard, Self::Error> {
        todo!()
    }
}