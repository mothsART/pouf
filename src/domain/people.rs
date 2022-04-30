use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(name) = matches.subcommand_matches("people.name") {
        use fake::faker::name::raw::{FirstName, LastName, Name, NameWithTitle, Title};

        if name.is_present("title") {
            return each!(Title, name);
        }
        if name.is_present("with-title") {
            return each!(NameWithTitle, name);
        }
        if name.is_present("firstname") && name.is_present("lastname") {
            return each!(Name, name);
        }
        if name.is_present("firstname") {
            return each!(FirstName, name);
        }
        if name.is_present("lastname") {
            return each!(LastName, name);
        }
        return each!(Name, name);
    }
}
