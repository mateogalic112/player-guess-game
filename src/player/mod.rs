#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub age: u8,
    position: Position,
    club: String,
    pub market_value: u8,
}

#[derive(Debug)]
pub enum Position {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

impl Player {
    pub fn player_info(&self) -> String {
        format!(
            "Player {} ({}, {:?}) plays for {}. Valued at: {} mil.",
            self.name, self.age, self.position, self.club, self.market_value
        )
    }

    pub fn is_older(&self, other: &Player) -> bool {
        self.age > other.age
    }

    pub fn is_more_valued(&self, other: &Player) -> bool {
        self.market_value > other.market_value
    }

    pub fn transfer(&mut self, new_club: String, fee: u8) {
        self.club = new_club;
        self.market_value = (self.market_value + fee) / 2;
    }
}

impl Player {
    pub fn find_player_by_name<'a>(
        players: &'a Vec<Player>,
        input_name: &'a String,
    ) -> Option<&'a Player> {
        players
            .iter()
            .find(|player| player.name.to_lowercase() == input_name.to_lowercase().trim())
    }

    pub fn is_oldest(players: &Vec<Player>, player: &Player) -> bool {
        players
            .iter()
            .all(|other| other.name == player.name || player.is_older(other))
    }

    pub fn is_most_valued(players: &Vec<Player>, player: &Player) -> bool {
        players
            .iter()
            .all(|other| other.name == player.name || player.is_more_valued(other))
    }
}

impl Player {
    pub fn create_from_line(line: &str) -> Option<Self> {
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

        Some(Player {
            name,
            age,
            position,
            club,
            market_value,
        })
    }
}
