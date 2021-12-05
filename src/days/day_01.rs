pub(crate) fn a(input: &str) -> u32 {
    let lines = input.lines().map(|line| line.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut increase_counter = 0;
    for index in 1..lines.len() {
        if lines[index] > lines[index-1] {
            increase_counter += 1;
        }
    }
    return increase_counter;
}
pub(crate) fn b(input: &str) -> u32 {
    let lines = input.lines().map(|line| line.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut increase_counter = 0;
    for index in 3..lines.len() {
        if lines[index] > lines[index-3] {
            increase_counter += 1;
        }
    }
    return increase_counter;
}

