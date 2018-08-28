use wampire::{Client};

mod setpassword;
mod networkshow;

use self::setpassword::set_password;
use self::networkshow::network_show;

pub fn listen_and_fire(mut session:Client, interactive:bool, command:String) {
    debug!("listen_and_fire({}, {}, {})", "session", interactive, command);
    if interactive {
        debug!("Starting interactive mode");
    }
    
    set_password(&mut session);
    //network_show(&mut session);
    let s_res = session.shutdown().unwrap();
    debug!("shutdown_res: {:?}", s_res);

    //debug!("Result: {}", cli)
}

