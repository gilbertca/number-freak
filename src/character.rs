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
}
