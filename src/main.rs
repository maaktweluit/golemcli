extern crate clap;

use clap::{Arg, App};

fn main() {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .args(
            &vec![
                Arg::from_usage("-i --interactive 'Enter interactive mode'").conflicts_with("input"),
                Arg::from_usage("-a --addres=[addres] 'Golem node's RPC address'"),
                Arg::from_usage("-p --port=[port] 'Golem node's RPC port'"),
                Arg::from_usage("-t --verify-trust 'Verify Golem node's certificate'"),
                Arg::from_usage("<command> 'The command to execute'").required(false),
            ]
        )
        .get_matches();

    let interactive = matches.is_present("interactive"); 
    let addres = matches.value_of("addres").unwrap_or("wss://127.0.0.1");
    let port = matches.value_of("port").unwrap_or("61000");
    let verify_trust = matches.is_present("verify-trust");
    let command = matches.value_of("command").unwrap_or("");

    println!("Interactive mode {}", interactive.to_string());
    println!("addres {}", addres);
    println!("port {}", port);
    println!("verify_trust {}", verify_trust.to_string());
    println!("command {}", command);
}
