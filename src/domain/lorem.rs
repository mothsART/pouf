use clap::ArgMatches;
use fake::Fake;

use pouf::lang_env;

pub fn run(matches: &ArgMatches) {
    if let Some(w) = matches.subcommand_matches("lorem.word") {
        use fake::faker::lorem::raw::Word;
        each!(Word, w)
    }
}
