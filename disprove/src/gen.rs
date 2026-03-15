use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Random value generator with a size parameter.
///
/// The size parameter controls the complexity of generated values
/// (e.g., maximum length of vectors, magnitude of integers).
pub struct Gen {
    rng: SmallRng,
    size: usize,
}

impl Gen {
    /// Create a new generator with the given size parameter.
    pub fn new(size: usize) -> Self {
        Gen {
            rng: SmallRng::from_rng(rand::thread_rng()).unwrap(),
            size,
        }
    }

    /// Create a new generator from a specific seed for deterministic replay.
    pub fn from_seed(size: usize, seed: u64) -> Self {
        Gen {
            rng: SmallRng::seed_from_u64(seed),
            size,
        }
    }

    /// Returns the size parameter.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a mutable reference to the internal RNG.
    pub fn rng(&mut self) -> &mut SmallRng {
        &mut self.rng
    }

    /// Choose a random element from a non-empty slice.
    ///
    /// # Panics
    ///
    /// Panics if the slice is empty.
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        assert!(!slice.is_empty(), "Gen::choose called on empty slice");
        let idx = self.rng.gen_range(0..slice.len());
        &slice[idx]
    }

    /// Generate a random value in the given range.
    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        R: rand::distributions::uniform::SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    /// Generate a random boolean.
    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }
}
