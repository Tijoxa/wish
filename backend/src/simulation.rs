use rand::prelude::*;

use crate::constant::{X, Y};

#[allow(clippy::too_many_arguments)]
pub fn simulate(
    input_pulls: u32,
    input_pity_character: usize,
    input_capturing_radiance: u32,
    input_focus_character: bool,
    input_pity_weapon: usize,
    input_epitomized_path: bool,
    input_focus_weapon: bool,
    input_constellation: i32,
    input_refinement: u32,
    wanted_constellation: i32,
    wanted_refinement: u32,
) -> (u32, i32, u32) {
    // one round of simulation
    let mut pulls = input_pulls;
    let mut pity_character = input_pity_character;
    let mut capturing_radiance = input_capturing_radiance;
    let mut focus_character = input_focus_character;
    let mut pity_weapon = input_pity_weapon;
    let mut epitomized_path = input_epitomized_path;
    let mut focus_weapons = input_focus_weapon;
    let mut constellation = input_constellation;
    let mut refinement = input_refinement;

    let mut rng = thread_rng();

    while (pulls > 0) && (constellation < wanted_constellation || refinement < wanted_refinement) {
        // simulate a wish
        pulls -= 1;

        if constellation < wanted_constellation {
            pull_character(
                &mut rng,
                &mut pity_character,
                &mut focus_character,
                &mut constellation,
                &mut capturing_radiance,
            );
        } else {
            pull_weapon(
                &mut rng,
                &mut pity_weapon,
                &mut epitomized_path,
                &mut refinement,
                &mut focus_weapons,
            );
        }
    }

    (pulls, constellation, refinement)
}

fn pull_character(
    rng: &mut ThreadRng,
    pity_character: &mut usize,
    focus_character: &mut bool,
    constellation: &mut i32,
    capturing_radiance: &mut u32,
) {
    let rand: f64 = rng.gen();

    if rand < X[*pity_character] {
        // pull 5*
        *pity_character = 0;

        if *focus_character {
            *constellation += 1;
            *focus_character = false;
        } else if *capturing_radiance <= 1 {
            // no capturing radiance
            let randbool: bool = rng.gen();

            if randbool {
                // lose
                *focus_character = true;
                *capturing_radiance += 1;
            } else {
                *focus_character = false;
                *capturing_radiance = 0;
                *constellation += 1;
            }
        } else if *capturing_radiance == 2 {
            // it's a 75/25
            let randrange = rng.gen_range(0..=3);

            if randrange == 0 {
                // lose
                *focus_character = true;
                *capturing_radiance += 1;
            } else {
                *focus_character = false;
                *capturing_radiance = 0;
                *constellation += 1;
            }
        } else {
            // it's guaranteed
            *focus_character = false;
            *capturing_radiance = 0;
            *constellation += 1;
        }
    } else {
        *pity_character += 1;
    }
}

fn pull_weapon(
    rng: &mut ThreadRng,
    pity_weapon: &mut usize,
    epitomized_path: &mut bool,
    refinement: &mut u32,
    focus_weapons: &mut bool,
) {
    let rand: f64 = rng.gen();

    if rand < Y[*pity_weapon] {
        // pull 5*
        *pity_weapon = 0;

        if *epitomized_path {
            *refinement += 1;
            *epitomized_path = false;
        } else if *focus_weapons {
            // it's a 50/50
            *focus_weapons = false;
            let randbool: bool = rng.gen();

            if randbool {
                // lose
                *epitomized_path = true;
            } else {
                *epitomized_path = false;
                *refinement += 1;
            }
        } else {
            // it's a 37.5/62.5
            let randrange: u32 = rng.gen_range(0..=3);

            if randrange == 0 {
                // lose ×2
                *focus_weapons = true;
                *epitomized_path = true;
            } else {
                *focus_weapons = false;
                let inner_randbool: bool = rng.gen();

                if inner_randbool {
                    // lose
                    *epitomized_path = true;
                } else {
                    *epitomized_path = false;
                    *refinement += 1;
                }
            }
        }
    } else {
        *pity_weapon += 1;
    }
}
