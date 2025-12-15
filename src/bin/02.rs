use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.replace("\n", "").split(",") {
        let Some((first, second)) = line.split_once("-") else {
            println!("{}", line);
            panic!("{} - not split", line);
        };
        let start_id = first.parse::<u64>().unwrap();
        let end_id = second.parse::<u64>().unwrap();

        let start_log_10 = start_id.ilog10();
        let mut current_pair_multiplier = match start_log_10 % 2 == 0 {
            true => 10_u64.pow(start_log_10 / 2),
            false => 10_u64.pow(start_log_10 / 2 + 1),
        };
        let mut first_digits = start_id / current_pair_multiplier;
        if first_digits >= current_pair_multiplier {
            first_digits = current_pair_multiplier;
            current_pair_multiplier *= 10;
        }
        while first_digits * (current_pair_multiplier + 1) <= end_id {
            let current_id = first_digits * (current_pair_multiplier + 1);
            first_digits += 1;
            if current_id < start_id {
                continue;
            }
            sum += current_id;
            if first_digits >= current_pair_multiplier {
                current_pair_multiplier *= 10;
            }
        }
    }
    Some(sum)
}

fn get_patterns(first_digits: u64, number_digits: u64) -> Vec<u64> {
    let mut possible_patterns = Vec::new();
    let mut digits = first_digits;

    while digits > 0 {
        let pattern_length = (digits.ilog10() + 1).into();
        if number_digits % pattern_length == 0 {
            let mut number = digits;
            for mult in (pattern_length..number_digits).step_by(pattern_length.try_into().unwrap())
            {
                number += 10_u64.pow(mult.try_into().unwrap()) * digits;
            }
            // print!("{} |", number);
            possible_patterns.push(number);
        }
        digits /= 10;
    }
    // println!();
    possible_patterns
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.replace("\n", "").split(",") {
        let Some((first, second)) = line.split_once("-") else {
            println!("{}", line);
            panic!("{} - not split", line);
        };
        let start_id = first.parse::<u64>().unwrap();
        let end_id = second.parse::<u64>().unwrap();
        let mut solutions = HashSet::new();

        let start_log_10 = start_id.ilog10();
        let mut current_pair_multiplier = match start_log_10 % 2 == 0 {
            true => 10_u64.pow(start_log_10 / 2),
            false => 10_u64.pow(start_log_10 / 2 + 1),
        };
        let mut first_digits = start_id / current_pair_multiplier;

        if first_digits >= current_pair_multiplier {
            current_pair_multiplier *= 10;
            first_digits /= 10;
        }
        if first_digits == 0 {
            first_digits = 1;
        }
        // println!("--------------------------\n{} {} - ", start_id, end_id);
        while (first_digits * (current_pair_multiplier) <= end_id) & (first_digits > 0) {
            let patterns = get_patterns(
                first_digits,
                (first_digits * current_pair_multiplier * 10)
                    .ilog10()
                    .into(),
            );
            for pattern in patterns {
                if (pattern >= start_id) & (pattern <= end_id) {
                    solutions.insert(pattern);
                }
            }
            first_digits += 1;
            if first_digits >= current_pair_multiplier {
                current_pair_multiplier *= 10;
                first_digits /= 10;
            }
        }
        let mut sorted = solutions.into_iter().collect::<Vec<u64>>();
        sorted.sort();
        let filter = sorted.len() == 3;
        if filter {
            println!("--------------------------\n{} {} - ", start_id, end_id);
        }
        for entry in sorted {
            if filter {
                print!("{} | ", entry);
            }
            sum += entry;
        }
        if filter {
            println!();
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
