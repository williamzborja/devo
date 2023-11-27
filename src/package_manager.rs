use std::process::Command;
use tempfile::Builder;
use std::fs::File;
use std::io::{copy, Write, Read};

use crate::config::{Config, ShellPackage};

pub trait PackageManager{
    fn install(pgs:  &Vec<String>) -> Result<(), String>;
}
pub struct Dnf;
pub struct Cargo;


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


pub async fn shell_packages(pkgs: &Vec<ShellPackage>) {
    for pack in pkgs.iter() {

        println!("name: {}", pack.name);

        let tmp_dir = Builder::new().prefix(&pack.name).tempdir().expect("file in tmp folder");
        let response = reqwest::get(&pack.url).await.unwrap();

        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");


        println!("file to download: '{}'", fname);

        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);

        let mut dest = File::create(&fname).unwrap();
        let content =  response.text().await.unwrap();
        let _ = copy(&mut content.as_bytes(), &mut dest);

        let mut child = Command::new("sh")
            .arg(&fname)
            .spawn()
            .expect("put valid packages names");
        let output = child.wait();

        println!("{:?}", output);
    } 

}

pub async fn install_packages(config: &Config) {
    let _ = Dnf::install(&config.packages.names);
    if let Some(cargo_pkgs) = &config.cargo_install {
        let _ = Cargo::install(&cargo_pkgs.names);
    }

    shell_packages(&config.shell_package).await;
}
