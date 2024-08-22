use clap::ArgMatches;
use fake::Fake;

use pouf::lang_env;

pub fn run(matches: &ArgMatches) {
    if let Some(l) = matches.subcommand_matches("address.country") {
        use fake::faker::address::raw::CountryName;
        use fake::locales::EN;

        return force_each!(CountryName, EN, l);
    }
    if let Some(l) = matches.subcommand_matches("address.city") {
        use fake::faker::address::raw::CityName;
        use fake::locales::EN;

        return force_each!(CityName, EN, l);
    }
    if let Some(l) = matches.subcommand_matches("address.street") {
        use fake::faker::address::raw::StreetName;
        use fake::locales::EN;

        force_each!(StreetName, EN, l)
    }
}
