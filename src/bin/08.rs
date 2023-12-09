use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let path: &str;
    let mut crossroads: HashMap<&str, (&str, &str)> = HashMap::new();

    path = input.lines().into_iter().collect::<Vec<&str>>()[0].trim();
    for line in input.lines() {
        if line != "" && line.contains("=") {
            let key = line.split('=').collect::<Vec<&str>>()[0].trim();
            let left_right = line.split('=').collect::<Vec<&str>>()[1].trim()
                    .split(',').into_iter().map(|c| {c.trim()}).collect::<Vec<&str>>();

            crossroads.insert(key, (left_right[0].trim_start_matches('('), left_right[1].trim_end_matches(')')));
        }
    }

    let mut number_step = 0;
    let mut current = crossroads.get("AAA");
    let mut current_string = "AAA";
    while current_string != "ZZZ" {
        for choice in path.chars() {
            if current.is_some() {
                number_step += 1;
                if choice == 'L' {
                    current_string = current.unwrap().0;
                    current = crossroads.get(current.unwrap().0);
                } else if choice == 'R' {
                    current_string = current.unwrap().1;
                    current = crossroads.get(current.unwrap().1);
                }

                if current_string == "ZZZ" {
                    break;
                }
            }
        }
    }

    Some(number_step)
}

pub fn part_two(input: &str) -> Option<u128> {
    let path: &str;
    let mut crossroads: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starting_points = Vec::<&str>::new();


    path = input.lines().into_iter().collect::<Vec<&str>>()[0].trim();
    for line in input.lines() {
        if line != "" && line.contains("=") {
            let key = line.split('=').collect::<Vec<&str>>()[0].trim();
            let left_right = line.split('=').collect::<Vec<&str>>()[1].trim()
                    .split(',').into_iter().map(|c| {c.trim()}).collect::<Vec<&str>>();

            if key.ends_with("A") {
                starting_points.push(key);
            }
            crossroads.insert(key, (left_right[0].trim_start_matches('('), left_right[1].trim_end_matches(')')));
        }
    }

    let mut number_step_vec = Vec::<u128>::new();

    for starting_point in starting_points {
        let mut number_step = 0;
        let mut current = crossroads.get(starting_point);
        let mut current_string = "xxA";
        while !current_string.ends_with("Z") {
            for choice in path.chars() {
                if current.is_some() {
                    number_step += 1;
                    if choice == 'L' {
                        current_string = current.unwrap().0;
                        current = crossroads.get(current.unwrap().0);
                    } else if choice == 'R' {
                        current_string = current.unwrap().1;
                        current = crossroads.get(current.unwrap().1);
                    }

                    if current_string.ends_with("Z") {
                        break;
                    }
                }
            }
        }

        number_step_vec.push(number_step);
    }

    dbg!(&number_step_vec);

    //lcm(a, b, c) = lcm(a, lcm(b, c))
    let lcm = number_step_vec.into_iter().reduce(|a, b| num::integer::lcm(a, b));

    lcm
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
