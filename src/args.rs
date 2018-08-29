use clap::{Arg, App, ArgMatches};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

const DEFAULT_ADDRES: &'static str = "127.0.0.1";
const DEFAULT_PORT: &'static str = "61000";

#[derive(Debug)]
pub struct Args {
    pub interactive: bool,
    pub addres: String,
    pub port: String,
    pub verify_trust: bool,
    pub command: String
}

pub fn parse_args() -> Args {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .args(&vec![
            Arg::from_usage("-i --interactive 'Enter interactive mode'").conflicts_with("input"),
            Arg::from_usage("-a --addres=[addres] 'Golem node's RPC address'"),
            Arg::from_usage("-p --port=[port] 'Golem node's RPC port'"),
            Arg::from_usage("-t --verify-trust 'Verify Golem node's certificate'"),
            Arg::from_usage("<command> 'The command to execute'").required(false),
        ])
        .get_matches();

    return matches_to_args(matches);
}

fn matches_to_args(matches:ArgMatches) -> Args {
    let interactive = matches.is_present("interactive"); 
    let addres = matches.value_of("addres").unwrap_or(DEFAULT_ADDRES).to_string();
    let port = matches.value_of("port").unwrap_or(DEFAULT_PORT).to_string();
    let verify_trust = matches.is_present("verify-trust");
    let command = matches.value_of("command").unwrap_or("").to_string();

    return Args{interactive, addres, port, verify_trust, command};
}
