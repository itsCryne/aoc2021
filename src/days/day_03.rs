pub(crate) fn a(input: &str) -> i64 {
    let lines: Vec<Vec<i64>> = input.lines().collect::<Vec<&str>>().iter().map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i64).collect()).collect::<Vec<Vec<i64>>>();

    let mut most_common_bits = Vec::new();
    for i in 0..lines[0].len(){
        most_common_bits.push(if lines.iter().map(|line| line[i]).sum::<i64>() as f64 >= (lines.len() as f64 / 2.0) {"1"} else  {"0"});
    }

    let gamma_rate = i64::from_str_radix(&*most_common_bits.join(""), 2).unwrap();
    let epsilon_rate = 0b111111111111 ^ gamma_rate;

    gamma_rate*(epsilon_rate as i64)
}

pub(crate) fn b(input: &str) -> i64 {
    let lines: Vec<Vec<i64>> = input.lines().collect::<Vec<&str>>().iter().map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i64).collect()).collect::<Vec<Vec<i64>>>();
    let mut ox_lines = lines.clone();
    let mut co_lines = lines.clone();

    let mut column_index: usize = 0;

    while ox_lines.len() > 1 {
        let most_common_bit_at_index = if ox_lines.iter().map(|line| line[column_index]).sum::<i64>() as f64 >= (ox_lines.len() as f64 / 2.0) {1} else  {0};
        ox_lines = ox_lines.into_iter().filter(|line| line[column_index] == most_common_bit_at_index).collect();
        column_index += 1;
    }

    let ox_gen_rating = i64::from_str_radix(&ox_lines[0].iter().map(|bit| bit.to_string()).collect::<String>(), 2).unwrap();
    column_index = 0;

    while co_lines.len() > 1 {
        let least_common_bit_at_index = if (co_lines.iter().map(|line| line[column_index]).sum::<i64>() as f64) < (co_lines.len() as f64 / 2.0) {1} else  {0};
        co_lines = co_lines.into_iter().filter(|line| line[column_index] == least_common_bit_at_index).collect();
        column_index += 1;
    }

    let co_scr_rating = i64::from_str_radix(&co_lines[0].iter().map(|bit| bit.to_string()).collect::<String>(), 2).unwrap();

    co_scr_rating * ox_gen_rating
}