use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Maps {
    seeds: Vec<u32>,
    seed_soil: HashMap<u32, u32>,
    soil_fertilizer: HashMap<u32, u32>,
    fertilizer_water: HashMap<u32, u32>,
    water_light: HashMap<u32, u32>,
    light_temperature: HashMap<u32, u32>,
    temperature_humidity: HashMap<u32, u32>,
    humidity_location: HashMap<u32, u32>,
}

impl Maps {
    fn new() -> Maps {
        Maps {
            seeds: vec![],
            seed_soil: HashMap::new(),
            soil_fertilizer: HashMap::new(),
            fertilizer_water: HashMap::new(),
            water_light: HashMap::new(),
            light_temperature: HashMap::new(),
            temperature_humidity: HashMap::new(),
            humidity_location: HashMap::new(),
        }
    }

    fn closest(&self) -> u32 {
        let mut lowest = u32::MAX;
        for s in &self.seeds {
            let mut keys = HashMap::new();

            if self.seed_soil.contains_key(&s) {
                keys.insert("soil", self.seed_soil[&s]);
            } else {
                keys.insert("soil", *s);
            }

            if self.soil_fertilizer.contains_key(&keys["soil"]) {
                keys.insert("fertilizer", self.soil_fertilizer[&keys["soil"]]);
            } else {
                keys.insert("fertilizer", keys["soil"]);
            }

            if self.fertilizer_water.contains_key(&keys["fertilizer"]) {
                keys.insert("water", self.fertilizer_water[&keys["fertilizer"]]);
            } else {
                keys.insert("water", keys["fertilizer"]);
            }

            if self.water_light.contains_key(&keys["water"]) {
                keys.insert("light", self.water_light[&keys["water"]]);
            } else {
                keys.insert("light", keys["water"]);
            }

            if self.light_temperature.contains_key(&keys["light"]) {
                keys.insert("temperature", self.light_temperature[&keys["light"]]);
            } else {
                keys.insert("temperature", keys["light"]);
            }

            if self.temperature_humidity.contains_key(&keys["temperature"]) {
                keys.insert("humidity", self.temperature_humidity[&keys["temperature"]]);
            } else {
                keys.insert("humidity", keys["temperature"]);
            }

            if self.humidity_location.contains_key(&keys["humidity"]) {
                keys.insert("location", self.humidity_location[&keys["humidity"]]);
            } else {
                keys.insert("location", keys["humidity"]);
            }

            if keys["location"] < lowest {
                lowest = keys["location"]
            }
        }
        lowest
    }
}

fn gen_map(line: &str, map: &mut HashMap<u32, u32>) {
    let mut iter = line.trim().split_ascii_whitespace();
    // dest / source / range
    let dest = iter.next().unwrap().parse::<u32>().unwrap();
    let source = iter.next().unwrap().parse::<u32>().unwrap();
    let range = iter.next().unwrap().parse::<u32>().unwrap();

    for idx in (0..range).into_iter() {
        map.insert(source + idx, dest + idx);
    }
}

fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let mut maps = Maps::new();
    let mut current_category = String::from("");

    for l in lines {
        if l.is_empty() {
            continue;
        }
        let split = l.split(":").collect::<Vec<&str>>();
        if split[0] == "seeds" {
            maps.seeds = split[1]
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        }

        // println!("seeds: {:?}", maps.seeds);

        let map_split = &split[0].split_ascii_whitespace().collect::<Vec<_>>();
        if map_split.len() > 1 && map_split[1] == "map" {
            current_category = map_split[0].to_string();
            continue;
        }

        if current_category == "seed-to-soil" {
            gen_map(split[0], &mut maps.seed_soil);
        }

        if current_category == "soil-to-fertilizer" {
            gen_map(split[0], &mut maps.soil_fertilizer);
        }

        if current_category == "fertilizer-to-water" {
            gen_map(split[0], &mut maps.fertilizer_water);
        }

        if current_category == "water-to-light" {
            gen_map(split[0], &mut maps.water_light);
        }

        if current_category == "light-to-temperature" {
            gen_map(split[0], &mut maps.light_temperature);
        }

        if current_category == "temperature-to-humidity" {
            gen_map(split[0], &mut maps.temperature_humidity);
        }

        if current_category == "humidity-to-location" {
            gen_map(split[0], &mut maps.humidity_location);
        }
    }

    // println!("maps: {:?}", maps);
    maps.closest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 35);
    }
}
