use clap::ArgMatches;
use fake::Fake;

use pouf::lang_env;

pub fn run(matches: &ArgMatches) {
    if let Some(code) = matches.subcommand_matches("http.code") {
        use fake::faker::http::raw::RfcStatusCode;
        each!(RfcStatusCode, code)
    }
}
