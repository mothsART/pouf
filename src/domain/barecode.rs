use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(isbn) = matches.subcommand_matches("barecode.isbn") {
        use fake::faker::barecode::raw::Isbn13;
        return each!(Isbn13, isbn);
    }
}
