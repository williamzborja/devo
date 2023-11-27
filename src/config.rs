use std::{path::PathBuf, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub packages: Packages,
    pub cargo_install: Option<Packages>,
    pub git:GitConfig,
    pub shell_package: Vec<ShellPackage>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packages {
    pub names: Vec<String> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitConfig {
    pub name: String,
    pub email: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellPackage {
    pub name: String,
    pub url: String
}

pub fn read_config() -> Config {
    let home_dir = std::env::var("HOME").expect("Home variable is required");
    let config_file: PathBuf = [home_dir.as_str(), r".config/devo/devo.toml"].iter().collect();
    let contents =fs::read_to_string(config_file).unwrap(); // TODO: implement error management
    toml::from_str(&contents).expect("put a valid config file")
}

