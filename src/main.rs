use std::fs::read_to_string;
use std::io::{self, Error};

#[derive(Debug)]
struct Player {
    name: String,
    age: u8,
    club: String,
}

fn main() -> Result<(), Error> {
    const FILE_NAME: &str = "players.txt";

    let players = read_lines(FILE_NAME).unwrap();

    loop {
        println!("Please input player name.");

        let mut player_name_guess: String = String::new();

        io::stdin()
            .read_line(&mut player_name_guess)
            .expect("Failed to read line");

        if players
            .iter()
            .any(|player| player.name.to_lowercase() == player_name_guess.to_lowercase().trim())
        {
            println!("Player found!");
            break;
        } else {
            println!("Player not found!");
        }
    }

    Ok(())
}

fn parse_player_line(line: &str) -> Option<Player> {
    let parts: Vec<&str> = line.split(" - ").collect();

    if parts.len() != 3 {
        return None;
    }

    let name = parts[0].trim().to_string();
    let age = parts[1].trim().parse::<u8>().unwrap();
    let club = parts[2].trim().to_string();

    Some(Player { name, age, club })
}

fn read_lines(filename: &str) -> Result<Vec<Player>, Error> {
    let content = read_to_string(filename)?;

    let players: Vec<Player> = content.lines().filter_map(parse_player_line).collect();

    Ok(players)
}
