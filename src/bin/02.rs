advent_of_code::solution!(2);

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct GameCubeSet {
    game_id: u32,
    cube_set: Vec<CubeCount>,
}
#[derive(Debug, Clone)]
struct CubeCount {
    color: Color,
    count: u32,
}

fn get_game_set(game_input: &str) -> GameCubeSet {
    let mut game_set = GameCubeSet{game_id:0, cube_set: Vec::<CubeCount>::new()};
    let temp: Vec<&str> = game_input.split(':').collect();
    game_set.game_id = temp[0].split(' ').collect::<Vec<&str>>()[1].trim().parse().unwrap();
    let games = temp[1].split(';');

    game_set.cube_set = vec![
        CubeCount{color:Color::Red, count:0},
        CubeCount{color:Color::Green, count:0},
        CubeCount{color:Color::Blue, count:0},
    ];

    for game in games {
        let playthrough = get_playthrough(game);
        
        for cube in playthrough {
            let index = game_set.cube_set.iter().position(|c| cube.color == c.color);
            if index.is_some() {
                if cube.count > game_set.cube_set[index.unwrap()].count {
                    game_set.cube_set[index.unwrap()] = cube;
                }
            }
        }
    }


    game_set
}

fn get_playthrough(playthrough_input: &str) -> Vec<CubeCount> {
    let mut game = Vec::<CubeCount>::new();
    for cube in playthrough_input.split(',') {
        let temp: Vec<&str> = cube.trim().split(' ').collect();
        let count: u32 = temp[0].trim().parse().unwrap();
        let mut color: Option<Color> = None;
        match temp[1] {
            "red" => {color = Some(Color::Red)}
            "green" => {color = Some(Color::Green)}
            "blue" => {color = Some(Color::Blue)}
            _ => {}
        }

        if color.is_some() {
            game.insert(0, CubeCount{color:color.unwrap(), count:count});
        }
    }

    game
}

pub fn part_one(input: &str) -> Option<u32> {
    // 12 red, 13 green, 14 blue
    let mut id_games_sum: u32 = 0;

    for line in input.lines() {
        let game_set = get_game_set(line);
        let mut impossible = false;
        
        for cube in game_set.cube_set {
            match cube.color {
                Color::Red => {impossible = cube.count > 12}
                Color::Green => {impossible = cube.count > 13}
                Color::Blue => {impossible = cube.count > 14}
            }
            if impossible {break;}
        }

        if !impossible {
            // println!("Game {} possible", id);
            id_games_sum += game_set.game_id;
        } else {
            // println!("Game {} impossible", id);
        }
    }
    
    Some(id_games_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game_power_sum: u32 = 0;

    for line in input.lines() {
        let mut game_power: u32 = 1; //It is a multiplication, so cannot init this to 0
        let game_set = get_game_set(line);

        for cube in game_set.cube_set {
            game_power *= cube.count;
        }

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
