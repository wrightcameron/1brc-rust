mod weather;

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
    // If file path isn't passed in, use default.
    let file_path = if let Some(file_path) = cli.file_path.as_deref() {
        file_path
    } else {
        "data/samples/measurements-1.txt"
    };
    let weather_stations = weather::scan_weather_stations(&file_path);
    weather::print_stations(&weather_stations);
}
