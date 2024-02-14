use scc_package_search::apis::configuration as cfg;
use scc_package_search::apis::packages_api as pkg;
use scc_package_search::models::Package;
use std::fmt::Error;
use tokio;

#[tokio::main]
async fn main() {
    // TODO:
    // 1. Add clap to parse the command line arguments
    // 2. provide simple search options for either cli or slack bot
    // 2.1. `scc-package-search htop sles` means search for the htop package in the sles products
    for package in search_packages().await.unwrap() {
        println!("{}", package);
    }
}

async fn search_packages() -> Result<Vec<Package>, Error> {
    let config = cfg::Configuration::new();
    let response = pkg::packages_get(
        &config,
        "2605",
        serde_json::Value::String("application/json".to_string()),
        Some(""), // Empty will list all of the packages
    );

    // Search for the 'htop' package
    let packages = response.await.unwrap();

    let found: Vec<Package> = packages.clone().data.into_iter().flatten().collect();

    Ok(found)
}
