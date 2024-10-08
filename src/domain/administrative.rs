use clap::ArgMatches;
use fake::Fake;

use pouf::lang_env;

pub fn run(matches: &ArgMatches) {
    if let Some(l) = matches.subcommand_matches("administrative.healthinsurrancecode") {
        use fake::faker::administrative::raw::HealthInsuranceCode;
        use fake::locales::FR_FR;

        force_each!(HealthInsuranceCode, FR_FR, l)
    }
}
