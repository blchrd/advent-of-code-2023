use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    for line in input.lines() {
        let mut init_vec = Vec::<i64>::new();
        let mut transform_vec = Vec::<Vec<i64>>::new();

        init_vec = line.trim().split(' ').collect::<Vec<&str>>()
            .into_iter().map(|s| s.trim().parse().unwrap()).collect();

        let mut current_vec = init_vec.clone();
        while current_vec.clone().into_iter().unique().collect::<Vec<i64>>() != vec![0] {
            let mut temp_vec = Vec::<i64>::new();
            for (i, _) in current_vec.clone().iter().enumerate() {
                if i+1 < current_vec.clone().len() {
                    temp_vec.push(current_vec[i+1] - current_vec[i]);
                }
            }

            transform_vec.push(temp_vec.clone());
            current_vec = temp_vec.clone();
        }

        let mut last_add: Option<i64> = None;
        for (i, _) in transform_vec.clone().into_iter().rev().enumerate() {
            if last_add.is_none() {
                last_add = transform_vec[i].clone().into_iter().last().clone();
            } else {
                last_add = Some(transform_vec[i].iter().last().unwrap().clone() + last_add.unwrap().clone());
            }
        }

        result += init_vec.clone().into_iter().last().unwrap() + last_add.unwrap();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    for line in input.lines() {
        let mut init_vec = Vec::<i64>::new();
        let mut transform_vec = Vec::<Vec<i64>>::new();

        init_vec = line.trim().split(' ').collect::<Vec<&str>>()
            .into_iter().rev().map(|s| s.trim().parse().unwrap()).collect();

        let mut current_vec = init_vec.clone();
        while current_vec.clone().into_iter().unique().collect::<Vec<i64>>() != vec![0] {
            let mut temp_vec = Vec::<i64>::new();
            for (i, _) in current_vec.clone().iter().rev().enumerate() {
                if i+1 < current_vec.clone().len() {
                    temp_vec.push(current_vec[i+1] - current_vec[i]);
                }
            }

            transform_vec.push(temp_vec.clone());
            current_vec = temp_vec.clone();
        }

        let mut last_add: Option<i64> = None;
        for (i, _) in transform_vec.clone().into_iter().rev().enumerate() {
            if last_add.is_none() {
                last_add = transform_vec[i].clone().into_iter().last().clone();
            } else {
                last_add = Some(transform_vec[i].iter().last().unwrap().clone() + last_add.unwrap().clone());
            }
        }

        result += init_vec.clone().into_iter().last().unwrap() + last_add.unwrap();
    }

    Some(result)
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
