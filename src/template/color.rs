use random_color::{Luminosity, RandomColor};

use crate::fake::{Fake, Faker};
use clap::ArgMatches;
use fake::locales::EN;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    pub hex: String,
    pub rgb: String,
    pub rgba: String,
    pub hsl: String,
    pub hsla: String,
}

impl Color {
    pub fn create(_arg: &ArgMatches) -> Color {
        let random_color = Faker.fake::<RandomColor>();
        Color {
            hex: random_color.to_hex(),
            rgb: random_color.to_rgb_string(),
            rgba: random_color.to_rgb_string(),
            hsl: random_color.to_hsl_string(),
            hsla: random_color.to_hsla_string(),
        }
    }
}
