use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub packages: Packages,
    pub cargo_install: Packages,
    pub git: GitConfig,
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
