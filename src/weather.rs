use std::collections::BTreeMap;

type Temp = f32;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct WeatherStation {
    station_name: String,
    min: Temp,
    max: Temp,
    mean: Temp,
    sum: Temp,
    count: u32,
}

impl WeatherStation {
    pub fn new(station_name: &str, temp: Temp) -> WeatherStation {
        WeatherStation {
            station_name: station_name.to_string(),
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
        self.mean = (self.mean + temp) / 2_f32;
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
            station_name: String::new(),
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            sum: 0.0,
            count: 0
        }
    }
}

impl std::fmt::Display for WeatherStation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}={:.1}/{:.1}/{:.1}", self.station_name, self.min, self.get_mean(), self.max)
    }
}

pub fn scan_weather_stations(input: &String) -> BTreeMap<String, WeatherStation> {
    let mut weather_stations: BTreeMap<String, WeatherStation> = BTreeMap::new();
    // let weather_stations: Vec<WeatherStation> = Vec::new();
    for line in input.lines().into_iter() {
        let (station_name, temp) = line.split_once(";").unwrap();
        let temp = temp.parse::<Temp>().unwrap();
        if !weather_stations.contains_key(station_name) {
            weather_stations.insert(station_name.to_string(), WeatherStation::new(station_name, temp) );
        } else {
            weather_stations.get_mut(station_name).unwrap().add_temp(temp);
        }
    }
    weather_stations
}

pub fn print_stations(stations: &BTreeMap<String, WeatherStation>){
    print!("{{");
    let mut is_first = true;
    for station in stations.values() {
        if is_first {
            print!("{station}");
            is_first = false;
        } else {
            print!(", {station}");
        }
        
    }
    println!("}}");
}