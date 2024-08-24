use clap::ArgMatches;
use fake::faker::http::raw::{RfcStatusCode, ValidStatusCode};
use fake::locales::EN;
use fake::Fake;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Http {
    pub rfc_status_code: String,
    pub valid_status_code: String,
}

impl Http {
    pub fn create(_arg: &ArgMatches) -> Http {
        Http {
            rfc_status_code: RfcStatusCode(EN).fake::<String>(),
            valid_status_code: ValidStatusCode(EN).fake::<String>(),
        }
    }
}

create_get_property!(
    Http,
    rfc_status_code: String,
    valid_status_code: String
);
