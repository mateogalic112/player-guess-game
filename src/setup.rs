use inquire::{InquireError, Select};

use crate::club::{Club, Country};

pub fn select_country() -> Option<Country> {
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

pub fn select_club(country: Country, clubs: &Vec<Club>) -> Option<Club> {
    let clubs = match country {
        Country::England => clubs
            .into_iter()
            .filter(|c| c.country == Country::England)
            .cloned()
            .collect(),
        Country::Spain => clubs
            .into_iter()
            .filter(|c| c.country == Country::Spain)
            .cloned()
            .collect(),
    };

    let clubs_ans: Result<Club, InquireError> = Select::new("Select club:", clubs).prompt();

    match clubs_ans {
        Ok(choice) => {
            println!("{}! That's mine too!", choice.name);
            Some(choice)
        }
        Err(_) => {
            println!("There was an error, please try again");
            None
        }
    }
}
