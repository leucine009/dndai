use rand::RngExt;

#[derive(Debug)]
pub enum EnemyType {
    Human,
    Goblin,
    Orc,
    Imp,
    Dragon,
}

#[derive(Debug)]
pub enum Level {
    Regular,
    MidBoss,
    Boss,
}

#[derive(Debug)]
pub struct Definition {
    pub enemy_type: EnemyType,
    pub enemy_level: Level,
    pub hp: u8,
    pub max_hp: u8,
    pub attack_power: u8,
}

pub fn create_random_enemy() -> Definition {
    let mut rng = rand::rng();
    let random_enemy_type: EnemyType = match rng.random_range(0..=4) {
        0 => EnemyType::Human,
        1 => EnemyType::Goblin,
        2 => EnemyType::Orc,
        3 => EnemyType::Imp,
        4 => EnemyType::Dragon,
        _ => unreachable!(),
    };
    let random_level: Level = match rng.random_range(0..=100) {
        0..=85 => Level::Regular,
        86..=98 => Level::MidBoss,
        99..=100 => Level::Boss,
        _ => unreachable!(),
    };

    let base_hp = match random_enemy_type {
        EnemyType::Human => 10,
        EnemyType::Goblin => 8,
        EnemyType::Orc => 15,
        EnemyType::Imp => 6,
        EnemyType::Dragon => 20,
    };

    let multiplier = match random_level {
        Level::Regular => 1,
        Level::MidBoss => 2,
        Level::Boss => 4,
    };

    let max_hp: u8 = base_hp * multiplier;

    let base_attack = match random_enemy_type {
        EnemyType::Human => 5,
        EnemyType::Goblin => 4,
        EnemyType::Orc => 8,
        EnemyType::Imp => 3,
        EnemyType::Dragon => 12,
    };

    let attack_multiplier = match random_level {
        Level::Regular => 1,
        Level::MidBoss => 2,
        Level::Boss => 3,
    };

    let attack_power: u8 = base_attack * attack_multiplier;

    return Definition {
        enemy_type: random_enemy_type,
        enemy_level: random_level,
        max_hp: max_hp,
        hp: max_hp,
        attack_power: attack_power,
    };
}
