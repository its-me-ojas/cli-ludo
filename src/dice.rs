use rand::prelude::*;

pub fn roll_dice() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}
