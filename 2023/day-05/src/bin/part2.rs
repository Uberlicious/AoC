use rayon::prelude::*;
use std::ops::Range;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct RangeMaps {
    seeds: Vec<Range<u64>>,
    seed_soil: Vec<(Range<u64>, Range<u64>)>,
    soil_fertilizer: Vec<(Range<u64>, Range<u64>)>,
    fertilizer_water: Vec<(Range<u64>, Range<u64>)>,
    water_light: Vec<(Range<u64>, Range<u64>)>,
    light_temperature: Vec<(Range<u64>, Range<u64>)>,
    temperature_humidity: Vec<(Range<u64>, Range<u64>)>,
    humidity_location: Vec<(Range<u64>, Range<u64>)>,
}

impl RangeMaps {
    fn new() -> RangeMaps {
        RangeMaps {
            seeds: vec![],
            seed_soil: vec![],
            soil_fertilizer: vec![],
            fertilizer_water: vec![],
            water_light: vec![],
            light_temperature: vec![],
            temperature_humidity: vec![],
            humidity_location: vec![],
        }
    }

    //     15
    // 10......25

    fn get_map(&self, ranges: &Vec<(Range<u64>, Range<u64>)>, s: u64, m: &mut Vec<u64>) {
        for range in ranges {
            if s >= range.0.start && s <= range.0.end {
                let idx = s - range.0.start;
                if let Some(map) = range.1.clone().nth(idx as usize) {
                    m.push(map);
                    return;
                }
            }
        }

        m.push(s);
    }

    fn closest(&self) -> u64 {
        *self
            .seeds
            .par_iter()
            .map(|s| {
                *s.clone()
                    .into_par_iter()
                    .map(|s| {
                        let mut lowest = u64::MAX;

                        let mut m = vec![];
                        self.get_map(&self.seed_soil, s.clone(), &mut m);
                        self.get_map(&self.soil_fertilizer, m[0].clone(), &mut m);
                        self.get_map(&self.fertilizer_water, m[1].clone(), &mut m);
                        self.get_map(&self.water_light, m[2].clone(), &mut m);
                        self.get_map(&self.light_temperature, m[3].clone(), &mut m);
                        self.get_map(&self.temperature_humidity, m[4].clone(), &mut m);
                        self.get_map(&self.humidity_location, m[5].clone(), &mut m);

                        if let Some(last) = m.last() {
                            if *last < lowest {
                                lowest = *last;
                            }
                        }

                        return lowest;
                    })
                    .collect::<Vec<u64>>()
                    .iter()
                    .min()
                    .unwrap()
            })
            .collect::<Vec<u64>>()
            .iter()
            .min()
            .unwrap()
    }
}

fn gen_ranges(line: &str, range_map: &mut Vec<(Range<u64>, Range<u64>)>) {
    let mut iter = line.trim().split_ascii_whitespace();
    // dest / source / range
    let dest = iter.next().unwrap().parse::<u64>().unwrap();
    let source = iter.next().unwrap().parse::<u64>().unwrap();
    let range = iter.next().unwrap().parse::<u64>().unwrap();

    let s = (source..source + range).clone();
    let d = (dest..dest + range).clone();
    range_map.push((s, d));
}

fn process(input: &str) -> u64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let mut maps = RangeMaps::new();
    let mut current_category = String::from("");

    for l in lines {
        if l.is_empty() {
            continue;
        }
        let split = l.split(":").collect::<Vec<&str>>();
        if split[0] == "seeds" {
            let seeds = split[1]
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            seeds
                .iter()
                .step_by(2)
                .zip(seeds.iter().skip(1).step_by(2))
                .for_each(|(x, y)| {
                    maps.seeds.push(*x..*x + *y);
                });
        }

        let map_split = &split[0].split_ascii_whitespace().collect::<Vec<_>>();
        if map_split.len() > 1 && map_split[1] == "map" {
            current_category = map_split[0].to_string();
            continue;
        }

        if current_category == "seed-to-soil" {
            gen_ranges(split[0], &mut maps.seed_soil);
        }

        if current_category == "soil-to-fertilizer" {
            gen_ranges(split[0], &mut maps.soil_fertilizer);
        }

        if current_category == "fertilizer-to-water" {
            gen_ranges(split[0], &mut maps.fertilizer_water);
        }

        if current_category == "water-to-light" {
            gen_ranges(split[0], &mut maps.water_light);
        }

        if current_category == "light-to-temperature" {
            gen_ranges(split[0], &mut maps.light_temperature);
        }

        if current_category == "temperature-to-humidity" {
            gen_ranges(split[0], &mut maps.temperature_humidity);
        }

        if current_category == "humidity-to-location" {
            gen_ranges(split[0], &mut maps.humidity_location);
        }
    }

    maps.closest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
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
        assert_eq!(result, 46);
    }
}
