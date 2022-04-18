use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(l) = matches.subcommand_matches("address.country") {
        use fake::locales::EN;
        use fake::faker::address::raw::CountryName;
        return force_each!(CountryName, EN, l);
    }
    if let Some(l) = matches.subcommand_matches("address.city") {
        use fake::locales::EN;
        use fake::faker::address::raw::CityName;
        return force_each!(CityName, EN, l);
    }
    if let Some(l) = matches.subcommand_matches("address.street") {
        use fake::locales::EN;
        use fake::faker::address::raw::StreetName;
        return force_each!(StreetName, EN, l);
    }
}
