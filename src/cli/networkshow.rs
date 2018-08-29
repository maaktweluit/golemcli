use eventual::Async;
use wampire::{URI, Value, Client, Dict};
use wampire::ArgDict;

#[derive(Debug)]
struct NetworkShowNode {
    node_name: String,
    node_id: String,
    version: String,
    pub_ip: String,
    pub_port: u64,
}

impl NetworkShowNode {
    fn from_dict(ref dict :&Dict) -> NetworkShowNode {
        let node_name = dict.get_string("node_name").unwrap().unwrap().to_string();
        let node_id = dict.get_string("node_id").unwrap().unwrap().to_string();
        let version = dict.get_string("version").unwrap().unwrap().to_string();
        let pub_ip = dict.get_string("pub_ip").unwrap().unwrap().to_string();
        let pub_port = dict.get_uint("pub_port").unwrap().unwrap();

        NetworkShowNode{ node_name, node_id, version, pub_ip, pub_port }
    }

    fn get_short_node_id(&self) -> String {
        let first = &self.node_id[..8];
        let len = self.node_id.len();
        let mid = &"..".to_string();
        let last  = &self.node_id[len-8..];
        [first, mid, last].concat()
    }
}

pub fn network_show(session:&mut Client) {
    let uri = URI::new("network.show");

    let result = session.call(uri, None, None).unwrap().await().unwrap();
    let mut table = table!(["ip", "port", "id", "name", "version"]);
    debug!("res0 : {:?}", result);
    debug!("res1 : {:?}", result.0);
    let list_raw = result.0.iter().next();
    debug!("res2 : {:?}", list_raw);
    let ref list = list_raw.unwrap();
    debug!("res3 : {:?}", list);
    if let Value::List(ref l) = *list {
        debug!("l {:?}", l);
        for ref node in l.iter() {
            debug!("row {:?}", node);
            if let Value::Dict(ref d) = *node {
                let row_node = NetworkShowNode::from_dict(&d);

                table.add_row(row!(
                    row_node.pub_ip,
                    row_node.pub_port,
                    row_node.get_short_node_id(),
                    row_node.node_name,
                    row_node.version,
                ));
            }
            else { error!("Failed to read return data, no dict") }
        }        
    }
    else { error!("Failed to read return data, no list") }
    table.printstd();
    //debug!("Result: {:?}", result);
}
