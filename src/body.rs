use crate::die::Die;

pub struct Body {
    name: String,
    die: Die
}

impl Body {
    /// Creates a new `Body` instance with a Character name and default values.
    pub fn new(name: String, die: Die) -> Body {
        Body { name, die }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Rolls the body's die and returns the results
    pub fn roll(&mut self, num_rolls: usize) -> Vec<usize> {
        self.die.roll(num_rolls)
    }
}
