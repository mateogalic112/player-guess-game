pub struct Shortlist {
    pub query: String,
}

impl Shortlist {
    pub fn new(args: &[String]) -> Shortlist {
        let query = args[1].clone();

        Shortlist { query }
    }
}
