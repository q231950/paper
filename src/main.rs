extern crate paper;

use paper::configuration::Configuration;

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};

#[tokio::main]
async fn main() {
    let app = app();
    let matches = matches_for_app(app);

    let mut configuration = Configuration {
        username: None,
        password: None,
    };

    if let Some(config) = matches.value_of("config") {
        println!("Using config: {}", config);
    }

    if let Some(username) = matches.value_of("username") {
        configuration = configuration.with_username(Some(username.to_string()));
    }

    if let Some(password) = matches.value_of("password") {
        configuration = configuration.with_password(Some(password.to_string()));
    }

    let paper = paper::Paper::with_config(configuration);
    paper.initiate_commands().await;
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("paper")
        .version(crate_version!())
        .author(crate_authors!())
        .about("List paper crashes")
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
        Arg::with_name("username")
            .help("the username to use")
            .takes_value(true)
            .short("u")
            .long("username")
            .required(true),
        Arg::with_name("password")
            .help("the password that matches the username")
            .takes_value(true)
            .short("p")
            .long("password")
            .required(true),
    ])
    .get_matches()
}
