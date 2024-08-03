#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub age: u8,
    pub position: Position,
    pub club: String,
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

    pub fn transfer(&mut self, new_club: String, fee: u8) {
        self.club = new_club;
        self.market_value = (self.market_value + fee) / 2;
    }
}
