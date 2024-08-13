use fake::Fake;
use clap::ArgMatches;
use fake::faker::job::raw::{Field, Position, Seniority, Title};
use serde::{Deserialize, Serialize};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub seniority: String,
    pub field: String,
    pub position: String,
    pub title: String,
}

impl Job {
    pub fn create(arg: &ArgMatches) -> Job {
        Job {
            seniority: lang_return!(Seniority, arg),
            field: lang_return!(Field, arg),
            position: lang_return!(Position, arg),
            title: lang_return!(Title, arg),
        }
    }
}

create_get_property!(
    Job,
    seniority: String,
    field: String,
    position: String,
    title: String
);
