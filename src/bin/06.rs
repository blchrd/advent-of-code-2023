advent_of_code::solution!(6);

fn get_race_record(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut times: String = "".to_string();
    let mut distances: String = "".to_string();
    for line in input.lines() {
        if line.starts_with("Time:") {
            times = line.strip_prefix("Time:").unwrap().to_string();
        }

        if line.starts_with("Distance:") {
            distances = line.strip_prefix("Distance:").unwrap().to_string();
        }
    }

    let times_vec: Vec<u32> = times.split_ascii_whitespace().into_iter().map(|s| s.trim().parse().unwrap()).collect();
    let distances_vec: Vec<u32> = distances.split_ascii_whitespace().into_iter().map(|s| s.trim().parse().unwrap()).collect();

    (times_vec, distances_vec)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (times, distances) = get_race_record(input);
    let mut i = 0;
    let mut response = 1;

    // T: Race Time
    // d: distance
    // t: holding time
    // d = t(T - t)
    // d = tT - t²
    // h² - tT + d = 0
    // h = (T +/- sqrt(T² - 4d)) / 2 // not gonna lie, I didn't remember this formula, so I searched it, but I remembered it was a formula, I guess that count

    for time in times {
        let sqrt = (time*time - 4*distances[i]) as f32;
        let min_holding_time = ((time as f32 - sqrt.sqrt().ceil()) / 2.0) as u32;
        let max_holding_time = ((time as f32 + sqrt.sqrt().floor()) / 2.0) as u32;

        let nb_ways = (min_holding_time..max_holding_time).count();
        response *= nb_ways as u32;

        i += 1
    }

    Some(response)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut times: String = "".to_string();
    let mut distances: String = "".to_string();
    for line in input.lines() {
        if line.starts_with("Time:") {
            times = line.strip_prefix("Time:").unwrap().to_string();
        }

        if line.starts_with("Distance:") {
            distances = line.strip_prefix("Distance:").unwrap().to_string();
        }
    }

    let time_str = times.replace(' ',"");
    let distance_str = distances.replace(' ',"");

    let time: u128 = time_str.trim().parse().unwrap();
    let distance: u128 = distance_str.trim().parse().unwrap();

    println!("{} / {} ; {} / {}", time_str, distance_str, time, distance);

    let sqrt = (time*time - 4*distance) as f64;
    let min_holding_time = ((time as f64 - sqrt.sqrt().ceil()) / 2.0) as u128;
    let max_holding_time = ((time as f64 + sqrt.sqrt().floor()) / 2.0) as u128;

    let nb_ways = (min_holding_time..max_holding_time).count() as u64;

    Some(nb_ways)
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
