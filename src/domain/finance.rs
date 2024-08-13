use clap::ArgMatches;
use fake::Fake;

use pouf::lang_env;

pub fn run(matches: &ArgMatches) {
    if let Some(bic) = matches.subcommand_matches("finance.bic") {
        use fake::faker::finance::raw::Bic;
        each!(Bic, bic)
    }
}
