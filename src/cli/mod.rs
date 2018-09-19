use wampire::Client;

mod networkshow;
mod setpassword;

use self::networkshow::network_show;
use self::setpassword::set_password;

const NETWORK_SHOW: &'static str = "network show";
const ACCOUNT_UNLOCK: &'static str = "account unlock";

pub fn listen_and_fire(mut session: Client, interactive: bool, command: String) {
    debug!(
        "listen_and_fire({}, {}, {})",
        "session", interactive, command
    );
    if interactive {
        debug!("Starting interactive mode");
    }

    match command.as_ref() {
        ACCOUNT_UNLOCK => set_password(&mut session),
        NETWORK_SHOW => network_show(&mut session),
        _ => error!("Unknown command, bye"),
    }
    let s_res = session.shutdown().unwrap();
    debug!("shutdown_res: {:?}", s_res);

    //debug!("Result: {}", cli)
}
