mod character;
use character::Character;

/// Duelling is a multi-step process. 
///
/// There are no rules dictating a difference
/// between attackers and defenders. 
///
/// Steps are as follows:
/// 1. Both characters simultaneously roll an Attack and Defend roll
///     - If a character has a Weapon
///     then they roll their weapons die for their Attack.
///     If the character has no Weapon, then they use their Body die.
///     - If a character has Armor
///     then they roll their armors die for their Defend.
///     If the character has no Armor, then they use their Body die. 
///     - If a character rolls an Destroyed number, the follow can happen:
///         1. Weapon: The Weapon is Ruined
///         until the Destroyed number is rolled again.
///         Once the Destroyed number is rolled again, that number
///         is restored back to the Die.
///         2. Armor: Nothing happens, but the Attack proceeds to the Body.
///         3. Body: The character dies and the duel ends,
///         unless both characters roll Destroyed numbers, then nothing occurs
///         (unless the roll would restore a Ruined weapon). 
/// 2. Both characters deal and suffer damage simultaneously. Rules for combat
/// are as follows:
///     - An Attack roll must be GREATER THAN a Defense roll
///     for an effect to occur. 
///     - If a Weapon loses an Attack, the number rolled is Destroyed.
///     If an Armor loses an Attack, the number rolled is Destroyed.
///     If the Body loses an Attack, the number rolled is Destroyed.
///     - It IS POSSIBLE for two characters
///     to kill each other at the same time. 
/// 3. Both characters repeat the previous process a certain number of times,
/// or until one of the characters dies. 
pub fn duel(character1: Character,
character2: Character,
number_of_turns: usize) {
    // Begin by rolling all three dice for each character:
    let (character1
}
