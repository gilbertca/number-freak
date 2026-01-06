use crate::character::Character;

/// Duelling is a multi-step process. 
///
/// There are no rules dictating a difference
/// between attackers and defenders. 
///
/// Steps are as follows, and are repeated until a character dies,
/// or until a chosen number of turns have passed in the duel:
/// ## 1) ROLLING:
/// Both characters roll an Attack, Defense, and Body roll.
/// 
/// If a character does not have a Weapon or Armor,
/// then the values for their Attack and Defense roll
/// are equal to the value of their Body roll respectively.
/// 
/// Rolls and their effects occur simultaneously.
///
/// ## 2) EFFECTS:
/// Effects are determined by comparing the values
/// of the opposing Attack and Defense rolls.
///
/// An Attack Roll must be greater than a Defense roll for an effect to occur.
///
/// If an Attack succeeds, it destroys the face of the Defending die.
/// For example, with an Attack of 6 and a Defense of 1,
/// the '1' side of the die is destroyed.
/// This holds true for Defense rolls using Armor,
/// and defense rolls using Body.
///
/// If the Attack roll strikes an already-destroyed face,
/// a different effect will occur (as described in section 3).
///
/// If the Attack roll fails, either from an Armor or Body
/// defense roll, then the rolled Attack face is destroyed.
/// For example, with an Attack of 1, and a Defense of 6,
/// the '1' side of the die is destroyed.
/// This holds true for Defense rolls with Armor and Body.
///
/// ## 3) ROLLING EMPTY FACES:
/// **REPAIRING WEAPONS**
/// If an Attack roll lands on a Destroyed face, then that character
/// attempts to repair their weapon. That character will only make
/// a Defense & Body Roll and a Repair Roll; they will not be able to attack
/// until the weapon is repaired.
/// The character makes their Repair Roll using their damaged weapon.
/// They make Repair rolls until they roll the Destroyed number
/// that was initially rolled to enter Repair mode -
/// which repairs the adjacent faces as well -
/// or 1 value above or below that number -
/// which only repairs the initially damaged face.
/// For example, a Weapon's '2' face is damaged, and then rolled again.
/// If the character rolls a '1' or a '3', then the '2' face is repaired.
/// If the character rolls a '2', then the '1' '2' and '3' faces are repaired.
/// The character will then resume to make Attack and Defense rolls as usual.
///
/// **EMPTY FACES ON DEFENSE**
/// If a Defense roll made with Armor lands on an empty face,
/// then nothing happens.
/// If an Attack roll hits armor that landed on an empty face,
/// then the Attack is able to *pierece* the armor,
/// and the Attack roll is then compared to the Body roll instead.
///
/// **EMPTY FACES ON BODY**
/// If an Attack strikes a destroyed face on a Body die, then the opposing
/// character dies and the duel ends. This is the ultimate goal of dueling.
///
/// It is possible for two Characters to simultaneously kill each other.
pub fn duel(character1: Character,
character2: Character,
number_of_turns: usize) -> (){
    ()
}
