use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub mod constant;
pub mod simulation;

pub fn simulate_n(
    input_pulls: u32,
    input_pity_character: usize,
    input_capture_radiance: u32,
    input_focus_character: bool,
    input_pity_weapon: usize,
    input_epitomized_path: bool,
    input_focus_weapon: bool,
    input_constellation: i32,
    input_refinement: u32,
    wanted_constellation: i32,
    wanted_refinement: u32,
) -> f64 {
    let nb_simulation = 1_000_000;
    let res = Arc::new(Mutex::new(0_f64));

    (0..nb_simulation).into_par_iter().for_each(|_| {
        let (pulls, constellation, refinement) = simulation::simulate(
            input_pulls,
            input_pity_character,
            input_capture_radiance,
            input_focus_character,
            input_pity_weapon,
            input_epitomized_path,
            input_focus_weapon,
            input_constellation,
            input_refinement,
            wanted_constellation,
            wanted_refinement,
        );
        if pulls > 0 || (constellation == wanted_constellation && refinement == wanted_refinement) {
            let mut res_lock = res.lock().unwrap();
            *res_lock += 1.0;
        }
    });
    let res_lock = *res.lock().unwrap();
    res_lock / nb_simulation as f64
}
