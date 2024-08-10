use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::internet::raw::*;
use fake::locales::EN;
use serde::{Deserialize, Serialize};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Internet {
    pub free_email_provider: String,
    pub domain_suffix: String,
    pub free_email: String,
    pub safe_email: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub ipv4: String,
    pub ipv6: String,
    pub mac_address: String,
    pub useragent: String,
}

impl Internet {
    pub fn create(arg: &ArgMatches) -> Internet {
        Internet {
            free_email_provider: lang_return!(FreeEmailProvider, arg),
            domain_suffix: lang_return!(DomainSuffix, arg),
            free_email: lang_return!(FreeEmail, arg),
            safe_email: lang_return!(SafeEmail, arg),
            username: lang_return!(Username, arg),
            password: Password(EN, 8..20).fake(),
            ip: lang_return!(IP, arg),
            ipv4: lang_return!(IPv4, arg),
            ipv6: lang_return!(IPv6, arg),
            mac_address: lang_return!(MACAddress, arg),
            useragent: lang_return!(UserAgent, arg),
        }
    }
}

create_get_property!(
    Internet,
    free_email_provider: String,
    domain_suffix: String,
    free_email: String,
    safe_email: String,
    username: String,
    password: String,
    ip: String,
    ipv4: String,
    ipv6: String,
    mac_address: String,
    useragent: String
);
