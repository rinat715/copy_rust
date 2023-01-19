use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

fn open_file(name: &str, content: &mut String) -> Result<(), std::io::Error> {
    let mut f = File::open(name)?;
    f.read_to_string(content)?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    customer: Customer,
    vendor: Vendor
}


#[derive(Deserialize, Serialize, Debug)]
struct Vendor {
    url: String,
    login: String,
    password: String,
}


#[derive(Deserialize, Serialize, Debug)]
struct Customer {
    url: String,
    login: String,
    password: String,
}

impl Config {
    fn new(content: &str) -> Result<Config, toml::de::Error> {
        Ok(toml::from_str(content)?)
    }


    pub fn from_file(name: &str) -> Config {
        let mut content = String::new();

        match open_file(name, &mut content) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound =>  panic!("File: {:?} not found Err: {:?}", name, error),
                other_error => {
                    panic!("Problem opening the file: {:?} Err: {:?}", name, other_error);
                }
            },
        };


        match Config::new(&content) {
            Ok(config) => config,
            Err(error) => {
                panic!("Problem parse the file: {:?} Err: {:?}", name, error);
            }
        }
    }
}


