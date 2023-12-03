advent_of_code::solution!(3);

fn get_schematic(input: &str) -> Vec<Vec<char>> {
    let mut schematic = Vec::<Vec<char>>::new();
    let mut y = 0;

    for line in input.lines() {
        schematic.insert(y, line.chars().collect());
        y += 1;
    }

    schematic
}

fn has_adjacent_symbol(schematic: Vec<Vec<char>> ,x: i32, y: i32) -> bool {
    let current_char = schematic[x as usize][y as usize];
    // x+1 / x-1 / y+1 / y-1 / x+1;y+1 / x+1;y-1 / x-1;y-1 / x-1;y+1
    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

    if current_char.is_digit(10) {
        for neighbor in neighbors {
            if x+neighbor.0 >= 0 && y+neighbor.1 >= 0 && x+neighbor.0 < 140 && y+neighbor.1 < 140 {
                if schematic[(x+neighbor.0) as usize][(y+neighbor.1) as usize] != '.' && !schematic[(x+neighbor.0) as usize][(y+neighbor.1) as usize].is_digit(10) {
                    return true;
                }
            }
        }
    }

    false
}

fn get_gear_ratio(schematic: Vec<Vec<char>>, x: i32, y: i32) -> Option<u32> {
    let current_char = schematic[x as usize][y as usize];
    let mut neighbors_number = Vec::<u32>::new();
    // x+1 / x-1 / y+1 / y-1 / x+1;y+1 / x+1;y-1 / x-1;y-1 / x-1;y+1
    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut neighbors_count = 0;

    if current_char == '*' {
        for neighbor in neighbors {
            if x+neighbor.0 >= 0 && y+neighbor.1 >= 0 && x+neighbor.0 < 140 && y+neighbor.1 < 140 {
                if schematic[(x+neighbor.0) as usize][(y+neighbor.1) as usize].is_digit(10) {
                    if !neighbors_number.contains(&get_number_from_char_position(schematic.clone(), (x+neighbor.0) as usize, (y+neighbor.1) as usize)) {
                        neighbors_count += 1;
                        neighbors_number.insert(0, get_number_from_char_position(schematic.clone(), (x+neighbor.0) as usize, (y+neighbor.1) as usize));
                    }
                }
            }
        }

        if neighbors_count == 2 {
            let mut gear_ratio = 1;
            for neighbor_number in neighbors_number {
                gear_ratio *= neighbor_number;
            }

            return Some(gear_ratio);
        }
    }

    None
}

fn get_number_from_char_position(schematic: Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let current_char = schematic[x][y];
    let mut current_number = "".to_string();

    if current_char.is_digit(10) {
        let mut current_y = y.clone();
        while current_y-1 > 0 && schematic[x][current_y-1].is_digit(10) {
            current_y -= 1;
        }

        while current_y < 140 && schematic[x][current_y].is_digit(10) {
            current_number.push_str(&schematic[x][current_y].to_string());
            current_y += 1;
        }

        let number: u32 = current_number.trim().parse().unwrap();
        return number;
    }

    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = get_schematic(input);
    let mut current_number = "".to_string();
    let mut is_part_number = false;
    let mut sum: u32 = 0;

    for x in 0..140 {
        for y in 0..140 {
            if has_adjacent_symbol(schematic.clone(), x as i32, y as i32) {
                is_part_number = true;
            }
            if schematic[x][y].is_digit(10) {
                current_number.push_str(&schematic[x][y].to_string());
            } else {
                if is_part_number {
                    let part_number: u32 = current_number.trim().parse().unwrap();
                    sum += part_number;
                    is_part_number = false;
                }

                current_number = "".to_string();
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = get_schematic(input);
    let mut sum: u32 = 0;

    for x in 0..140 {
        for y in 0..140 {
            let gear_ratio = get_gear_ratio(schematic.clone(), x, y);
            if gear_ratio.is_some() {
                sum += gear_ratio.unwrap();
            }
        }
    }

    Some(sum)
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
