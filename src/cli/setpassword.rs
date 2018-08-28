use std::io;

use eventual::Async;
use wampire::{URI, Value, Client};

pub  fn set_password(session:&mut Client) {
    let mut pass = String::new();
    println!("Password:");
    io::stdin().read_line(&mut pass)
        .expect("Failed to read line");

    
    debug!("Password: {:?}", pass);
    let args = Some(vec![Value::String(pass.trim().to_string())]);
    let uri = URI::new("golem.password.set");
    let result = session.call(uri, args, None).unwrap().await().unwrap();
    debug!("Result: {:?}", result);
}
