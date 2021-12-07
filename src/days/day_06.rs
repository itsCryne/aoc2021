pub(crate) fn a(input: &str) -> usize {
    let mut fish_array = [0_usize; 9];
    input.as_bytes().iter().step_by(2).for_each(|fish| fish_array[(*fish - 48) as usize] += 1);

    for _ in 0..80 {
        fish_array.rotate_left(1);
        fish_array[6] += fish_array[8];
    }
    fish_array.iter().sum::<usize>()
}

pub(crate) fn b(input: &str) -> usize {
    let mut fish_array = [0_usize; 9];
    input.as_bytes().iter().step_by(2).for_each(|fish| fish_array[(*fish - 48) as usize] += 1);

    for _ in 0..256 {
        fish_array.rotate_left(1);
        fish_array[6] += fish_array[8];
    }
    fish_array.iter().sum::<usize>()
}