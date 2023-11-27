mod config;
mod git;

use std::{process::Command, path::PathBuf};
use std::fs;
use git::git_config_global;
use toml;
use config::Config;

trait PackageManager{
    fn install(pgs:  &Vec<String>) -> Result<(), String>;
}
struct Dnf;
struct Cargo;


impl PackageManager for Dnf {
   fn install(pgs: &Vec<String>) -> Result<(), String> {
        let mut child = Command::new("sudo")
            .arg("dnf").arg("install")
            .args(pgs)
            .arg("-y")
            .spawn()
            .expect("put valid packages names");
        let output = child.wait();

        println!("{:?}", output);
        Ok(())
   }
}

impl PackageManager for Cargo {
   fn install(pgs: &Vec<String>) -> Result<(), String> {
        let mut child = Command::new("cargo")
            .arg("install")
            .args(pgs)
            .spawn()
            .expect("put valid packages names");
        let output = child.wait();

        println!("{:?}", output);
        Ok(())
   }
}

fn read_config() -> Config {
    let home_dir = std::env::var("HOME").expect("Home variable is required");
    let config_file: PathBuf = [home_dir.as_str(), r".config/devo/devo.toml"].iter().collect();
    let contents = match fs::read_to_string(config_file){
        Ok(c)=> c,
        Err(e)=> panic!("error: {}", e),
    };

    toml::from_str(&contents).expect("put a valid config file")
}

fn main() {
    let config = read_config();

    git_config_global(&config.git);
    let _ = Dnf::install(&config.packages.names);
    let _ = Cargo::install(&config.cargo_install.names);
}
