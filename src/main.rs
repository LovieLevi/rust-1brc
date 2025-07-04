use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct WeatherStation {
    pub name: String,
    pub min_temp: f32,
    pub max_temp: f32,
    pub sum: f32,
    pub count: i64,
}

fn main() {
    let mut weather_stations: Vec<WeatherStation> = Vec::new();

    if let Ok(lines) = read_lines("./data/measurements.csv") {
        let mut i: usize = 0;
        for line in lines.flatten() {
            if i % 1000 == 0 {
                println!("Processing line {}", i);
            }
            let parsed_line: Vec<&str> = line.split(';').collect::<Vec<&str>>();
            let weather_station_name = parsed_line[0].to_string();
            let weather_station_temp = parsed_line[1].parse::<f32>().unwrap();
            if weather_stations.iter().any(|s| s.name == weather_station_name) {
                let station = weather_stations.iter_mut().find(|s| s.name == weather_station_name).unwrap();
                station.sum += weather_station_temp;
                station.count += 1;
            } else {
                let station = WeatherStation {
                    name: weather_station_name,
                    min_temp: weather_station_temp,
                    max_temp: weather_station_temp,
                    sum: weather_station_temp,
                    count: 1,
                };
                weather_stations.push(station);
            }
            i += 1;
        }
    }

    weather_stations.sort_by(|a, b| a.name.cmp(&b.name));
    let mut result: String = "{".to_string();
    for station in weather_stations.iter_mut() {
        let avg_temp = station.sum / station.count as f32;
        result += format!("{}={}/{}/{}, ", station.name, station.min_temp, station.max_temp, avg_temp).as_str();
    }
    result = result.trim_end_matches(", ").to_string();
    result += "}";
    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
