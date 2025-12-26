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
