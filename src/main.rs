#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
mod args;

use args::{Args, parse_args};

fn main() {
    env_logger::init();
    trace!("START");
    debug!("Parsing args...");
    let a:Args = parse_args();
    debug!("Interactive mode '{}'", a.interactive.to_string());
    debug!("addres '{}'", a.addres);
    debug!("port '{}'", a.port);
    debug!("verify_trust '{}'", a.verify_trust.to_string());
    debug!("command '{}'", a.command);


    trace!("END");
}
