use rand::prelude::*;

/// generates one die
fn generate_die(sides: i8) -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides)
}

/// generates multiple dice
pub fn generate_dice(dice: i8, sides: i8) -> Vec<i8> {
    let mut results: Vec<i8> = Vec::with_capacity(dice as usize);

    for _ in 1..=dice {
        results.push(generate_die(sides));
    }

    results
}
