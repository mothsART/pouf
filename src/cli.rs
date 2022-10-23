use clap::builder::PossibleValuesParser;
use clap::{Arg, ArgAction, Command};

pub fn build_cli(name: &'static str, version: &'static str) -> Command {
    let lang_arg = Arg::new("lang")
        .short('l')
        .long("lang")
        .help("give lang (ie: fr_FR)")
        .value_parser(PossibleValuesParser::new(["fr", "fr_FR", "en"]));

    let number_arg = Arg::new("number")
        .short('n')
        .long("number")
        .action(ArgAction::Set)
        .help("number of values");

    Command::new(name)
        .bin_name(name)
        .version(version)
        .author("Ferry Jérémie ferryjeremie@free.fr")
        .about("give fake datas")
        .arg_required_else_help(true)
        // address
        .subcommand(
            Command::new("address.city")
                .about("give a city name (English only)")
                .args([&number_arg]),
        )
        .subcommand(
            Command::new("address.country")
                .about("give a country name and code (English only)")
                .args([&number_arg]),
        )
        .subcommand(
            Command::new("address.street")
                .about("give a street name (English only)")
                .args([&number_arg]),
        )
        // administrative
        .subcommand(
            Command::new("administrative.healthinsurrancecode")
                .about("give a Health insurrance code (French only)")
                .args([&number_arg]),
        )
        // auto
        .subcommand(
            Command::new("auto.licenseplate")
                .about("give an automotive license plate (French only)")
                .args([&number_arg]),
        )
        // barecode
        .subcommand(
            Command::new("barcode.isbn")
                .about("give an isbn code")
                .args([&number_arg]),
        )
        // color
        .subcommand(
            Command::new("color")
                .about("give a fake hexadecimal color")
                .about("give a fake color (hexadécimal, rgb, rgba, hsl and hsla representation)")
                .args([
                    Arg::new("hexa")
                        .short('d')
                        .long("hexa")
                        .help("give a fake hexadecimal color"),
                    Arg::new("rgb")
                        .short('r')
                        .long("rgb")
                        .help("give a fake rgb color"),
                    Arg::new("rgba")
                        .short('a')
                        .long("rgba")
                        .help("give a fake rgba color"),
                    Arg::new("hsl")
                        .short('t')
                        .long("hsl")
                        .help("give a fake hsl (tsl) color"),
                    Arg::new("hsla")
                        .short('l')
                        .long("hsla")
                        .help("give a fake hsla (tsl) color"),
                ]),
        )
        //filesystem
        .subcommand(
            Command::new("filesystem.mimetype")
                .about("give a fake mime-type")
                .args([&number_arg]),
        )
        .subcommand(
            Command::new("filesystem.semver")
                .about("give a fake semver version")
                .args([
                    &Arg::new("stable")
                        .short('s')
                        .long("stable")
                        .action(clap::ArgAction::SetTrue)
                        .help("give exclusivly stable semver version (X.Y.Z)"),
                    &Arg::new("unstable")
                        .short('u')
                        .long("unstable")
                        .action(clap::ArgAction::SetTrue)
                        .help("give exclusivly unstable semver version (X-Y-Z-V.W)"),
                    &number_arg,
                ]),
        )
        // finance
        .subcommand(
            Command::new("finance.bic")
                .about("give a fake BIC (Business Identifier Code)")
                .args([&number_arg]),
        )
        // http
        .subcommand(
            Command::new("http.code")
                .about("give a fake HTTP code")
                .args([&lang_arg, &number_arg]),
        )
        // internet
        .subcommand(
            Command::new("internet.ip")
                .about("give a fake IP (Internet Protocol)")
                .args([
                    &Arg::new("ipv4")
                        .short('4')
                        .long("ipv4")
                        .action(clap::ArgAction::SetTrue)
                        .help("give exclusivly IPv4"),
                    &Arg::new("ipv6")
                        .short('6')
                        .long("ipv6")
                        .action(clap::ArgAction::SetTrue)
                        .help("give exclusivly IPv6"),
                    &number_arg,
                ]),
        )
        .subcommand(
            Command::new("internet.mac")
                .about("give a fake mac adress")
                .args([&number_arg]),
        )
        .subcommand(
            Command::new("internet.mail")
                .about("give a fake mail")
                .args([&number_arg]),
        )
        .subcommand(
            Command::new("internet.useragent")
                .about("give a fake user agent")
                .args([&number_arg]),
        )
        // lorem
        .subcommand(
            Command::new("lorem.word")
                .about("give a fake word (in Latin)")
                .args([&number_arg]),
        )
        // people
        .subcommand(
            Command::new("people.name").about("give a fake name").args([
                &Arg::new("title")
                    .short('t')
                    .long("title")
                    .help("give a fake name title"),
                &Arg::new("with-title")
                    .short('w')
                    .long("with-title")
                    .help("give a fake name with her title"),
                &Arg::new("firstname")
                    .short('f')
                    .long("firstname")
                    .help("give a fake firstname"),
                &Arg::new("lastname")
                    .short('z')
                    .long("lastname")
                    .help("give a fake lastname"),
                &lang_arg,
                &number_arg,
            ]),
        )
        // time
        .subcommand(
            Command::new("time.time")
                .about("give a fake time")
                .args([&number_arg]),
        )
        .subcommand(
            Command::new("time.date")
                .about("give a fake date")
                .args([&number_arg]),
        )
}
