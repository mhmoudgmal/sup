use clap::{App, Arg, ArgMatches};

pub fn match_args<'a>() -> ArgMatches<'a> {
    return App::new("lsup")
        .version("0.1")
        .author("Mahmoud G. <mhmoudgmal.89@gmail.com>")
        .about("TODO://")
        .arg(
            Arg::with_name("stackfile")
                .short("f")
                .long("stackfile")
                .value_name("FILE")
                .help("TODO://")
                .takes_value(true),
        )
        .get_matches();
}
