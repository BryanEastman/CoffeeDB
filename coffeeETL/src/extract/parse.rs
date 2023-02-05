use crate::config::{Conf};
use std::fs::File;
use glob::{glob};
use std::io::BufReader;

pub fn file_loop() -> Result<(), std::io::Error> {
    let cred_file = File::open("../config.json")?;
    let reader = BufReader::new(cred_file);
    let creds: Conf = serde_json::from_reader(reader)?;

    let raw: String = creds.data_source;
    let finished: String = creds.data_dump;

    let search_string: String = format!("{}/*.json", raw);

    for entry in glob(&search_string).expect("files not found") {
        println!("{}", entry.expect("error parsing filename").display());
    }

    Ok(())
}