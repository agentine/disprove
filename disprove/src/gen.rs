use rand::rngs::SmallRng;
use rand::SeedableRng;

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

    /// Returns the size parameter.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a mutable reference to the internal RNG.
    pub fn rng(&mut self) -> &mut SmallRng {
        &mut self.rng
    }
}
