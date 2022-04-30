use clap::ArgMatches;
use fake::locales::FR_FR;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(l) = matches.subcommand_matches("auto.licenseplate") {
        use fake::faker::automotive::raw::LicencePlate;
        force_each!(LicencePlate, FR_FR, l);
    }
}
