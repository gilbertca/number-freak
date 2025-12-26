mod dice;
use dice::Die;

pub struct Weapon {
    name: String,
    die: Die,
    ruined: bool
}

impl Weapon {
    /// Creates a new `Weapon` with a given name and default values.
    pub fn new(name: String, die: Die) -> Weapon {
        Weapon { name, die }
    }
}
