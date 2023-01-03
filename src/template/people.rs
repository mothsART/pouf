use chrono::{DateTime, Duration, Utc};
use clap::ArgMatches;
use fake::faker::boolean::raw::Boolean;
use fake::faker::chrono::raw::DateTimeBetween;
use fake::faker::name::raw::{FirstName, LastName, Title};
use fake::faker::phone_number::raw::{CellNumber, PhoneNumber};
use fake::locales::EN;
use serde::{Deserialize, Serialize};

use crate::fake::Fake;
use crate::lang_env;
use crate::template::address::Address;
use crate::template::automotive::Automotive;

#[derive(Serialize, Deserialize, Debug)]
pub struct People {
    pub gender: String,
    pub birthday: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,

    pub location: Address,

    pub automotive: Automotive,
}

impl People {
    pub fn create(arg: &ArgMatches) -> People {
        let b = Boolean(EN, 50).fake::<bool>();
        let mut gender = "female";
        if b {
            gender = "male";
        }
        let years = 365 * 120;
        let first_date = Utc::now() - Duration::days(years);
        let last_date = Utc::now();
        let birthday_datetime: DateTime<Utc> = DateTimeBetween(EN, first_date, last_date).fake();
        People {
            gender: gender.to_string(),
            birthday: birthday_datetime.format("%m-%d-%Y").to_string(),
            title: lang_return!(Title, arg),
            first_name: lang_return!(FirstName, arg),
            last_name: lang_return!(LastName, arg),

            location: Address::create(arg),

            automotive: Automotive::create(arg),
        }
    }
}
