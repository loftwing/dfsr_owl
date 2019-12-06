use std::collections::HashMap;
use serde::Deserialize;
use wmi::{
    Variant,
    COMLibrary,
    WMIConnection
};

#[derive(Deserialize, Debug)]
struct DfsrReplicationGroupConfig {
    
}

mod queries {
    pub const RGROUP_QUERY: &str = "SELECT ReplicationGroupName FROM DfsrReplicationGroupConfig"; 
}

fn main() {
    let com_con = COMLibrary::new().unwrap();

    match WMIConnection::with_namespace_path("ROOT\\MicrosoftDfs", com_con.into()) {
        Err(e) => {
            dbg!(e);
            panic!();
        },
        Ok(r) => {
            let results: Vec<HashMap<String, Variant>> = r.raw_query(queries::RGROUP_QUERY).unwrap();

            for os in results {
                println!("{:#?}", os);
            }
        }
    }
    
}

