pub(crate) fn a(input: &str) -> usize {
    let fishs = input.trim().split(",").map(|f| f.parse().unwrap()).collect::<Vec<usize>>();

    let mut fish_array = [0_usize; 9];
    for fish in fishs {
        fish_array[fish] += 1
    }

    for _ in 0..80 {
        let mut new_fish_array = [0_usize; 9];
        for (age, fishgroup) in fish_array.into_iter().enumerate() {
            match age {
                0 => {
                    new_fish_array[6] += fishgroup;
                    new_fish_array[8] += fishgroup;
                },
                1..=8 => {
                    new_fish_array[age-1] += fishgroup;
                }
                _ => {
                    unreachable!()
                }
            }
        }
        fish_array = new_fish_array;
    }
    fish_array.iter().sum::<usize>()
}

pub(crate) fn b(input: &str) -> usize {
    let fishs = input.trim().split(",").map(|f| f.parse().unwrap()).collect::<Vec<usize>>();

    let mut fish_array = [0_usize; 9];
    for fish in fishs {
        fish_array[fish] += 1
    }

    for _ in 0..256 {
        let mut new_fish_array = [0_usize; 9];
        for (age, fishgroup) in fish_array.into_iter().enumerate() {
            match age {
                0 => {
                    new_fish_array[6] += fishgroup;
                    new_fish_array[8] += fishgroup;
                },
                1..=8 => {
                    new_fish_array[age-1] += fishgroup;
                }
                _ => {
                    unreachable!()
                }
            }
        }
        fish_array = new_fish_array;
    }
    fish_array.iter().sum::<usize>()
}