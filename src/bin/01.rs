use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut calibration: u32 = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for char in line.chars() {
            if char.is_digit(10) {
                if !first_digit.is_some() {
                    first_digit = char.to_digit(10);
                }
                last_digit = char.to_digit(10);
            }
        }

        let line_calibration: u32 = format!("{}{}", first_digit.unwrap(), last_digit.unwrap())
            .trim()
            .parse()
            .unwrap();
        calibration += line_calibration;
    }

    Some(calibration)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calibration: u32 = 0;
    let number_strings: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    for line in input.lines() {
        let mut str_temp: String = "".to_string();
        let mut current_digit: Option<u32> = None;
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for char in line.chars() {
            if char.is_digit(10) {
                current_digit = char.to_digit(10);
            } else {
                str_temp.push_str(&format!("{}", char));
                for (number_string, digit) in number_strings.clone() {
                    if str_temp.contains(&number_string) {
                        current_digit = Some(digit);
                        str_temp = "".to_string();
                        str_temp.push_str(&format!("{}", char));
                    }
                }
            }

            if current_digit.is_some() {
                if !first_digit.is_some() {
                    first_digit = current_digit.clone();
                }
                last_digit = current_digit.clone();
            }
        }

        let line_calibration: u32 = format!("{}{}", first_digit.unwrap(), last_digit.unwrap())
            .trim()
            .parse()
            .unwrap();

        calibration += line_calibration;
    }

    Some(calibration)
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
