#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;

extern crate clap;
extern crate env_logger;
extern crate eventual;
extern crate wampire;

mod args;
mod cli;
mod wamp;

use args::{parse_args, Args};
use cli::listen_and_fire;
use wamp::connect;

fn main() {
    env_logger::init();
    trace!("START");
    debug!("Parsing args...");
    let a: Args = parse_args();
    debug!("Args '{:?}'", a);
    debug!("DONE");
    debug!("Starting session...");
    let session = connect(a.addres, a.port, a.verify_trust);
    debug!("DONE");
    debug!("Starting cli...");
    let cli = listen_and_fire(session, a.interactive, a.command);
    debug!("DONE");

    while false {
        // TODO: Replace false with: cli.busy() {
        trace!("tick tack")
    }

    trace!("END");
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
