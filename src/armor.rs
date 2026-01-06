use crate::die::Die;

pub struct Armor {
    name: String,
    die: Die
}

impl Armor {
    /// Creates a new `Armor` instance with a given name and default values.
    pub fn new(name: String, die: Die) -> Armor {
        Armor { name, die }
    }

    /// Rolls the armor's die and returns the results
    pub fn roll(&mut self, num_rolls: usize) -> Vec<usize> {
        self.die.roll(num_rolls)
    }
}
