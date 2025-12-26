mod dice;
use dice::Die;

pub struct Armor {
    name: String,
    die: Die
}

impl Armor {
    /// Creates a new `Armor` instance with a given name and default values.
    pub fn new(name: String, die: Die) -> Armor {
        Armor { name, die }
    }
}
