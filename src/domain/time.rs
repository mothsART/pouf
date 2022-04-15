use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(date) = matches.subcommand_matches("time.date") {
        use fake::faker::chrono::raw::DateTime;
        return each!(DateTime, date);
    }

    if let Some(_time) = matches.subcommand_matches("time.time") {
        use fake::faker::chrono::raw::Time;
        return each!(Time, _time);
    }
}
