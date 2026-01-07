use crate::character::Character;

pub enum DuelResult {
    WeaponDamaged,
    ArmorDamaged,
    BodyDamaged,
    Victory(usize), // 1 = P1, 2 = P2

    NoEffect,

}

pub fn duel(mut character1: Character,
mut character2: Character,
number_of_turns: usize)
-> Vec<DuelResult> {
    let mut results: Vec<DuelResult> = Vec::with_capacity(number_of_turns); 

    // Main loop
    for turn_num in 0..number_of_turns {
        // Begin by rolling dice for each character:
        let rolls: Vec<_> = [&mut character1, &mut character2]
            .iter_mut()
            .map(|c| c.roll_for_duel(1))
            .collect();

        let weapon_rolls = [rolls[0][0].clone(), rolls[1][0].clone()];
        let armor_rolls = [rolls[0][1].clone(), rolls[1][1].clone()];
        let body_rolls = [rolls[0][2].clone(), rolls[1][2].clone()];

        // Process targets of effects individually
        // Attack 1
        for (index, weapon_roll) in weapon_rolls[0].iter().enumerate() {
            if *weapon_roll > armor_rolls[1][index] { // Success
                if armor_rolls[1][index] == 0 { // If armor is broken
                    if *weapon_roll > body_rolls[1][index] { // Hit body
                        if body_rolls[1][index] == 0 { // Dead
                            results.resize(
                                number_of_turns,
                                DuelResult::Victory(1)
                            )
                            return ();
                        }
                    }
                }
            }
        }
        // Attack 1 end
    }
}
