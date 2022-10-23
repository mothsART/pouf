use clap::ArgMatches;
use fake::locales::EN;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(c) = matches.subcommand_matches("color") {
        use fake::faker::color::raw::{Color, HexColor, HslColor, HslaColor, RgbColor, RgbaColor};

        if !c.args_present() {
            let all_color: String = Color(EN).fake();
            println!("{}", all_color);
            return;
        }

        if c.contains_id("hexa") {
            let hex_val: String = HexColor(EN).fake();
            println!("{}", hex_val);
        }
        if c.contains_id("rgb") {
            let rgb_val: String = RgbColor(EN).fake();
            println!("{}", rgb_val);
        }
        if c.contains_id("rgba") {
            let rgba_val: String = RgbaColor(EN).fake();
            println!("{}", rgba_val);
        }
        if c.contains_id("hsl") {
            let hsl_val: String = HslColor(EN).fake();
            println!("{}", hsl_val);
        }
        if c.contains_id("hsla") {
            let hsla_val: String = HslaColor(EN).fake();
            println!("{}", hsla_val);
        }
    }
}
