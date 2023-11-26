use std::process::Command;
use std::fs;
use toml;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

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
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    packages: Packages,
    cargo_install: Packages,
}

#[derive(Serialize, Deserialize, Debug)]
struct Packages {
    names: Vec<String> 
}

fn read_config(filename: &str) -> Config {
    let contents = match fs::read_to_string(filename){
        Ok(c)=> c,
        Err(e)=> panic!("error: {}", e),
    };

    toml::from_str(&contents).expect("put a valid config file")
}

fn main() {
    let filename = "devo.toml";
    let config = read_config(&filename);
    let _ = Dnf::install(&config.packages.names);
    let _ = Cargo::install(&config.cargo_install.names);
}
