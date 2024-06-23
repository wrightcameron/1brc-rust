mod weather;

use std::fs;
use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional file_path to operate on
    file_path: Option<String>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let file_path = if let Some(file_path) = cli.file_path.as_deref() {
        file_path
    } else {
        "data/samples/measurements-1.txt"
    };
    log::debug!("Scanning file: {file_path}");
    let input = fs::read_to_string(file_path).expect("Data file doesn't exist!");
    let weather_stations = weather::scan_weather_stations(&input);
    weather::print_stations(&weather_stations);
}