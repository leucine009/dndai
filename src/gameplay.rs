use crate::{commands, dice, enemy, player};
use std::io;

pub fn run() {
    println!("Welcome Adventurer!");
    println!("You have dared to challenge the perpetual forest of death!");
    println!("No one has ever attempted this feat before.");
    println!("The villagers are profoundly impressed by your courage.");
    println!(
        "And your limitless kindness in helping these helpless people by deafeating the monsters."
    );
    println!("Oh! I couldn't contain my excitement and forgot to ask you you're name adventurer.");
    println!("What's your name?");
    let mut input: String = String::new();
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
    println!("Player Initialized : ");
    let adventurer = player::PlayerDefinition::create_new_player(name);
    println!("{:?}", adventurer);
    println!(
        "--------------------------------------------------------------------------------------"
    );
    println!("You venture into the perpetual forest.");
    commands::take_input();
    let adversary = handle_enemy();
    println!("You encountered : {:?}", adversary);
}

pub fn handle_enemy() -> enemy::Definition {
    let adversary = enemy::create_random_enemy();
    println!("{:?}", adversary);
    adversary
}
