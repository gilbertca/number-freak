mod weapon;
use weapon::Weapon;

mod armor;
use armor::Armor;

mod body;
use body::Body;

pub struct Character {
    weapon: Option<Weapon>,
    armor: Option<Armor>,
    body: Body
}

impl Character {
    /// Creates a default character with given weapon and armor.
    pub fn new(weapon: Option<Weapon>,
    armor: Option<Armor>,
    body: Body)
    -> Character {
       Character { weapon, armor, body } 
    }

    /// Retrieves a `Character`'s name
    pub fn get_name(&self) -> &str {
        self.body.name.as_str()
    }

    /// Rolls all three of a `Character`'s dice a given number of times
    pub fn roll_for_duel(&mut self, num_rolls: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>,) {
        (self.weapon.roll(num_rolls)
         self
         self)
    }
}
