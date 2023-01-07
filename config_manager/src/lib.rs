#![allow(unused)] //TODO: remove

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io::Read};

static CONFIG_LOCATIONS: [&str; 4] = [
    "./mock_files/sherlock.toml", //TODO: remove. for debugging purposes only
    "./sherlock.toml",
    "~/.sherlock.toml",
    "/opt/sherlock/sherlock.toml",
];

#[derive(Deserialize, Serialize,Debug)]
pub struct ConfigManager {
    engines_location: String,
    storage: StrorageType,
    //TODO: add struct for holding sensitive information like credentials and cryptographic keys.
}

impl ConfigManager {
     
    pub fn init() -> Result<ConfigManager, String> {
        //pick the the first config file that exists from the different options
        let config_file = CONFIG_LOCATIONS
            .iter()
            .find(|file| Path::new(file).exists());


        //read the file and create an instance out of it
        match config_file {
            Some(path) => {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        //try to deserilize the toml file into a ConfigManager instance
                        match toml::from_str(&content) {
                            Ok(instance) => Ok(instance),
                            Err(error) => Err(error.to_string()),
                        }
                    }
                    Err(error) => Err(error.to_string()),
                }
            }
            None => Err("can't find config file".to_owned()),
        }
    }

    pub fn get_engines_location(&self) -> &str{
        return &self.engines_location;
    }
}

#[derive(Deserialize, Serialize,Debug)]
pub enum StrorageType {
    #[serde(rename="remote")]
    Remote,
    #[serde(rename="local")]
    Local { path: String, encrypted: bool },
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_from_toml(){
        let content =  fs::read_to_string(CONFIG_LOCATIONS[0]).unwrap();
        println!("{}",content);
        let manager:ConfigManager = toml::from_str(&content).unwrap();
        println!("{:?}", manager);
    }

}
