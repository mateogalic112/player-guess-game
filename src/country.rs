use std::fmt::{self, Display, Formatter};

use crate::club::Club;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Country {
    England,
    Spain,
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let country_name = match self {
            Country::England => "England",
            Country::Spain => "Spain",
        };
        write!(f, "{}", country_name)
    }
}

impl Country {
    pub fn all() -> Vec<Country> {
        vec![Country::England, Country::Spain]
    }

    pub fn get_clubs_from_country<'a>(country: Country, clubs: &'a Vec<Club>) -> Vec<&'a Club> {
        clubs.iter().filter(|c| c.country == country).collect()
    }
}
