use std::fs::read_to_string;
use std::io::{self, Error};

#[derive(Debug)]
enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

#[derive(Debug)]
struct Player {
    name: String,
    age: u8,
    position: Position,
    club: String,
    market_value: u8,
}

impl Player {
    fn player_info(&self) -> String {
        format!("Player {} ({}, {:?}) plays for {}. Valued at: {} million.", self.name, self.age, self.position, self.club, self.market_value)
    }

    fn is_older(&self, other: &Player) -> bool {
        self.age > other.age
    }
}

fn main() {
    const FILE_NAME: &str = "players.txt";

    let players = read_lines(FILE_NAME).unwrap();

    println!("Please input player name: ");
    loop {
        let mut player_name_guess: String = String::new();

        io::stdin()
            .read_line(&mut player_name_guess)
            .expect("Failed to read line");

        match find_player_by_name(&players, player_name_guess) {
            Some(player) => {
                if is_oldest(&players, player) {
                    println!("{} is the oldest player in the squad - {} years old.", player.name, player.age);
                }

                if is_most_valued(&players, player) {
                    println!("{} is the most valued player in the squad - {} million.", player.name, player.market_value);
                }

                println!("{}", player.player_info());
                break;
            }
            None => println!("Player not found, try again: "),
        }
    }
}


fn find_player_by_name(players: &Vec<Player>, player_name_guess: String) -> Option<&Player> {
    players
    .iter()
    .find(|player| player.name.to_lowercase() == player_name_guess.to_lowercase().trim())
}

fn is_oldest(players: &Vec<Player>, player: &Player) -> bool {
    players
        .iter()
        .filter(|other| other.name != player.name)
        .all(|other| player.is_older(other))
}

fn is_most_valued(players: &Vec<Player>, player: &Player) -> bool {
    players
        .iter()
        .filter(|other| other.name != player.name)
        .all(|other| player.market_value > other.market_value)
}

fn parse_player_line(line: &str) -> Option<Player> {
    let parts: Vec<&str> = line.split(" - ").collect();

    if parts.len() != 5 {
        return None;
    }

    let name = parts[0].trim().to_string();

    let age = parts[1].trim().parse::<u8>();
    let age = match age {
        Ok(value) => value,
        Err(_) => 0,
    };
    

    let position: Position = match parts[2].trim() {
        "GK" => Position::Goalkeeper,
        "CB" | "DL" | "DR" => Position::Defender,
        "CM" | "DM" => Position::Midfielder,
        "CF" | "ST" => Position::Forward,
        _ => return None,
    };

    let club = parts[3].trim().to_string();

    let market_value = parts[4].trim().parse::<u8>();
    let market_value: u8 = match market_value {
        Ok(value) => value,
        Err(_) => 0,
    };

    Some(Player { name, age, position, club, market_value })
}

fn read_lines(filename: &str) -> Result<Vec<Player>, Error> {
    let content = read_to_string(filename)?;

    let players: Vec<Player> = content
        .lines()
        .filter_map(parse_player_line)
        .collect();

    Ok(players)
}
