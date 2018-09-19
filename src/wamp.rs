use wampire::{Client, Connection};

pub fn connect(address: String, port: String, verify_trust: bool) -> Client {
    let uri = format!("wss://{}:{}/ws", address, port);
    let connection = Connection::new(&uri, "golem");
    debug!("Connecting");
    let client = connection.connect().unwrap();
    debug!("Connnnected");
    return client;
}
