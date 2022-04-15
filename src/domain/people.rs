use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(name) = matches.subcommand_matches("people.name") {
        use fake::faker::name::raw::{Name, FirstName, LastName};

        if name.is_present("firstname") && name.is_present("lastname") {
            each!(Name, name);
            return;
        }
        if name.is_present("firstname") {
            each!(FirstName, name);
            return;
        }
        if name.is_present("lastname") {
            each!(LastName, name);
            return;
        }
        each!(Name, name);
        return;
    }
}
