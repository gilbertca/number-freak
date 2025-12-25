use rand::Rng;

struct Die {
    num_faces: usize,
}

impl Roll for Die {
    /// Defines methods for rollable objects
    pub fn roll(&self, num_rolls: usize) -> Vec<usize> {
        /// Rolls this instance of the die `num_rolls` times
        /// and returns a vector of the results
    }
}
