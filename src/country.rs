use std::fmt::{self, Display, Formatter};

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
}
