use eventual::Async;
use wampire::{URI, Value, Client};
use wampire::ArgDict;

pub fn listen_and_fire(mut session:Client, interactive:bool, command:String) {
    debug!("listen_and_fire({}, {}, {})", "session", interactive, command);
    if interactive {
        debug!("Starting interactive mode");
    }
    
    //set_password()
    network_show(&mut session);
    let s_res = session.shutdown().unwrap();
    debug!("shutdown_res: {:?}", s_res);

    //debug!("Result: {}", cli)
}

fn set_passwor(session:&mut Client) {
    let uri = URI::new("golem.password.set");

    let args = Some(vec![Value::String("a".to_string())]);
    
    let result = session.call(uri, args, None).unwrap();
    debug!("Result: {:?}", result);
}

/*
#[derive(Serialize, Deserialize, Debug)]
struct NetworkShowNode {
    node_name: String,
    node_id: String,
    version: String,
    pub_ip: String,
    pub_port: u32,
}
*/
fn network_show(session:&mut Client) {
    let uri = URI::new("network.show");

    let result = session.call(uri, None, None).unwrap().await().unwrap();
    let mut table = table!(["ip", "port", "id", "name", "version"]);
    debug!("res0 : {:?}", result);
    debug!("res1 : {:?}", result.0);
    let list_raw = result.0.iter().next();
    debug!("res2 : {:?}", list_raw);
    let ref list = list_raw.unwrap();
    debug!("res3 : {:?}", list);
    match *list {
        Value::List(ref l) => {
            debug!("l {:?}", l);
            for ref node in l.iter() {
                debug!("row {:?}", node);
                match *node {
                    Value::Dict(ref d) => {
                        debug!("get {:?}", d.get_string("pub_ip").unwrap());
                        let node_id = d.get_string("node_id").unwrap().unwrap();
                        let mut node_id_short: String = String::new();
                        node_id_short.push_str(&node_id[..8]);
                        node_id_short.push_str(&"...".to_string());
                        node_id_short.push_str(&node_id[node_id.len()-8..]);
                        table.add_row(row!(
                            d.get_string("pub_ip").unwrap().unwrap(),
                            d.get_uint("pub_port").unwrap().unwrap().to_string(),
                            node_id_short,
                            d.get_string("node_name").unwrap().unwrap(),
                            d.get_string("version").unwrap().unwrap(),
                        ));
                   }
                   _ => {}
                }
            }
        }        
        _ => {}
    }
    table.printstd();
    //debug!("Result: {:?}", result);
}
