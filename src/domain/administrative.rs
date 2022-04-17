use clap::ArgMatches;
use fake::Fake;
use fake::locales::FR_FR;

pub fn run(matches: &ArgMatches) {
    if let Some(l) = matches.subcommand_matches("administrative.healthinsurrancecode") {
        use fake::faker::administrative::raw::HealthInsuranceCode;
        force_each!(HealthInsuranceCode, FR_FR, l);
    }
}
