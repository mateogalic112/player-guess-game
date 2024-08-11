use crate::player::Player;

pub struct Club {
    pub name: String,
    pub squad: Vec<Player>,
}

impl Club {
    pub fn new(name: String) -> Self {
        Club {
            name,
            squad: Vec::new(),
        }
    }
}
