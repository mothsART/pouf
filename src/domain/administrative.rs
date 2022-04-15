use clap::ArgMatches;
use fake::Fake;
use fake::locales::FR_FR;

pub fn run(matches: &ArgMatches) {
    if let Some(_) = matches.subcommand_matches("administrative.healthinsurrancecode") {
        use fake::faker::administrative::raw::HealthInsuranceCode;

        let val: String = HealthInsuranceCode(FR_FR).fake();
        println!("{}", val);
    }
}
