struct BingoSheet {
    rows: Vec<Vec<i32>>,
    columns: Vec<Vec<i32>>,
    numbers_called_in_row: Vec<i32>,
    numbers_called_in_column: Vec<i32>,
    last_number_called: i32,
    called_numbers: Vec<i32>
}

impl BingoSheet {
    fn update(&mut self, number_called: i32) {
        for (index, row) in self.rows.iter().enumerate() {
            if row.contains(&number_called) {
                self.numbers_called_in_row[index] += 1;
                self.called_numbers.push(number_called);
            }
        }

        for (index, col) in self.columns.iter().enumerate() {
            if col.contains(&number_called) {
                self.numbers_called_in_column[index] += 1;
            }
        }

        self.last_number_called = number_called;
    }

    fn get_result(&self) -> Option<i64> {
        let row =  match self.numbers_called_in_row.iter().position(|count| count == &5) {
            Some(_) => {
                let mut rsum: i32 = 0;
                for row in &self.rows {
                    rsum += row.iter().sum::<i32>();
                }
                for num in &self.called_numbers {
                    rsum -= num;
                }
                Some((rsum * self.last_number_called) as i64)
            },
            None => None
        };

        if let Some(res) = row {
            return Some(res);
        } else {
            return match self.numbers_called_in_column.iter().position(|count| count == &5) {
                Some(_) => {
                    let mut rsum: i32 = 0;
                    for row in &self.rows {
                        rsum += row.iter().sum::<i32>();
                    }
                    for num in &self.called_numbers {
                        rsum -= num;
                    }
                    Some((rsum * self.last_number_called) as i64)
                },
                None => None
            };
        }
    }
}

pub(crate) fn a(input: &str) -> i64 {
    let mut bingo_lines: Vec<&str> = input.split("\n\n").collect();
    let chosen_numbers: Vec<i32> = bingo_lines[0].split(",").map(|num| num.parse().unwrap()).collect();

    let mut sheets: Vec<BingoSheet> = Vec::new();
    bingo_lines.remove(0);

    for sheet in bingo_lines {
        let rows: Vec<Vec<i32>> = sheet.split("\n")
            .map(|row| row.split(" ").filter(|splitres| splitres != &"")
                .map(|num| num.parse().unwrap()).collect())
            .collect();

        let mut columns: Vec<Vec<i32>> = vec![Vec::new(); rows.len()];
        for row in &rows {
            for (index, num) in row.iter().enumerate() {
                columns[index].push(*num);
            }
        }

        let rlen = rows.len();
        let clen = rows.len();
        sheets.push(BingoSheet{
            rows,
            columns,
            numbers_called_in_row: vec![0; rlen],
            numbers_called_in_column: vec![0; clen],
            last_number_called: 0,
            called_numbers: vec![]
        })
    }

    for num in chosen_numbers {
        for sheet in &mut sheets {
            sheet.update(num);
            if let Some(result) = sheet.get_result() {
                return result;
            }
        }
    }
    unreachable!();
}

pub(crate) fn b(input: &str) -> i64 {
    let mut bingo_lines: Vec<&str> = input.split("\n\n").collect();
    let chosen_numbers: Vec<i32> = bingo_lines[0].split(",").map(|num| num.parse().unwrap()).collect();

    let mut sheets: Vec<BingoSheet> = Vec::new();
    bingo_lines.remove(0);

    for sheet in bingo_lines {
        let rows: Vec<Vec<i32>> = sheet.split("\n")
            .map(|row| row.split(" ").filter(|splitres| splitres != &"")
                .map(|num| num.parse().unwrap()).collect())
            .collect();

        let mut columns: Vec<Vec<i32>> = vec![Vec::new(); rows.len()];
        for row in &rows {
            for (index, num) in row.iter().enumerate() {
                columns[index].push(*num);
            }
        }

        let rlen = rows.len();
        let clen = rows.len();
        sheets.push(BingoSheet{
            rows,
            columns,
            numbers_called_in_row: vec![0; rlen],
            numbers_called_in_column: vec![0; clen],
            last_number_called: 0,
            called_numbers: vec![]
        })
    }

    let mut won_boards = Vec::new();
    for num in chosen_numbers {
        let mut won_sheets_index = Vec::new();
        for (index, sheet) in sheets.iter_mut().enumerate() {
            sheet.update(num);
            if let Some(result) = sheet.get_result() {
                if result != 0 && !won_boards.contains(&result) {
                    won_boards.push(result);
                    won_sheets_index.push(index);
                }
            }
        }
        won_sheets_index.sort();
        won_sheets_index.iter().rev().for_each(|index| {sheets.remove(*index);});
        won_sheets_index.clear();
    }
    return won_boards.pop().unwrap();
}