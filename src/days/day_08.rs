pub(crate) fn a(input: &str) -> usize {
    input
        .lines()
        .map(|line| line
            .split(" | ")
            .last()
            .unwrap()
            .split(" ")
            .map(|word| word.len() as u16)
            .collect::<Vec<u16>>()
        ).flatten()
        .map(|dig| if [2,4,3,7].contains(&dig) {1} else {0})
        .sum()
}

pub(crate) fn b(input: &str) -> usize {
    let input_lines_first_part: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line
            .split(" | ")
            .next()
            .unwrap()
            .split(" ").collect::<Vec<&str>>()
        ).collect();
    let input_lines_second_part: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line
            .split(" | ")
            .last()
            .unwrap()
            .split(" ").collect::<Vec<&str>>()
        ).collect();

    let mut output_sum = 0_usize;

    for (input_words, output_words) in input_lines_first_part.into_iter().zip(input_lines_second_part) {
        let one = *input_words.iter().find(|word| word.len() == 2).unwrap();
        let four = *input_words.iter().find(|word| word.len() == 4).unwrap();
        let seven = *input_words.iter().find(|word| word.len() == 3).unwrap();
        let eight = *input_words.iter().find(|word| word.len() == 7).unwrap();

        let mut zero = "";
        let mut two = "";
        let mut three = "";
        let mut five = "";
        let mut six = "";
        let mut nine = "";

        let unknown: Vec<&str> = input_words.into_iter().filter(|w| ![one, four, seven, eight].contains(w)).collect();

        for word in unknown {
            match word.len() {
                5 => {
                    if one.chars().all(|c| word.contains(c)) {
                        three = word;
                    } else {
                        let five_detector = four.chars().filter(|c| !one.contains(*c)).collect::<String>();
                        if five_detector.chars().all(|c| word.contains(c)) {
                            five = word;
                        } else {
                            two = word;
                        }
                    }
                },
                6 => {
                    if four.chars().all(|c| word.contains(c)) {
                        nine = word;
                    } else if one.chars().all(|c| word.contains(c)){
                        zero = word;
                    } else {
                        six = word;
                    }
                },
                _ => unreachable!()
            }
        }
        let decoder = [zero, one, two, three, four, five, six, seven, eight, nine];
        for (exponent, display) in output_words.into_iter().rev().enumerate() {
            let num_output = decoder.iter().position(|&d| d.chars().all(|c| display.contains(c)) && display.chars().all(|c| d.contains(c))).unwrap();
            output_sum +=  10_usize.pow(exponent as u32) * num_output;
        }
    }
    output_sum
}