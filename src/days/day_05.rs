struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32
}

impl Line {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn get_points_of_line(&self) -> Vec<[usize; 2]> {
        if self.is_straight() {
            if self.x1 == self.x2 {
                if self.y2 > self.y1 {
                    return (self.y1..=self.y2).map(|y| [self.x1 as usize, y as usize]).collect();
                } else {
                    return (self.y2..=self.y1).map(|y| [self.x1 as usize, y as usize]).collect();
                }
            } else {
                if self.x2 > self.x1 {
                    return (self.x1..=self.x2).map(|x| [x as usize, self.y1 as usize]).collect();
                } else {
                    return (self.x2..=self.x1).map(|x| [x as usize, self.y1 as usize]).collect();
                }
            }
        } else {
            if self.x2 >= self.x1 && self.y2 >= self.y1 {
                return (self.x1..=self.x2).zip(self.y1..=self.y2).map(| point| [point.0 as usize, point.1 as usize]).collect();
            } else if self.x2 < self.x1 && self.y2 > self.y1 {
                return ((self.x2..=self.x1).rev()).zip(self.y1..=self.y2).map(|point| [point.0 as usize, point.1 as usize]).collect();
            } else if self.x2 > self.x1 && self.y2 < self.y1 {
                return (self.x1..=self.x2).zip((self.y2..=self.y1).rev()).map(|point| [point.0 as usize, point.1 as usize]).collect();
            } else if self.x2 < self.x1 && self.y2 < self.y1 {
                return ((self.x2..=self.x1).rev()).zip((self.y2..=self.y1).rev()).map(|point| [point.0 as usize, point.1 as usize]).collect();
            }
            unreachable!();
        }
    }
}

pub(crate) fn a(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let width = 1_000;
    let height = 1_000;

    let mut  therm_lines = Vec::with_capacity(1000);
    for line in lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        let p_a = points[0];
        let p_b = points[1];

        let p_a_arr: Vec<&str> = p_a.split(",").collect();
        let x1 = p_a_arr[0].parse().unwrap();
        let y1 = p_a_arr[1].parse().unwrap();

        let p_b_arr: Vec<&str> = p_b.split(",").collect();
        let x2 = p_b_arr[0].parse().unwrap();
        let y2 = p_b_arr[1].parse().unwrap();

        therm_lines.push(Line {
            x1,
            y1,
            x2,
            y2
        })
    }

    let mut field = vec![vec![0_u32; height]; width];
    let therm_lines: Vec<Line> = therm_lines.into_iter().filter(|line| line.is_straight()).collect();

    for therm_line in therm_lines {
        let points = therm_line.get_points_of_line();
        for point in points {
            field[point[1]][point[0]] += 1;
        }
    }

    let mut dangerous_spot_count = 0;
    for (_, y_row) in field.iter().enumerate() {
        for (_, val) in y_row.iter().enumerate() {
            if val > &1 {
                dangerous_spot_count += 1;
            }
        }
    }

    dangerous_spot_count
}

pub(crate) fn b(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let width = 1_000;
    let height = 1_000;

    let mut  therm_lines = Vec::with_capacity(1000);
    for line in lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        let p_a = points[0];
        let p_b = points[1];

        let p_a_arr: Vec<&str> = p_a.split(",").collect();
        let x1 = p_a_arr[0].parse().unwrap();
        let y1 = p_a_arr[1].parse().unwrap();

        let p_b_arr: Vec<&str> = p_b.split(",").collect();
        let x2 = p_b_arr[0].parse().unwrap();
        let y2 = p_b_arr[1].parse().unwrap();

        therm_lines.push(Line {
            x1,
            y1,
            x2,
            y2
        })
    }

    let mut field = vec![vec![0_u32; height]; width];
    //let therm_lines: Vec<Line> = therm_lines.into_iter().filter(|line| line.is_straight()).collect();

    for therm_line in therm_lines {
        let points = therm_line.get_points_of_line();
        for point in points {
            field[point[1]][point[0]] += 1;
        }
    }

    let mut dangerous_spot_count = 0;
    for (_, y_row) in field.iter().enumerate() {
        for (_, val) in y_row.iter().enumerate() {
            if val > &1 {
                dangerous_spot_count += 1;
            }
        }
    }

    dangerous_spot_count
}
