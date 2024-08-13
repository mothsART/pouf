use clap::ArgMatches;
use fake::Fake;

use pouf::lang_env;

pub fn run(matches: &ArgMatches) {
    if let Some(isbn) = matches.subcommand_matches("barcode.isbn") {
        use fake::faker::barcode::raw::Isbn13;
        each!(Isbn13, isbn)
    }
}
