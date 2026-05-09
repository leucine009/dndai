use rand::RngExt;

pub fn roll_d20() -> u8 {
    let mut rng = rand::rng();
    let roll = rng.random_range(1..=20);
    roll
}
