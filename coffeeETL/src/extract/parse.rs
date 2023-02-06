use crate::config::{Conf};
use std::fs::File;
use glob::{glob};
use std::io::BufReader;
use std::path::PathBuf;

use crate::extract::structures;

pub fn parse_json(log_path: &PathBuf) -> Result<(), std::io::Error> {
    let f = File::open(log_path).expect("failed at file open");
    let reader = BufReader::new(f);
    let log: structures::RawLog = serde_json::from_reader(reader).unwrap();

    println!("please god let this work {:#?}", log.roastingnotes);
    
    
    // let roast: structures:: = 

    // println!("{}", flav.acidity);
    Ok(())
}

pub fn file_loop() -> Result<(), std::io::Error> {
    let cred_file = File::open("../config.json")?;
    let reader = BufReader::new(cred_file);
    let creds: Conf = serde_json::from_reader(reader)?;

    let raw: String = creds.data_source;
    let finished: String = creds.data_dump;

    let search_string: String = format!("{}/*.json", raw);

    for entry in glob(&search_string).expect("files not found") {
        println!("file:{}", &entry.as_ref().expect("").display());
        parse_json(&entry.expect("error parsing file"));
    }

    Ok(())
}