use chrono::{DateTime, Duration, Utc};
use clap::ArgMatches;
use fake::faker::boolean::raw::Boolean;
use fake::faker::chrono::raw::DateTimeBetween;
use fake::faker::name::raw::{FirstName, LastName, Title};
use fake::locales::EN;
use serde::{Deserialize, Serialize};

use fake::Fake;
use crate::template::{address::Address, automotive::Automotive, job::Job, phone::Phone};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct People {
    pub gender: String,
    pub birthday: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,

    pub location: Address,

    pub automotive: Automotive,

    pub phone: Phone,

    pub job: Job,
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

            phone: Phone::create(arg),

            job: Job::create(arg),
        }
    }
}

create_get_property!(
    People,
    gender: String,
    birthday: String,
    title: String,
    first_name: String,
    last_name: String,
    location: Address
);
