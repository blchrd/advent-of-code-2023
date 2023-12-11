use std::{collections::HashMap, thread::current};

advent_of_code::solution!(10);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

fn get_connections() -> HashMap<char, Vec<Direction>> {
    let mut connection = HashMap::<char, Vec<Direction>>::new();

    connection.insert('|', vec![Direction::Up, Direction::Down]);
    connection.insert('-', vec![Direction::Left, Direction::Right]);

    connection.insert('L', vec![Direction::Up, Direction::Right]);
    connection.insert('J', vec![Direction::Up, Direction::Left]);
    connection.insert('7', vec![Direction::Down, Direction::Left]);
    connection.insert('F', vec![Direction::Down, Direction::Right]);

    connection.insert('.', vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]);
    connection.insert('S', vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]);

    connection
}

fn get_direction_delta(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Left => {return (0, -1)},
        Direction::Right => {return (0, 1)},
        Direction::Up => {return (-1, 0)},
        Direction::Down => {return (1, 0)},
    }
}

fn connecting(pipe_map: Vec<Vec<char>>, start_pipe: (u32, u32), direction: Direction) -> bool {
    let mut connected: bool = false;
    let connections = get_connections();
    let direction_delta = get_direction_delta(direction.clone());
    let destination_pipe = ((start_pipe.0 as i32+direction_delta.0) as u32, (start_pipe.1 as i32+direction_delta.1) as u32);

    let connections_start = connections.get(&pipe_map[start_pipe.0 as usize][start_pipe.1 as usize]).unwrap();
    let connections_destination = connections.get(&pipe_map[destination_pipe.0 as usize][destination_pipe.1 as usize]).unwrap();

    if connections_start.contains(&direction) {
        match direction.clone() {
            Direction::Left => {connected = connections_destination.contains(&Direction::Right)},
            Direction::Right => {connected = connections_destination.contains(&Direction::Left)},
            Direction::Up => {connected = connections_destination.contains(&Direction::Down)},
            Direction::Down => {connected = connections_destination.contains(&Direction::Up)},
        }
    }

    connected
}

fn moving(pipe_map: Vec<Vec<char>>, start_position: (u32, u32), direction: Direction) -> Option<(u32, u32)> {
    let mut destination_position: Option<(u32, u32)> = None;
    let move_delta = get_direction_delta(direction.clone());
    if connecting(pipe_map.clone(), start_position.clone(), direction.clone()) {
        destination_position = Some(((start_position.0 as i32 + move_delta.0) as u32, (start_position.1 as i32 + move_delta.1) as u32));
    }

    // dbg!(pipe_map[start_position.0 as usize][start_position.1 as usize]);
    // if destination_position.is_some() {
    //     dbg!(pipe_map[destination_position.unwrap().0 as usize][destination_position.unwrap().1 as usize]);
    // }

    destination_position
}

fn get_pipe_map(input: &str) -> Vec<Vec<char>> {
    let mut pipe_map = Vec::<Vec<char>>::new();
    let mut y = 0;

    for line in input.lines() {
        pipe_map.insert(y, line.chars().collect());
        y += 1;
    }

    pipe_map
}

pub fn part_one(input: &str) -> Option<u32> {
    let pipe_map = get_pipe_map(input);
    let mut visited_map = Vec::<(u32, u32)>::new();
    let mut start_position: (u32, u32) = (0, 0);
    for y in 0..pipe_map.len() {
        for x in 0..pipe_map[y].len() {
            if pipe_map[y][x] == 'S' {
                start_position = (y as u32, x as u32);
                break;
            }
        }

        if start_position != (0,0) {
            break;
        }
    }

    visited_map.push(start_position);
    let mut current_position = start_position;
    let mut current_direction: Option<Direction> = None;
    if connecting(pipe_map.clone(), start_position, Direction::Left) {
        current_direction = Some(Direction::Left);
    }
    if connecting(pipe_map.clone(), start_position, Direction::Right) {
        current_direction = Some(Direction::Right);
    }
    if connecting(pipe_map.clone(), start_position, Direction::Up) {
        current_direction = Some(Direction::Up);
    }
    if connecting(pipe_map.clone(), start_position, Direction::Down) {
        current_direction = Some(Direction::Down);
    }

    let mut nb_step: u32 = 1;
    current_position = moving(pipe_map.clone(), current_position, current_direction.clone().unwrap()).unwrap();
    while pipe_map[current_position.0 as usize][current_position.1 as usize] != 'S' {
        nb_step += 1;
        let mut direction_chose = false;
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Left) && current_direction.clone() != Some(Direction::Right) {
            current_direction = Some(Direction::Left);
            direction_chose = true;
        }
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Right) && current_direction.clone() != Some(Direction::Left) {
            current_direction = Some(Direction::Right);
            direction_chose = true;
        }
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Up) && current_direction.clone() != Some(Direction::Down) {
            current_direction = Some(Direction::Up);
            direction_chose = true;
        }
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Down) && current_direction.clone() != Some(Direction::Up) {
            current_direction = Some(Direction::Down);
        }

        current_position = moving(pipe_map.clone(), current_position, current_direction.clone().unwrap()).unwrap();
    }

    Some(nb_step / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    // We'll need to know the pipe type of starting point

    let pipe_map = get_pipe_map(input);
    let mut visited_map = Vec::<(u32, u32)>::new();
    let mut start_position: (u32, u32) = (0, 0);
    for y in 0..pipe_map.len() {
        for x in 0..pipe_map[y].len() {
            if pipe_map[y][x] == 'S' {
                start_position = (y as u32, x as u32);
                break;
            }
        }

        if start_position != (0,0) {
            break;
        }
    }

    visited_map.push(start_position);
    let mut current_position = start_position;
    let mut current_direction: Option<Direction> = None;
    if connecting(pipe_map.clone(), start_position, Direction::Left) {
        current_direction = Some(Direction::Left);
    }
    if connecting(pipe_map.clone(), start_position, Direction::Right) {
        current_direction = Some(Direction::Right);
    }
    if connecting(pipe_map.clone(), start_position, Direction::Up) {
        current_direction = Some(Direction::Up);
    }
    if connecting(pipe_map.clone(), start_position, Direction::Down) {
        current_direction = Some(Direction::Down);
    }

    current_position = moving(pipe_map.clone(), current_position, current_direction.clone().unwrap()).unwrap();
    while pipe_map[current_position.0 as usize][current_position.1 as usize] != 'S' {
        let mut direction_chose = false;
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Left) && current_direction.clone() != Some(Direction::Right) {
            current_direction = Some(Direction::Left);
            direction_chose = true;
        }
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Right) && current_direction.clone() != Some(Direction::Left) {
            current_direction = Some(Direction::Right);
            direction_chose = true;
        }
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Up) && current_direction.clone() != Some(Direction::Down) {
            current_direction = Some(Direction::Up);
            direction_chose = true;
        }
        if !direction_chose && connecting(pipe_map.clone(), current_position, Direction::Down) && current_direction.clone() != Some(Direction::Up) {
            current_direction = Some(Direction::Down);
        }

        current_position = moving(pipe_map.clone(), current_position, current_direction.clone().unwrap()).unwrap();
    }

    None
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
