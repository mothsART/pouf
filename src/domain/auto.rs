use clap::ArgMatches;
use fake::Fake;
use fake::locales::FR_FR;

pub fn run(matches: &ArgMatches) {
    if let Some(_) = matches.subcommand_matches("auto.licenseplate") {
        use fake::faker::automotive::raw::LicencePlate;
        let val: String = LicencePlate(FR_FR).fake();
        println!("{}", val);
    }
}
