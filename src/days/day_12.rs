use std::{collections::HashSet, iter::FromIterator};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Cave {
    BigCave(String),
    SmallCave(String)
}


#[derive(PartialEq, Eq, Hash, Clone)]

struct Connection {
    caves: [Cave; 2]
}


#[derive(PartialEq, Eq, Clone)]

struct CaveSystem {
    connections: Vec<Connection>,
    caves: HashSet<Cave>
}

impl CaveSystem {
    fn get_connections_to(&self, cave: &Cave) -> HashSet<Cave> {
        let mut caves = HashSet::new();
        for conn in self.connections.iter() {
            if conn.caves.contains(cave) {
                caves.insert(conn.caves.iter().filter(|&c| c != cave).next().unwrap().to_owned());
            }
        }
        caves
    }

    fn visited_small_twice(&self, caves: &Vec<Cave>) -> bool {
        let small_caves = caves.into_iter().filter(|c| if let Cave::SmallCave(_) = c {true} else {false}).collect::<Vec<&Cave>>();
        !(small_caves.len() == HashSet::<&Cave>::from_iter(small_caves).len())
    }

    fn traverse(&self, caves: Vec<Cave>, counter: &mut usize, allow_twice: bool) {
        let mut connections = self.get_connections_to(caves.last().unwrap());
        if caves.len() >= 2 {
            if let Some(last_visited) = caves.get(caves.len() - 1) {
                connections.remove(last_visited);
            }
        }

        for new_cave in connections.iter() {
            match new_cave.clone() {
                Cave::BigCave(_) => {
                    let mut new_caves = caves.clone();
                    new_caves.push(new_cave.clone());

                    self.traverse(new_caves, counter, allow_twice);
                },
                Cave::SmallCave(sc) => {
                    if caves.contains(&new_cave) {
                        if new_cave == &Cave::SmallCave(String::from("start")) || new_cave == &Cave::SmallCave(String::from("end")) {
                            continue;
                        }

                        if allow_twice {
                            if self.visited_small_twice(&caves) {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }

                    if sc == String::from("end") {
                        *counter += 1;
                    } else {
                        let mut new_caves = caves.clone();
                        new_caves.push(new_cave.clone());

                        self.traverse(new_caves, counter, allow_twice);
                    }
                },
            }
        }

    }
}

pub(crate) fn a(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let mut connections: Vec<Connection> = Vec::new();
    let mut caves: HashSet<Cave> = HashSet::new();

    for line in lines {
        let string_caves = line.split('-').collect::<Vec<&str>>();

        let cave_one = if string_caves[0].to_ascii_lowercase() == string_caves[0] {
            Cave::SmallCave(string_caves[0].to_string())
        } else {
            Cave::BigCave(string_caves[0].to_string())
        };
        let cave_two = if string_caves[1].to_ascii_lowercase() == string_caves[1] {
            Cave::SmallCave(string_caves[1].to_string())
        } else {
            Cave::BigCave(string_caves[1].to_string())
        };

        caves.insert(cave_one.clone());
        caves.insert(cave_two.clone());

        connections.push(Connection {
            caves: [cave_one, cave_two]
        });
    }



    let system = CaveSystem {
        connections,
        caves,
    };

    let mut counter = 0;
    system.traverse(vec![Cave::SmallCave(String::from("start"))], &mut counter, false);

    counter
}

pub(crate) fn b(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let mut connections: Vec<Connection> = Vec::new();
    let mut caves: HashSet<Cave> = HashSet::new();

    for line in lines {
        let string_caves = line.split('-').collect::<Vec<&str>>();

        let cave_one = if string_caves[0].to_ascii_lowercase() == string_caves[0] {
            Cave::SmallCave(string_caves[0].to_string())
        } else {
            Cave::BigCave(string_caves[0].to_string())
        };
        let cave_two = if string_caves[1].to_ascii_lowercase() == string_caves[1] {
            Cave::SmallCave(string_caves[1].to_string())
        } else {
            Cave::BigCave(string_caves[1].to_string())
        };

        caves.insert(cave_one.clone());
        caves.insert(cave_two.clone());

        connections.push(Connection {
            caves: [cave_one, cave_two]
        });
    }



    let system = CaveSystem {
        connections,
        caves,
    };

    let mut counter = 0;
    system.traverse(vec![Cave::SmallCave(String::from("start"))], &mut counter, true);

    counter
}