use std::fs::File;
use serde::{Serialize,Deserialize};
use serde_yaml::{self};



#[derive(Serialize,Deserialize,Debug)]
pub struct Service {
    pub url: String,
    pub port: u16,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Config {
    pub service: Service,
}

pub fn load_config(cfg_file: String) -> Config {
    let file = File::open(cfg_file).expect("Error opening file");
    let config: Config = serde_yaml::from_reader(file).unwrap();

    config
}