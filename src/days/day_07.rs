pub(crate) fn a(input: &str) -> usize {
    let mut pos_vec: Vec<i16> = input.trim().split(",").map(|p| p.parse().unwrap()).collect();
    pos_vec.sort_unstable();
    let target_pos = pos_vec[pos_vec.len() / 2];
    println!("{}", target_pos);
    pos_vec.iter().map(|p| (p-target_pos).abs()).sum::<i16>() as usize
}

pub(crate) fn b(input: &str) -> usize {
    let pos_vec: Vec<i32> = input.trim().split(",").map(|p| p.parse().unwrap()).collect();
    // I don't have __any__ mathematical proof for it - this just works for my input and ymmv.
    // However, the correct target_pos should be guaranteed to be in the (-1;1) interval around the mean.
    let target_pos = (pos_vec.iter().sum::<i32>() as f32 / pos_vec.len() as f32).floor() as i32;
    println!("{}", target_pos);
    pos_vec.iter().map(|p| (((p-target_pos).pow(2) + (p-target_pos).abs())/2).abs()).sum::<i32>() as usize
}