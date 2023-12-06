use std::ops::Range;

advent_of_code::solution!(5);

#[derive(Debug, Clone)]
struct Map {
    source: Range<u64>,
    destination: Range<u64>,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    seeds_range: Vec<Range<u64>>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn get_seeds(input: &str) -> Vec<u64> {
    let mut seeds = Vec::<u64>::new();

    for line in input.lines() {
        if line != "" {
            if line.starts_with("seeds:") {
                seeds = line.split(':').collect::<Vec<&str>>()[1]
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                break;
            }
        }
    }

    seeds
}

fn get_seeds_range(input: &str) -> Vec<Range<u64>> {
    let mut seeds = Vec::<u64>::new();
    let mut seeds_range = Vec::<Range<u64>>::new();

    for line in input.lines() {
        if line != "" {
            if line.starts_with("seeds:") {
                seeds = line.split(':').collect::<Vec<&str>>()[1]
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                break;
            }
        }
    }

    let mut i = 0;
    while i < seeds.len() {
        seeds_range.push(seeds[i]..seeds[i + 1]);
        i += 2;
    }

    seeds_range
}

fn get_map(input: &str, start_line: &str) -> Vec<Map> {
    let mut parsing_started = false;
    let mut maps = Vec::<Map>::new();
    for line in input.lines() {
        if line == "" && parsing_started {
            break;
        }

        if line.trim() == start_line {
            parsing_started = true
        } else if parsing_started {
            let map_part: Vec<_> = line.split(' ').collect();
            let destination_range_start: u64 = map_part[0].trim().parse().unwrap();
            let source_range_start: u64 = map_part[1].trim().parse().unwrap();
            let range_length: u64 = map_part[2].trim().parse().unwrap();

            maps.push(
                Map {
                    source: source_range_start..source_range_start+range_length,
                    destination: destination_range_start..destination_range_start+range_length,
                }
            );
        }
    }

    maps
}

fn get_value_from_map(source_value: u64, maps: Vec<Map>) -> u64 {
    let mut destination_value = source_value;

    for map in maps {
        if map.source.start <= source_value
            && source_value <= map.source.end
        {
            destination_value =
                map.destination.start + (source_value - map.source.start);
            break;
        }
    }

    destination_value
}

fn get_location(almanac: Almanac, seed: u64) -> u64 {
    let soil = get_value_from_map(seed, almanac.seed_to_soil);
    let fertilizer = get_value_from_map(soil, almanac.soil_to_fertilizer);
    let water = get_value_from_map(fertilizer, almanac.fertilizer_to_water);
    let light = get_value_from_map(water, almanac.water_to_light);
    let temperature = get_value_from_map(light, almanac.light_to_temperature);
    let humidity = get_value_from_map(temperature, almanac.temperature_to_humidity);
    let location = get_value_from_map(humidity, almanac.humidity_to_location);

    location
}

fn get_ranges(source_ranges: Vec<Range<u64>>, maps: Vec<Map>) -> Vec<Range<u64>> {
    let mut ranges = Vec::<Range<u64>>::new();

    for map in maps {
        for source_range in &source_ranges {
            if (source_range.start < map.source.start && source_range.end < map.source.start) || (source_range.start > map.source.end && source_range.end > map.source.end) {
                ranges.push(source_range.start..source_range.end);
            } else if source_range.start < map.source.start && source_range.end > map.source.end {
                ranges.push(source_range.start..map.source.start);
                ranges.push(map.destination.start..map.destination.end);
                ranges.push(map.source.end..map.source.end);
            } else if source_range.start > map.source.start && source_range.end < map.source.end {
                let range_start = map.destination.start + (source_range.start - map.source.start);
                let range_end = map.destination.start + (source_range.end - map.source.start);

                ranges.push(range_start..range_end);
            } else if source_range.start > map.source.start && source_range.end > map.source.end {
                ranges.push(map.destination.start + (source_range.start - map.source.start)..map.destination.end);
                ranges.push(map.source.end..map.source.end);
            } else if source_range.start < map.source.start && source_range.end < map.source.end {
                ranges.push(source_range.start..map.source.start);
                ranges.push(map.destination.start..map.destination.end);
            }
        }
    }

    ranges
}

fn get_min_location_for_seed(almanac: Almanac, seeds: Vec<Range<u64>>) -> u64 {
    let mut min_location: Option<u64> = None;

    let soil = get_ranges(seeds, almanac.clone().seed_to_soil);
    let fertilizer = get_ranges(soil, almanac.clone().soil_to_fertilizer);
    let water = get_ranges(fertilizer, almanac.clone().fertilizer_to_water);
    let light = get_ranges(water, almanac.clone().water_to_light);
    let temperature = get_ranges(light, almanac.clone().light_to_temperature);
    let humidity = get_ranges(temperature, almanac.clone().temperature_to_humidity);
    let location = get_ranges(humidity, almanac.clone().humidity_to_location);

    let location_bound = location.into_iter().min_by(|a, b| a.start.cmp(&b.start));
    if location_bound.is_some() {
        min_location = Some(location_bound.unwrap().start);
    }

    min_location.unwrap()
}

fn get_almanac(input: &str) -> Almanac {
    let almanac = Almanac {
        seeds: get_seeds(input),
        seeds_range: get_seeds_range(input),
        seed_to_soil: get_map(input, "seed-to-soil map:"),
        soil_to_fertilizer: get_map(input, "soil-to-fertilizer map:"),
        fertilizer_to_water: get_map(input, "fertilizer-to-water map:"),
        water_to_light: get_map(input, "water-to-light map:"),
        light_to_temperature: get_map(input, "light-to-temperature map:"),
        temperature_to_humidity: get_map(input, "temperature-to-humidity map:"),
        humidity_to_location: get_map(input, "humidity-to-location map:"),
    };

    almanac
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = get_almanac(input);
    let mut min_location: Option<u64> = None;

    for seed in &almanac.seeds {
        let location = get_location(almanac.clone(), *seed);

        if min_location.is_none() {
            min_location = Some(location.clone());
        } else if location < min_location.unwrap() {
            min_location = Some(location.clone());
        }
    }

    min_location
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = get_almanac(input);
    let mut min_location: Option<u64> = None;
    let mut seed_index = 0;

    for seed in &almanac.seeds_range {
        seed_index += 1;
        println!("{} / {}", seed_index, almanac.seeds_range.clone().len());

        let location = get_min_location_for_seed(almanac.clone(), vec![seed.clone()]);

        if min_location.is_none() {
            min_location = Some(location.clone());
        } else if location < min_location.unwrap() {
            min_location = Some(location.clone());
        }
    }

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
