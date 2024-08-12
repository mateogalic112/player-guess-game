use crate::player::Player;

pub enum Country {
    England,
}

pub struct Club {
    pub country: Country,
    pub name: String,
    pub transfer_budget: u16,
    pub squad: Vec<Player>,
}

impl Club {
    pub fn new(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(" - ").collect();

        if parts.len() != 3 {
            return None;
        }

        let country: Country = match parts[0].trim() {
            "England" => Country::England,
            _ => return None,
        };

        let name = parts[1].trim().to_string();

        let transfer_budget = parts[2].trim().parse::<u16>();
        let transfer_budget: u16 = match transfer_budget {
            Ok(value) => value,
            Err(_) => 0,
        };

        Some(Club {
            country,
            name,
            transfer_budget,
            squad: Vec::new(),
        })
    }

    pub fn get_text_file() -> &'static str {
        const CLUBS_FILE: &str = "clubs.txt";
        CLUBS_FILE
    }
}
