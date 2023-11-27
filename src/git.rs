use std::process::Command;

use crate::config::GitConfig;

pub fn git_config_global(conf: &GitConfig) {
    let _ = Command::new("git")
        .args(["config", "--global", "user.name", &conf.name])
        .spawn().expect("error in git config");
    let _ = Command::new("git")
        .args(["config", "--global", "user.email", &conf.email])
        .spawn().expect("error in git config");


}
