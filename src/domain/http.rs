use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(code) = matches.subcommand_matches("http.code") {
        use fake::faker::http::raw::RfcStatusCode;
        return each!(RfcStatusCode, code);
    }
}
