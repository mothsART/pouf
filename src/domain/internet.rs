use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(mail) = matches.subcommand_matches("internet.mail") {
        use fake::faker::internet::raw::FreeEmail;
        return each!(FreeEmail, mail);
    }

    if let Some(mac) = matches.subcommand_matches("internet.mac") {
        use fake::faker::internet::raw::MACAddress;
        return each!(MACAddress, mac);
    }

    if let Some(ip) = matches.subcommand_matches("internet.ip") {
        use fake::faker::internet::raw::{IPv4, IPv6};
        use fake::faker::boolean::en;

        if !ip.args_present() || (ip.is_present("ipv4") && ip.is_present("ipv6")) {
            if en::Boolean(50).fake() {
                return each!(IPv4, ip);
            }
            return each!(IPv6, ip);
        }

        if ip.is_present("ipv4") {
            return each!(IPv4, ip);
        }
        if ip.is_present("ipv6") {
            return each!(IPv6, ip);
        }
    }

    if let Some(useragent) = matches.subcommand_matches("internet.useragent") {
        use fake::faker::internet::raw::UserAgent;
        each!(UserAgent, useragent);
        return;
    }

    if let Some(color) = matches.subcommand_matches("internet.color") {
        use fake::faker::internet::raw::Color;
        return each!(Color, color);
    }
}
