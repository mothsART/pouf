use clap::ArgMatches;
use fake::locales::Data;
use fake::{Dummy, Fake};
use rand::Rng;

struct UnidecodeFreeEmail<L>(L);

impl<L: Data + Copy> Dummy<UnidecodeFreeEmail<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &UnidecodeFreeEmail<L>, rng: &mut R) -> Self {
        use fake::faker::internet::raw::FreeEmail;
        use unidecode::unidecode;
        let mail: String = FreeEmail(c.0).fake_with_rng(rng);
        unidecode(&mail)
    }
}

pub fn run(matches: &ArgMatches) {
    if let Some(mail) = matches.subcommand_matches("internet.mail") {
        return each!(UnidecodeFreeEmail, mail);
    }

    if let Some(mac) = matches.subcommand_matches("internet.mac") {
        use fake::faker::internet::raw::MACAddress;
        return each!(MACAddress, mac);
    }

    if let Some(ip) = matches.subcommand_matches("internet.ip") {
        use fake::faker::boolean::en;
        use fake::faker::internet::raw::{IPv4, IPv6};

        if let (Some(ipv4), Some(ipv6)) = (ip.get_one::<bool>("ipv4"), ip.get_one::<bool>("ipv6")) {
            if *ipv4 && *ipv6 || !*ipv4 && !*ipv6 {
                match ip.get_one::<String>("number").map(|s| s.as_str()) {
                    Some(number) => {
                        let n: i32 = number.parse().unwrap_or(1);
                        for _i in 0..n {
                            if en::Boolean(50).fake() {
                                lang!(IPv4, ip);
                                continue;
                            }
                            lang!(IPv6, ip);
                        }
                        return;
                    }
                    None => {
                        if en::Boolean(50).fake() {
                            return lang!(IPv4, ip);
                        }
                        return lang!(IPv6, ip);
                    }
                }
            }

            if *ipv4 {
                return each!(IPv4, ip);
            }
            if *ipv6 {
                return each!(IPv6, ip);
            }
        }
    }

    if let Some(useragent) = matches.subcommand_matches("internet.useragent") {
        use fake::faker::internet::raw::UserAgent;
        each!(UserAgent, useragent);
    }
}
