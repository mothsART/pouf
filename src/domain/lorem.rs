use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(w) = matches.subcommand_matches("lorem.word") {
        use fake::faker::lorem::raw::Word;
        return each!(Word, w);
    }
}
