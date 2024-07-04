use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Temp = f32;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct WeatherStation {
    min: Temp,
    max: Temp,
    mean: Temp,
    sum: Temp,
    count: u32,
}

impl WeatherStation {
    pub fn new(temp: Temp) -> WeatherStation {
        WeatherStation {
            min: temp,
            max: temp,
            mean: temp,
            sum: temp,
            count: 1,
        }
    }

    pub fn add_temp(&mut self, temp: Temp) {
        self.min = f32::min(self.min, temp);
        self.max = f32::max(self.max, temp);
        self.sum += temp;
        self.count += 1;
    }

    fn get_mean(&self) -> Temp {
        self.sum / self.count as f32
    }
}

impl Default for WeatherStation {
    fn default() -> WeatherStation {
        WeatherStation {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            sum: 0.0,
            count: 0
        }
    }
}

pub fn scan_weather_stations(file_path: &str) -> HashMap<String, WeatherStation> {
    let mut weather_stations: HashMap<String, WeatherStation> = HashMap::new();
    // let weather_stations: Vec<WeatherStation> = Vec::new();
    log::debug!("Scanning file: {file_path}");
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let split = line.split_once(";");
            // Skip if line is improperly formated that split finds nothing.
            let (station_name, temp) = match split {
                Some( (station_name, temp) ) => (station_name, temp),
                None => {
                    log::debug!("Skipping line, improper line format ({line}).");
                    continue;
                }
            };
            // Add data to HashMap
            let temp = temp.parse::<Temp>().unwrap();
            if !weather_stations.contains_key(station_name) {
                weather_stations.insert(station_name.to_string(), WeatherStation::new(temp) );
            } else {
                weather_stations.get_mut(station_name).unwrap().add_temp(temp);
            }
        }
    }
    weather_stations
}

pub fn print_stations(stations: &HashMap<String, WeatherStation>){
    let mut stations_vec = stations.into_iter().collect::<Vec<_>>();
    stations_vec.sort_unstable_by_key(|p| p.0);
    let mut is_first = true;
    print!("{{");
    for (station_name, data) in stations_vec {
            if is_first {
                print!("{station_name}={:.1}/{:.1}/{:.1}", data.min, data.get_mean(), data.max);
                is_first = false;
            } else {
                print!(", {station_name}={:.1}/{:.1}/{:.1}", data.min, data.get_mean(), data.max);
            }
        }
    print!("}}\n");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}