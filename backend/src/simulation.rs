use rand::prelude::*;
use rand::rngs::SmallRng;

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
    let mut rng = SmallRng::from_entropy();
    simulate_with_rng(
        &mut rng,
        input_pulls,
        input_pity_character,
        input_capturing_radiance,
        input_focus_character,
        input_pity_weapon,
        input_epitomized_path,
        input_focus_weapon,
        input_constellation,
        input_refinement,
        wanted_constellation,
        wanted_refinement,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn simulate_with_rng<R: Rng>(
    rng: &mut R,
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

    while (pulls > 0) && (constellation < wanted_constellation || refinement < wanted_refinement) {
        // simulate a wish
        pulls -= 1;

        if constellation < wanted_constellation {
            pull_character(
                rng,
                &mut pity_character,
                &mut focus_character,
                &mut constellation,
                &mut capturing_radiance,
            );
        } else {
            pull_weapon(
                rng,
                &mut pity_weapon,
                &mut epitomized_path,
                &mut refinement,
                &mut focus_weapons,
            );
        }
    }

    (pulls, constellation, refinement)
}

fn pull_character<R: Rng>(
    rng: &mut R,
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
            // it's a 95/5
            let randrange = rng.gen_range(0..=19);

            if randrange > 0 {
                // lose
                *focus_character = true;
                *capturing_radiance = 3;
            } else {
                *focus_character = false;
                *capturing_radiance = 1;
                *constellation += 1;
            }
        } else {
            // it's guaranteed
            *focus_character = false;
            *capturing_radiance = 1;
            *constellation += 1;
        }
    } else {
        *pity_character += 1;
    }
}

fn pull_weapon<R: Rng>(
    rng: &mut R,
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

