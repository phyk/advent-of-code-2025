use std::ops::Add;

advent_of_code::solution!(1);

enum Direction {
    R,
    L,
}

struct Operation {
    direction: Direction,
    value: u64,
}

struct Code {
    current_value_r: u64,
    current_value_l: u64,
    password: u64,
}

impl Operation {
    fn simplify(self) -> Operation {
        Operation {
            direction: self.direction,
            value: self.value % 100,
        }
    }
}

impl Code {
    fn new(password: u64, current_value: u64, direction: Direction) -> Code {
        if current_value > 100 {
            panic!("Impossible value");
        }
        let other_value = (100 - current_value) % 100;
        match direction {
            Direction::L => Code {
                current_value_r: other_value,
                current_value_l: current_value,
                password,
            },
            Direction::R => Code {
                current_value_r: current_value,
                current_value_l: other_value,
                password,
            },
        }
    }

    fn simple_add_op(self, op: Operation) -> Code {
        let simple_op = op.simplify();
        let current_value = (simple_op.value
            + match simple_op.direction {
                Direction::L => self.current_value_l,
                Direction::R => self.current_value_r,
            })
            % 100;
        let password = self.password
            + match current_value == 0 {
                true => 1,
                _ => 0,
            };
        return Code::new(password, current_value, simple_op.direction);
    }

    fn complex_add_op(self, op: Operation) -> Code {
        let new_value = op.value
            + match op.direction {
                Direction::L => self.current_value_l,
                Direction::R => self.current_value_r,
            };
        let password = self.password + new_value / 100;
        return Code::new(password, new_value % 100, op.direction);
    }
}

fn get_op(line: &str) -> Operation {
    let mut direction = Direction::R;
    if line.chars().nth(0) == Some('L') {
        direction = Direction::L;
    }
    let value: u64 = line
        .replace("L", "")
        .replace("R", "")
        .parse::<u64>()
        .unwrap();
    return Operation { direction, value };
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut code = Code::new(0, 50, Direction::L);
    for line in input.lines() {
        let op = get_op(line);
        code = code.simple_add_op(op);
    }
    Some(code.password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut code = Code::new(0, 50, Direction::L);
    for line in input.lines() {
        let op = get_op(line);
        code = code.complex_add_op(op);
    }
    Some(code.password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
