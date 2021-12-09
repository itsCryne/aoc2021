use std::collections::HashMap;

pub fn get_surrounding_fields(map: &Vec<Vec<u32>>, pos: [usize; 2]) -> Vec<[usize; 2]> {
    let cols = map.len();
    let rows = map[0].len();

    let mut points = Vec::new();
    if pos[1] != 0 {
        points.push([pos[0], pos[1]-1]);
    }
    if pos[1] < cols - 1 {
        points.push([pos[0], pos[1]+1]);
    }
    if pos[0] != 0 {
        points.push([pos[0]-1, pos[1]]);
    }
    if pos[0] < rows - 1 {
        points.push([pos[0]+1, pos[1]]);
    }
    points
}

pub(crate) fn a(input: &str) -> usize {
    let map: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();

    let mut low_points = Vec::new();

    for (col_index, row) in map.iter().enumerate() {
        for (row_index, col) in row.iter().enumerate() {
            if get_surrounding_fields(&map, [row_index, col_index]).iter().all(|cp| map[cp[1]][cp[0]] > *col) {
                low_points.push(col+1);
            }
        }
    }
    low_points.iter().sum::<u32>() as usize
}

fn get_low_point(map: &Vec<Vec<u32>>, pos: [usize; 2]) -> [usize; 2] {
    let surrounding_fields_with_vals: Vec<(u32, [usize; 2])> = get_surrounding_fields(&map, [pos[0], pos[1]]).iter().map(|cp| (map[cp[1]][cp[0]], *cp)).collect();
    let goto = surrounding_fields_with_vals.iter().map(|fv| fv.0).min().unwrap();
    if goto >= map[pos[1]][pos[0]] {
        return pos
    } else {
        get_low_point(map, surrounding_fields_with_vals.iter().find(|e| e.0 == goto).unwrap().1)
    }
}

pub(crate) fn b(input: &str) -> usize {
    let map: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();

    let mut low_points = Vec::new();

    for (col_index, row) in map.iter().enumerate() {
        for (row_index, col) in row.iter().enumerate() {
            if get_surrounding_fields(&map, [row_index, col_index]).iter().all(|cp| map[cp[1]][cp[0]] > *col) {
                low_points.push(([row_index, col_index], 0));
            }
        }
    }

    let mut basin_sizes: HashMap<[usize; 2], u32> = low_points.into_iter().collect();

    for (col_index, row) in map.iter().enumerate() {
        for (row_index, col) in row.iter().enumerate() {
            if col == &9 {continue;}
            let lp = get_low_point(&map, [row_index, col_index]);
            basin_sizes.insert(lp, basin_sizes.get(&lp).unwrap() + 1);
        }
    }
    let mut basin_size_vec: Vec<&u32> = basin_sizes.values().collect();
    basin_size_vec.sort_unstable();
    let mut basin_size_vec_iter_rev = basin_size_vec.into_iter().rev();
    let (a, b,c) = (basin_size_vec_iter_rev.next().unwrap(), basin_size_vec_iter_rev.next().unwrap(), basin_size_vec_iter_rev.next().unwrap());
    (a * b * c) as usize
}