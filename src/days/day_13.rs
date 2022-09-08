use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Debug)]
enum Fold {
    XFold(u32),
    YFold(u32)
}

pub(crate) fn a(input: &str) -> usize {
    let splitres = input.split("\n\n").collect::<Vec<&str>>();

    let mut points = splitres.first().unwrap().lines()
        .map(|l| Point {x: l.split(",").next().unwrap().parse().unwrap(), y: l.split(",").last().unwrap().parse().unwrap()})
        .collect::<HashSet<Point>>();
    let instructions = splitres.last().unwrap().lines()
        .map(|l| {
            if l.split(" ").last().unwrap().starts_with("x") {
                Fold::XFold(l.split("=").last().unwrap().parse().unwrap())
            } else {
                Fold::YFold(l.split("=").last().unwrap().parse().unwrap())
            }
        })
        .collect::<Vec<Fold>>();
    
    for fold in instructions {
        match fold {
            Fold::XFold(x_line) => {
                points = points.into_iter().map(|p| {
                    if p.x < x_line {
                        p
                    } else {
                        Point {
                            x: 2 * x_line - p.x,
                            y: p.y
                        }
                    }
                })
                .collect();
            }
            Fold::YFold(y_line) => {
                points = points.into_iter().map(|p| {
                    if p.y < y_line {
                        p
                    } else {
                        Point {
                            x: p.x,
                            y: 2 * y_line - p.y
                        }
                    }
                })
                .collect();
            },
        }
        break;
    }

    points.len()
}

pub(crate) fn b(input: &str) -> usize {
    let splitres = input.split("\n\n").collect::<Vec<&str>>();

    let mut points = splitres.first().unwrap().lines()
        .map(|l| Point {x: l.split(",").next().unwrap().parse().unwrap(), y: l.split(",").last().unwrap().parse().unwrap()})
        .collect::<HashSet<Point>>();
    let instructions = splitres.last().unwrap().lines()
        .map(|l| {
            if l.split(" ").last().unwrap().starts_with("x") {
                Fold::XFold(l.split("=").last().unwrap().parse().unwrap())
            } else {
                Fold::YFold(l.split("=").last().unwrap().parse().unwrap())
            }
        })
        .collect::<Vec<Fold>>();
    
    for fold in instructions {
        match fold {
            Fold::XFold(x_line) => {
                points = points.into_iter().map(|p| {
                    if p.x < x_line {
                        p
                    } else {
                        Point {
                            x: 2 * x_line - p.x,
                            y: p.y
                        }
                    }
                })
                .collect();
            }
            Fold::YFold(y_line) => {
                points = points.into_iter().map(|p| {
                    if p.y < y_line {
                        p
                    } else {
                        Point {
                            x: p.x,
                            y: 2 * y_line - p.y
                        }
                    }
                })
                .collect();
            },
        }
    }

    for y in 0..6 {
        for x in 0..39 {
            if points.contains(&Point{x, y}) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!();

    points.len()
}