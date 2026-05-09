use crate::enemy::Definition;
use crate::gameplay;
use std::collections::HashSet;
use std::io;

#[derive(Debug)]
pub enum Actions {
    PhysicalAttack,
    MagicAttack,
    Move,
}

#[derive(Debug)]
pub struct PlayerAction {
    pub player_actions: Actions,
}

#[derive(Debug)]
pub struct Target {
    pub target: Definition,
}

pub fn take_input() {
    loop {
        let mut input: String = String::new();
        println!(">");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        let trimmed = input.trim();
        if input.trim() == "quit" {
            break;
        }
        let result = parse_input(trimmed);
        match result {
            Ok(some) => {
                println!("{:?}", some)
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn parse_input(trimmed: &str) -> Result<PlayerAction, String> {
    let attack_actions: HashSet<&str> = [
        "battle",
        "fight",
        "war",
        "clash",
        "duel",
        "engage",
        "attack",
        "hit",
        "strike",
        "punch",
        "slap",
        "kick",
        "smash",
        "bash",
        "whack",
        "jab",
        "uppercut",
        "headbutt",
        "elbow",
        "knee",
        "assault",
        "ambush",
        "charge",
        "rush",
        "storm",
        "lunge",
        "pounce",
        "leap",
        "stab",
        "slash",
        "cut",
        "pierce",
        "shoot",
        "fire",
        "blast",
        "bomb",
        "shell",
        "beat",
        "pummel",
        "batter",
        "thrash",
        "maul",
        "crush",
        "wreck",
        "destroy",
        "grab",
        "grapple",
        "wrestle",
        "tackle",
        "pin",
        "choke",
        "strangle",
        "slam",
        "throw",
        "invade",
        "raid",
        "siege",
        "skirmish",
        "confront",
        "retaliate",
        "counter",
        "brawl",
        "scrap",
        "rumble",
    ]
    .into();

    let magic_actions: HashSet<&str> = [
        "cast",
        "spellcast",
        "invoke",
        "conjure",
        "summon",
        "channel",
        "weave",
        "manifest",
        "zap",
        "shock",
        "electrocute",
        "burn",
        "ignite",
        "scorch",
        "freeze",
        "chill",
        "explode",
        "detonate",
        "discharge",
        "surge",
        "fireball",
        "bolt",
        "missile",
        "arcane_blast",
        "energy_blast",
        "ray",
        "beam",
        "curse",
        "hex",
        "jinx",
        "haunt",
        "drain",
        "leech",
        "siphon",
        "push",
        "pull",
        "repel",
        "bind",
        "snare",
        "trap",
        "banish",
        "dispel",
        "raise",
        "animate",
        "spawn",
        "create",
        "materialize",
        "empower",
        "enchant",
        "infuse",
        "weaken",
        "cripple",
        "silence",
        "slow",
    ]
    .into();

    let move_actions: HashSet<&str> = [
        "run", "walk", "move", "go", "travel", "dash", "sprint", "jump", "climb", "crawl", "sneak",
        "advance", "retreat", "flee", "escape", "enter", "exit", "approach", "leave",
    ]
    .into();

    for word in trimmed.split_whitespace() {
        let lowercase = word.to_ascii_lowercase();

        if attack_actions.contains(lowercase.as_str()) {
            return Ok(PlayerAction {
                player_actions: Actions::PhysicalAttack,
            });
        } else if magic_actions.contains(lowercase.as_str()) {
            return Ok(PlayerAction {
                player_actions: Actions::MagicAttack,
            });
        } else if move_actions.contains(lowercase.as_str()) {
            return Ok(PlayerAction {
                player_actions: Actions::Move,
            });
        }
    }

    Err("Unspecified Action!".to_string())
}
