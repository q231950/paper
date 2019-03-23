mod paper;
use crate::paper::configuration::Configuration;
#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};

fn main() {
    let app = App::new("paper")
        .version(crate_version!())
        .author(crate_authors!())
        .about("List paper crashes");
    let matches = matches_for_app(app);

    let mut configuration = Configuration::new();

    println!(
        "Debugging mode is: {}",
        if matches.is_present("debug") {
            "ON"
        } else {
            "OFF"
        }
    );

    if let Some(config) = matches.value_of("config") {
        println!("Using config: {}", config);
    }

    if let Some(access_token) = matches.value_of("access token") {
        let token = matches.value_of("access token").unwrap();
        configuration = configuration.with_access_token(token);
        println!("Using access token: {}", configuration.access_token);
    }
}

fn matches_for_app<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    app.arg(
        Arg::with_name("debug")
            .help("turn on debugging information")
            .long("debug")
            .short("d"),
    )
    .args(&[
        Arg::with_name("config")
            .help("sets a config file to use")
            .takes_value(true)
            .short("c")
            .long("config"),
        Arg::with_name("access token")
            .help("an access token to use")
            .takes_value(true)
            .short("a")
            .long("access-token")
            .required(true),
    ])
    .get_matches()
}
