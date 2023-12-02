advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // 12 red, 13 green, 14 blue
    let mut id_games_sum: u32 = 0;

    for line in input.lines() {
        let mut possible = true;
        let mut temp: Vec<&str> = line.split(':').collect();
        let id: u32 = temp[0].clone().split(' ').collect::<Vec<&str>>()[1].trim().parse().unwrap();
        let games = temp[1].clone().split(';');

        for game in games {
            let cubes = game.split(',');
            for cube in cubes {
                temp = cube.trim().split(' ').collect();

                let number: u32 = temp[0].trim().parse().unwrap();
                let color = temp[1];

                match color {
                    "red" => {
                        if number > 12 {
                            possible = false;
                            break;
                        }
                    }
                    "green" => {
                        if number > 13 {
                            possible = false;
                            break;
                        }
                    }
                    "blue" => {
                        if number > 14 {
                            possible = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        if possible {
            // println!("Game {} possible", id);
            id_games_sum += id
        } else {
            // println!("Game {} impossible", id);
        }
    }

    Some(id_games_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game_power_sum: u32 = 0;

    for line in input.lines() {
        let mut nb_red_cubes: u32 = 0;
        let mut nb_green_cubes: u32 = 0;
        let mut nb_blue_cubes: u32 = 0;
        let game_power: u32;
        let mut temp: Vec<&str> = line.split(':').collect();
        // let id: u32 = temp[0].clone().split(' ').collect::<Vec<&str>>()[1].trim().parse().unwrap();
        let games = temp[1].clone().split(';');

        for game in games {
            let cubes = game.split(',');
            for cube in cubes {
                temp = cube.trim().split(' ').collect();

                let number: u32 = temp[0].trim().parse().unwrap();
                let color = temp[1];

                match color {
                    "red" => {
                        if number > nb_red_cubes {
                            nb_red_cubes = number.clone();
                        }
                    }
                    "green" => {
                        if number > nb_green_cubes {
                            nb_green_cubes = number.clone();
                        }
                    }
                    "blue" => {
                        if number > nb_blue_cubes {
                            nb_blue_cubes = number.clone();
                        }
                    }
                    _ => {}
                }
            }
        }

        game_power = nb_red_cubes*nb_green_cubes*nb_blue_cubes;

        // println!("Power of game {id} ({red}r, {green}g, {blue}b): {power}", id=id, red=nb_red_cubes, green=nb_green_cubes, blue=nb_blue_cubes, power=game_power);

        game_power_sum += game_power;
    }

    Some(game_power_sum)
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
