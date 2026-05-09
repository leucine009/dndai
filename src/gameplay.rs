use crate::{
    commands::{self, Actions, PlayerAction},
    dice, enemy, player,
};
use std::io;

#[derive(Debug)]
pub struct Target {
    pub enemy: Option<enemy::Definition>,
}

pub fn run() {
    // Welcome message (keep your existing one)
    println!("Welcome Adventurer!");
    println!("You have dared to challenge the perpetual forest of death!");
    println!("No one has ever attempted this feat before.");
    println!("The villagers are profoundly impressed by your courage.");
    println!(
        "And your limitless kindness in helping these helpless people by defeating the monsters."
    );
    println!("Oh! I couldn't contain my excitement and forgot to ask you your name adventurer.");
    println!("What's your name?");

    let mut input = String::new();
    println!(">");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    let name = input.trim();

    println!("{}, How heroic and brave name.", name);
    println!("The villagers are assured you'll save them from their fate.");
    println!("Good luck, {}", name);
    println!(
        "--------------------------------------------------------------------------------------"
    );

    let mut adventurer = player::PlayerDefinition::create_new_player(name);
    println!("Player Initialized : {:?}", adventurer);
    println!(
        "--------------------------------------------------------------------------------------"
    );
    println!("You venture into the perpetual forest.");

    // GAME LOOP
    loop {
        // Check if player is alive
        if adventurer.hp <= 0 {
            println!("You have died! Game Over!");
            break;
        }

        // Random chance to encounter enemy (odd numbers = encounter)
        let encounter_roll = dice::roll_d20();
        println!("[DEBUG] Encounter roll: {}", encounter_roll);

        if encounter_roll % 2 == 1 {
            // Odd number = enemy!
            println!("\n⚔️ A monster appears! ⚔️\n");

            let mut enemy = enemy::create_random_enemy();
            println!(
                "You face: {:?} (Level: {:?})",
                enemy.enemy_type, enemy.enemy_level
            );
            println!("Enemy HP: {}, Attack: {}", enemy.hp, enemy.attack_power);

            // BATTLE LOOP
            while enemy.hp > 0 && adventurer.hp > 0 {
                println!(
                    "\nYour HP: {}/{}, Enemy HP: {}/{}",
                    adventurer.hp, adventurer.max_hp, enemy.hp, enemy.max_hp
                );
                println!("What do you do?");

                let action = commands::take_input();

                match action {
                    Ok(player_action) => {
                        match player_action.player_actions {
                            Actions::PhysicalAttack => {
                                let attack_roll = dice::roll_d20();
                                println!("Attack roll: {}", attack_roll);

                                if attack_roll >= 10 {
                                    // 50% chance to hit
                                    let damage = adventurer.attack_power;
                                    enemy.hp = enemy.hp.saturating_sub(damage);
                                    println!("Hit! You deal {} damage!", damage);
                                } else {
                                    println!("Miss!");
                                }
                            }
                            Actions::MagicAttack => {
                                let magic_roll = dice::roll_d20();
                                println!("Magic roll: {}", magic_roll);

                                if magic_roll >= 8 {
                                    // Slightly better chance
                                    let damage = adventurer.attack_power + 2;
                                    enemy.hp = enemy.hp.saturating_sub(damage);
                                    println!("Spell hits! You deal {} damage!", damage);
                                } else {
                                    println!("Spell fizzles!");
                                }
                            }
                            Actions::Move => {
                                println!("You try to flee!");
                                let flee_roll = dice::roll_d20();
                                if flee_roll >= 12 {
                                    println!("You escaped!");
                                    break; // Exit battle
                                } else {
                                    println!("Failed to escape!");
                                }
                            }
                        }
                    }
                    Err(msg) => {
                        println!("{}", msg);
                        continue;
                    }
                }

                // Enemy turn (if still alive)
                if enemy.hp > 0 && adventurer.hp > 0 {
                    let enemy_roll = dice::roll_d20();
                    println!("Enemy attacks! Roll: {}", enemy_roll);

                    if enemy_roll >= 8 {
                        // Enemy hits on 8+
                        let damage = enemy.attack_power;
                        adventurer.hp = adventurer.hp.saturating_sub(damage);
                        println!("Enemy hits! You take {} damage!", damage);
                    } else {
                        println!("Enemy misses!");
                    }
                }
            }

            // Battle result
            if enemy.hp <= 0 {
                println!(
                    "\n🎉 Victory! You defeated the {:?}! 🎉\n",
                    enemy.enemy_type
                );
                // Small heal after battle
                adventurer.hp = (adventurer.hp + 2).min(adventurer.max_hp);
            }
        } else {
            println!("The forest is quiet... you continue your journey.");

            // Small heal when no enemy
            adventurer.hp = (adventurer.hp + 1).min(adventurer.max_hp);
        }

        // Simple way to exit
        println!("\nPress Enter to continue, or type 'quit' to exit...");
        let mut exit_input = String::new();
        io::stdin().read_line(&mut exit_input).unwrap();
        if exit_input.trim().to_lowercase() == "quit" {
            println!("Thanks for playing!");
            break;
        }
    }
}
