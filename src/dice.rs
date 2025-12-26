use rand::Rng;

pub struct Die<R> {
    rng: R,
    num_faces: usize,
    face_values: Vec<Option<usize>>,
}

impl<R: Rng> Die<R> {
    /// Creates a new instance of a `Die`.
    /// The RNG **MUST** be seeded before initialization.
    pub fn new(num_faces: usize, seeded_rng: R) -> Die<R> {
        Die { rng: seeded_rng,
            num_faces: num_faces,
            face_values: (1..=num_faces).map(Some).collect()
        }
    }

    /// Rolls this instance of the die `num_rolls` times
    /// and returns a vector of the results.
    pub fn roll(&mut self, num_rolls: usize) -> Vec<usize> {
        let mut results: Vec<usize> = vec![0;num_rolls];
        for n in 0..num_rolls {
            let random_value: usize = self.rng.random_range(0..self.num_faces);
            // Replace value at index with face value, or 0 for 'None' faces
            results[n] = match self.face_values[random_value] {
                Some(face_value) => face_value,
                None => 0
            }
        }
        results
    }

    /// Changes the value of a face
    pub fn modify_face(&mut self,
    face_index: usize,
    new_face_value: Option<usize>) 
    -> () {
        self.face_values[face_index] = new_face_value;
        ()
    }

    /// Helper method to remove face.
    /// Calls `modify_face` internally.
    pub fn destroy_face(&mut self,
    face_index: usize,)
    -> () {
        self.face_values[face_index] = None;
        ()
    }
}
