use std::fs::write;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    pub user: String,
    pub host: String,
    pub database: String,
    pub data_source: String,
    pub data_dump: String,
}

pub fn init(args: &Vec<String>) -> std::io::Result<()> {
    let user = Conf { 
        user: args[2].to_owned(),
        host: args[3].to_owned(), 
        database: args[4].to_owned(),
        data_source: args[5].to_owned(),
        data_dump: args[6].to_owned(),
    };

    let serialized = serde_json::to_string(&user)?;

    println!("serialized = {}", serialized);

    write("../config.json", serialized)?;
    Ok(())
}