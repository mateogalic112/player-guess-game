pub struct Shortlist {
    pub query: String,
}

impl Shortlist {
    pub fn build(args: &[String]) -> Result<Shortlist, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();

        Ok(Shortlist { query })
    }
}
