use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(l) = matches.subcommand_matches("administrative.healthinsurrancecode") {
        use fake::locales::FR_FR;
        use fake::faker::administrative::raw::HealthInsuranceCode;
        return force_each!(HealthInsuranceCode, FR_FR, l);
    }
}
