pub(crate) fn a(input: &str) -> i64 {
    let lines: Vec<Vec<&str>> = input.lines().map(|line| line.split(' ').collect()).collect();

    let mut depth = 0;
    let mut length = 0;

    for line in lines {
        match line[0] {
            "down" => {
                depth += line[1].parse::<i64>().unwrap();
            },
            "up" => {
                depth -= line[1].parse::<i64>().unwrap();
            },
            "forward" => {
                length += line[1].parse::<i64>().unwrap();
            }
            _ => {
                panic!("Unknown instruction {}", line[0])
            }
        }
    }

    return depth*length
}

pub(crate) fn b(input: &str) -> i64 {
    let lines: Vec<Vec<&str>> = input.lines().map(|line| line.split(' ').collect()).collect();

    let mut depth = 0;
    let mut length = 0;
    let mut aim = 0;

    for line in lines {
        match line[0] {
            "down" => {
                aim += line[1].parse::<i64>().unwrap();
            },
            "up" => {
                aim -= line[1].parse::<i64>().unwrap();
            },
            "forward" => {
                length += line[1].parse::<i64>().unwrap();
                depth += aim * line[1].parse::<i64>().unwrap();
            }
            _ => {
                panic!("Unknown instruction {}", line[0])
            }
        }
    }

    return depth*length
}