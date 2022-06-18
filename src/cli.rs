use clap::{Arg, Command};

struct GenCommand<'a> {
    lang: Arg<'a>,
    number: Arg<'a>,
}

struct NumberCommand<'a> {
    number: Arg<'a>,
}

impl<'a> GenCommand<'a> {
    fn new() -> Self {
        let lang_arg = Arg::new("lang")
            .short('l')
            .long("lang")
            .help("give lang (ie: fr_FR)")
            .possible_values(&["fr", "fr_FR", "en"]);
        let number_arg = Arg::new("number")
            .short('n')
            .long("number")
            .help("number of values")
            .takes_value(true);
        GenCommand {
            lang: lang_arg,
            number: number_arg,
        }
    }
    fn create(&self, c: Command<'a>) -> Command<'a> {
        c.arg(&self.lang).arg(&self.number)
    }
}

impl<'a> NumberCommand<'a> {
    fn new() -> Self {
        let number_arg = Arg::new("number")
            .short('n')
            .long("number")
            .help("number of values")
            .takes_value(true);
        NumberCommand { number: number_arg }
    }
    fn create(&self, c: Command<'a>) -> Command<'a> {
        c.arg(&self.number)
    }
}

pub fn build_cli(name: &'static str, version: &'static str) -> Command<'static> {
    let gen_command = GenCommand::new();
    let number_command = NumberCommand::new();

    Command::new(name)
        .bin_name(name)
        .version(version)
        .author("Ferry Jérémie ferryjeremie@free.fr")
        .about("give fake datas")
        .arg_required_else_help(true)
        // address
        .subcommand(
            gen_command
                .create(Command::new("address.city").about("give a city name (English only)")),
        )
        .subcommand(gen_command.create(
            Command::new("address.country").about("give a country name and code (English only)"),
        ))
        .subcommand(
            gen_command
                .create(Command::new("address.street").about("give a street name (English only)")),
        )
        // administrative
        .subcommand(
            gen_command.create(
                Command::new("administrative.healthinsurrancecode")
                    .about("give a Health insurrance code (French only)"),
            ),
        )
        // auto
        .subcommand(
            gen_command.create(
                Command::new("auto.licenseplate")
                    .about("give an automotive license plate (French only)"),
            ),
        )
        // barecode
        .subcommand(number_command.create(Command::new("barcode.isbn").about("give an isbn code")))
        // color
        .subcommand(Command::new("color")
            .about("give a fake hexadecimal color")
            .about("give a fake color (hexadécimal, rgb, rgba, hsl and hsla representation)")
            .arg(Arg::new("hexa")
                .short('d')
                .long("hexa")
                .help("give a fake hexadecimal color")
            )
            .arg(Arg::new("rgb")
                .short('r')
                .long("rgb")
                .help("give a fake rgb color")
            )
            .arg(Arg::new("rgba")
                .short('a')
                .long("rgba")
                .help("give a fake rgba color")
            )
            .arg(Arg::new("hsl")
                .short('t')
                .long("hsl")
                .help("give a fake hsl (tsl) color")
            )
            .arg(Arg::new("hsla")
                .short('l')
                .long("hsla")
                .help("give a fake hsla (tsl) color")
            )
        )
        //filesystem
        .subcommand(
            number_command
                .create(Command::new("filesystem.mimetype").about("give a fake mime-type")),
        )
        .subcommand(
            number_command.create(
                Command::new("filesystem.semver")
                    .about("give a fake semver version")
                    .arg(
                        Arg::new("stable")
                            .short('s')
                            .long("stable")
                            .help("give exclusivly stable semver version (X.Y.Z)"),
                    )
                    .arg(
                        Arg::new("unstable")
                            .short('u')
                            .long("unstable")
                            .help("give exclusivly unstable semver version (X-Y-Z-V.W)"),
                    ),
            ),
        )
        // finance
        .subcommand(number_command.create(
            Command::new("finance.bic").about("give a fake BIC (Business Identifier Code)"),
        ))
        // http
        .subcommand(number_command.create(Command::new("http.code").about("give a fake HTTP code")))
        // internet
        .subcommand(
            number_command.create(
                Command::new("internet.ip")
                    .about("give a fake IP (Internet Protocol)")
                    .arg(
                        Arg::new("ipv4")
                            .short('4')
                            .long("ipv4")
                            .help("give exclusivly IPv4"),
                    )
                    .arg(
                        Arg::new("ipv6")
                            .short('6')
                            .long("ipv6")
                            .help("give exclusivly IPv6"),
                    ),
            ),
        )
        .subcommand(
            number_command.create(Command::new("internet.mac").about("give a fake mac adress")),
        )
        .subcommand(gen_command.create(Command::new("internet.mail").about("give a fake mail")))
        .subcommand(
            number_command
                .create(Command::new("internet.useragent").about("give a fake user agent")),
        )
        // lorem
        .subcommand(
            number_command.create(Command::new("lorem.word").about("give a fake word (in Latin)")),
        )
        // people
        .subcommand(
            gen_command.create(
                Command::new("people.name")
                    .about("give a fake name")
                    .arg(
                        Arg::new("title")
                            .short('t')
                            .long("title")
                            .help("give a fake name title"),
                    )
                    .arg(
                        Arg::new("with-title")
                            .short('w')
                            .long("with-title")
                            .help("give a fake name with her title"),
                    )
                    .arg(
                        Arg::new("firstname")
                            .short('f')
                            .long("firstname")
                            .help("give a fake firstname"),
                    )
                    .arg(
                        Arg::new("lastname")
                            .short('z')
                            .long("lastname")
                            .help("give a fake lastname"),
                    ),
            ),
        )
        // time
        .subcommand(number_command.create(Command::new("time.time").about("give a fake time")))
        .subcommand(number_command.create(Command::new("time.date").about("give a fake date")))
}
