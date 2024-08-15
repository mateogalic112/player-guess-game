use std::io::Write;
use std::{fs::File, io::Read};

use inquire::{InquireError, Select};

use crate::club::{Club, Country};

fn select_country() -> Option<Country> {
    let country_options: Vec<Country> = vec![Country::England, Country::Spain];

    let country_ans: Result<Country, InquireError> =
        Select::new("Select club country?", country_options).prompt();

    match country_ans {
        Ok(choice) => {
            println!("{}! That's mine too!", choice);
            Some(choice)
        }
        Err(_) => {
            println!("There was an error, please try again");
            None
        }
    }
}

fn select_club(country: &Country, clubs: &Vec<Club>) -> Option<Club> {
    let clubs = match country {
        Country::England => clubs
            .into_iter()
            .filter(|c| c.country == Country::England)
            .collect(),
        Country::Spain => clubs
            .into_iter()
            .filter(|c| c.country == Country::Spain)
            .collect(),
    };

    let clubs_ans: Result<&Club, InquireError> = Select::new("Select club:", clubs).prompt();

    match clubs_ans {
        Ok(choice) => {
            println!("{}! That's mine too!", choice.name);
            Some(choice.clone())
        }
        Err(_) => {
            println!("There was an error, please try again");
            None
        }
    }
}

pub fn init(game_file: &mut File, clubs: &Vec<Club>) -> Club {
    let mut file_content = String::new();
    game_file.read_to_string(&mut file_content).unwrap();

    let club: Club = match file_content.is_empty() {
        true => {
            let country: Country = select_country().unwrap();
            let selected_club = select_club(&country, &clubs).unwrap();
            writeln!(game_file, "Current club: {}", selected_club).unwrap();
            selected_club
        }
        false => {
            let club_name = file_content.lines().fold(String::new(), |mut acc, el| {
                if el.starts_with("Current club: ") {
                    acc = el.split("Current club: ").last().unwrap().to_string();
                }
                acc
            });

            let selected_club = clubs.iter().find(|c| c.name == club_name).unwrap().clone();
            selected_club
        }
    };

    println!("Welcome to the game! You are now managing {}!", club.name);

    club
}
