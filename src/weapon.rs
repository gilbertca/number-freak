use crate::die::Die;

pub struct Weapon {
    name: String,
    die: Die,
    ruined: bool
}

impl Weapon {
    /// Creates a new `Weapon` with a given name and default values.
    pub fn new(name: String, die: Die) -> Weapon {
        Weapon { name, die, ruined: false }
    }

    /// Rolls the weapon's die and returns the results
    pub fn roll(&mut self, num_rolls: usize) -> Vec<usize> {
        self.die.roll(num_rolls)
    }
}
