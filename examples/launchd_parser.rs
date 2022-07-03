use std::{fs::OpenOptions, io::Write};

use log::LevelFilter;
use macos_launchd::launchd::LaunchdPlist;
use simplelog::{Config, SimpleLogger};

fn main() {
    println!("Starting LaunchD parser...");
    SimpleLogger::init(LevelFilter::Warn, Config::default())
        .expect("Failed to initialize simple logger");
    let mut daemon_results =
        LaunchdPlist::get_launchd_daemons().expect("Failed to get launchd daemons");
    let mut agent_results =
        LaunchdPlist::get_launchd_agents().expect("Failed to get launchd agents");

    daemon_results.append(&mut agent_results);
    parse_data(daemon_results);
}

fn parse_data(results: Vec<LaunchdPlist>) {
    let mut json_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("output.json")
        .expect("Failed to create output.json");

    let serde_data =
        serde_json::to_string(&results).expect("Failed to convert data to json string using serde");

    json_file
        .write_all(serde_data.as_bytes())
        .expect("Failed to write data json");
    println!("\nFinished parsing LoginItems data. Saved results to: output.json");
}
