#[derive(Debug)]
pub struct PlayerDefinition {
    pub name: String,
    pub max_hp: u8,
    pub hp: u8,
    pub attack_power: u8,
    pub level: u8,
}

impl PlayerDefinition {
    pub fn create_new_player(name: &str) -> Self {
        return PlayerDefinition {
            name: name.to_string(),
            max_hp: 10,
            hp: 10,
            attack_power: 10,
            level: 1,
        };
    }
}
