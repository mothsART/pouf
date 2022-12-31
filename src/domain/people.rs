use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(name) = matches.subcommand_matches("people.name") {
        use fake::faker::name::raw::{FirstName, LastName, Name, NameWithTitle, Title};

        if name.contains_id("title") {
            return each!(Title, name);
        }
        if name.contains_id("with-title") {
            return each!(NameWithTitle, name);
        }
        if name.contains_id("firstname") && name.contains_id("lastname") {
            return each!(Name, name);
        }
        if name.contains_id("firstname") {
            return each!(FirstName, name);
        }
        if name.contains_id("lastname") {
            return each!(LastName, name);
        }
        each!(Name, name)
    }
}
