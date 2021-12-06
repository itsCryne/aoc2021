pub(crate) fn a(input: &str) -> usize {
    let lines = input.lines().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut increase_counter = 0;
    for index in 1..lines.len() {
        if lines[index] > lines[index-1] {
            increase_counter += 1;
        }
    }
    return increase_counter;
}
pub(crate) fn b(input: &str) -> usize {
    let lines = input.lines().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut increase_counter = 0;
    for index in 3..lines.len() {
        if lines[index] > lines[index-3] {
            increase_counter += 1;
        }
    }
    return increase_counter;
}

