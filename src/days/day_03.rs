pub(crate) fn a(input: &str) -> i64 {
    let lines: Vec<Vec<i64>> = input.lines().collect::<Vec<&str>>().iter().map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i64).collect()).collect::<Vec<Vec<i64>>>();
    let mut columns = Vec::new();
    for i in 0..lines[1].len() {
        let column_vec: Vec<i64> = lines.iter().map(move |line| line[i]).collect();
        columns.push(column_vec)
    }

    let most_common_bits: Vec<&str> = columns.iter().map(|column| if column.iter().sum::<i64>() > lines.len() as i64 / 2 {"1"} else  {"0"}).collect();
    let least_common_bits: Vec<&str> = columns.iter().map(|column| if column.iter().sum::<i64>() < lines.len() as i64 / 2 {"1"} else  {"0"}).collect();



    let gamma_rate = i64::from_str_radix(&*most_common_bits.join(""), 2).unwrap();
    let epsilon_rate = i64::from_str_radix(&*least_common_bits.join(""), 2).unwrap();

    gamma_rate*epsilon_rate
}

pub(crate) fn b(input: &str) -> i64 {
    let lines: Vec<Vec<i64>> = input.lines().collect::<Vec<&str>>().iter().map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i64).collect()).collect::<Vec<Vec<i64>>>();
    let mut ox_lines = lines.clone();
    let mut co_lines = lines.clone();

    let mut column_index: usize = 0;

    while ox_lines.len() > 1 {
        let mut columns = Vec::new();
        for i in 0..ox_lines[1].len() {
            let column_vec: Vec<i64> = ox_lines.iter().map(move |line| line[i]).collect();
            columns.push(column_vec)
        }

        let most_common_bits: Vec<i64> = columns.iter().map(|column| if column.iter().sum::<i64>() as f64 >= (ox_lines.len() as f64 / 2.0) {1} else  {0}).collect();
        ox_lines = ox_lines.into_iter().filter(|line| line[column_index] == most_common_bits[column_index]).collect();
        column_index += 1;
    }

    let ox_gen_rating = i64::from_str_radix(&ox_lines[0].iter().map(|bit| bit.to_string()).collect::<String>(), 2).unwrap();
    column_index = 0;

    while co_lines.len() > 1 {
        let mut columns = Vec::new();
        for i in 0..co_lines[1].len() {
            let column_vec: Vec<i64> = co_lines.iter().map(move |line| line[i]).collect();
            columns.push(column_vec)
        }

        let least_common_bits: Vec<i64> = columns.iter().map(|column| if (column.iter().sum::<i64>() as f64) < (co_lines.len() as f64 / 2.0) {1} else  {0}).collect();
        co_lines = co_lines.into_iter().filter(|line| line[column_index] == least_common_bits[column_index]).collect();
        column_index += 1;
    }

    let co_scr_rating = i64::from_str_radix(&co_lines[0].iter().map(|bit| bit.to_string()).collect::<String>(), 2).unwrap();

    co_scr_rating * ox_gen_rating
}