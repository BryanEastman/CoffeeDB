use std::env;
use std::io;
use std::io::{BufReader};
use std::fs::File;

use crate::config::{init, Conf};

fn get_password() -> String {
    println!("enter password: ");
    let mut pw = String::new();
    io::stdin()
        .read_line(&mut pw)
        .expect("failed to read line");
    pw
}

pub fn connection() -> std::io::Result<()> {
    let cred_file = File::open("../config.json")?;
    let reader = BufReader::new(cred_file);
    let creds: Conf = serde_json::from_reader(reader)?;

    let password = get_password();

    let con_str = format!("postgresql://{}:{}@{}/{}", creds.user, password, creds.host, creds.database);
    println!("connected to database {} with user {}", creds.database, creds.user);
    Ok(())
}
