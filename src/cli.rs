use clap::{App, Arg};

const VERSION: &'static str = "0.1.0";

pub fn build_cli() -> App<'static> {
    App::new("pouf")
    .bin_name("pouf")
    .version(VERSION)
    .author("Ferry Jérémie ferryjeremie@free.fr")
    .about("give fake datas")
    .subcommand(App::new("lorem.word")
        .about("give a fake word")
    )
    .subcommand(App::new("internet.mail")
        .about("give a fake mail")
        .arg(Arg::new("lang")
            .short('l')
            .about("give lang (ie: fr_FR)")
            .takes_value(true)
        )
        .arg(Arg::new("number")
            .short('n')
            .about("number of values")
            .takes_value(true)
        )
    )
    .subcommand(App::new("internet.ip")
        .about("give a fake IP (Internet Protocol)")
        .arg(Arg::new("ipv4")
            .short('4')
            .long("ipv4")
            .about("give exclusivly IPv4")
        )
        .arg(Arg::new("ipv6")
            .short('6')
            .long("ipv6")
            .about("give exclusivly IPv6")
        )
    )
    .subcommand(App::new("internet.mac")
        .about("give a fake mac adress")
    )
    .subcommand(App::new("internet.useragent")
        .about("give a fake user agent")
    )
    .subcommand(App::new("http.code")
        .about("give a fake HTTP code")
    )
    .subcommand(App::new("time.time")
        .about("give a fake time")
    )
    .subcommand(App::new("time.date")
        .about("give a fake date")
    )
    .subcommand(App::new("filesystem.mimetype")
        .about("give a fake mime-type")
    )
    .subcommand(App::new("administrative.healthinsurrancecode")
        .about("give a Health insurrance code")
    )
    .subcommand(App::new("finance.bic")
        .about("give a fake BIC (Business Identifier Code)")
    )
    .subcommand(App::new("auto.licenseplate")
        .about("give a automotive license plate")
    )
}
