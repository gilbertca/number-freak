use crate::weapon::Weapon;
use crate::armor::Armor;
use crate::body::Body;

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
        self.body.get_name()
    }

    /// Rolls all three of a `Character`'s dice a given number of times
    pub fn roll_for_duel(&mut self, num_rolls: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>,) {
        let weapon_rolls = match &mut self.weapon {
            None => vec![0;num_rolls],
            Some(weapon_die) => weapon_die.roll(num_rolls)
        };
        let armor_rolls = match &mut self.armor {
            None => vec![0;num_rolls],
            Some(armor_die) => armor_die.roll(num_rolls)
        };
        let body_rolls = self.body.roll(num_rolls);

        (weapon_rolls, armor_rolls, body_rolls)
    }
}
