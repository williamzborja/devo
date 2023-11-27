mod config;
mod git;
mod package_manager;

use git::git_config_global;
use config::read_config;
use package_manager::install_packages;
use tokio;

#[tokio::main]
async fn  main() {
    let config = read_config();

    git_config_global(&config.git);
    install_packages(&config).await;
}
