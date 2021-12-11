fn get_all_surrounding_fields(map: &Vec<Vec<u32>>, pos: [usize; 2]) -> Vec<[usize; 2]> {
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

    if pos[1] != 0 {
        if pos[0] != 0 {
            points.push([pos[0]-1, pos[1]-1]);
        }
        if pos[0] < rows - 1 {
            points.push([pos[0]+1, pos[1]-1]);
        }
    }

    if pos[1] < cols - 1 {
        if pos[0] != 0 {
            points.push([pos[0]-1, pos[1]+1]);
        }
        if pos[0] < rows - 1 {
            points.push([pos[0]+1, pos[1]+1]);
        }
    }
    points
}

pub(crate) fn a(input: &str) -> usize {
    let mut map: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
    let posmap = map.clone();
    let mut needs_update = Vec::with_capacity(100);
    let mut has_flashed = Vec::with_capacity(100);
    let mut flash_count = 0;

    for _ in 0..100 {
        // Step 1: Increase all energy levels by 1
        for row in map.iter_mut() {
            for octopus in row.iter_mut() {
                *octopus +=1;
            }
        }


        // Step 2: Let 'em flash
        needs_update.clear();
        has_flashed.clear();
        for (row_index, row) in map.iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                needs_update.push([row_index, col_index]);
            }
        }

        while !needs_update.is_empty() {
            for row_index in 0..map.len() {
                for col_index in 0..map[row_index].len() {
                    if needs_update.contains(&[row_index, col_index]) {
                        needs_update.remove(needs_update.iter().position(|&e| e == [row_index, col_index]).unwrap());
                        if map[row_index][col_index] > 9 && !has_flashed.contains(&[row_index, col_index]){
                            flash_count += 1;

                            has_flashed.push([row_index, col_index]);


                            let surrounding = get_all_surrounding_fields(&posmap, [row_index, col_index]);
                            for pos in surrounding {
                                map[pos[0]][pos[1]] += 1;
                                if !needs_update.contains(&pos) {
                                    needs_update.push(pos);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Step 3: Reset energy
        for pos in has_flashed.iter() {
            map[pos[0]][pos[1]] = 0;
        }
    }
    flash_count
}

pub(crate) fn b(input: &str) -> usize {
    let mut map: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();
    let posmap = map.clone();
    let mut needs_update = Vec::with_capacity(100);
    let mut has_flashed = Vec::with_capacity(100);

    let mut step = 0;
    loop {
        step += 1;

        // Step 1: Increase all energy levels by 1
        for row in map.iter_mut() {
            for octopus in row.iter_mut() {
                *octopus +=1;
            }
        }


        // Step 2: Let 'em flash
        needs_update.clear();
        has_flashed.clear();
        for (row_index, row) in map.iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                needs_update.push([row_index, col_index]);
            }
        }

        while !needs_update.is_empty() {
            for row_index in 0..map.len() {
                for col_index in 0..map[row_index].len() {
                    if needs_update.contains(&[row_index, col_index]) {
                        needs_update.remove(needs_update.iter().position(|&e| e == [row_index, col_index]).unwrap());
                        if map[row_index][col_index] > 9 && !has_flashed.contains(&[row_index, col_index]){
                            has_flashed.push([row_index, col_index]);


                            let surrounding = get_all_surrounding_fields(&posmap, [row_index, col_index]);
                            for pos in surrounding {
                                map[pos[0]][pos[1]] += 1;
                                if !needs_update.contains(&pos) {
                                    needs_update.push(pos);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Step 3: Reset energy
        for pos in has_flashed.iter() {
            map[pos[0]][pos[1]] = 0;
        }

        if has_flashed.len() == 100 {
            return step;
        }
    }
}