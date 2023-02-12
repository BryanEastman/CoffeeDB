use std::env;

use crate::load::connect::connection;
use crate::config::{init};
use crate::extract::parse::file_loop;

pub mod extract;
// pub mod transform;
pub mod load;
pub mod config;


pub fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    match args[1].trim() {
        "init" => init(&args),
        "load" => connection(),
        "transf" => file_loop(),
        _ => Ok(println!("invalid argument")),
    }.unwrap();

    Ok(())
}