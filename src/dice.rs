use rand::prelude::*;

pub fn roll_dice() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}
