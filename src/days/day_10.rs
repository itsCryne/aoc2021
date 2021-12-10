use std::collections::HashMap;

pub(crate) fn a(input: &str) -> usize {
    let b_map = HashMap::from([
        ('(', ')'),
        ('{', '}'),
        ('[', ']'),
        ('<', '>'),
      ]);
    let v_map = HashMap::from([
        (')', 3_usize),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let mut invalid_char_count = 0;
    for line in input.lines() {
        let mut expected_stack = Vec::with_capacity(line.len()/2);
        for char in line.chars() {
            match char {
                '(' | '{' | '[' | '<' => {
                    expected_stack.push(b_map.get(&char).unwrap());
                },
                ')' | '}' | ']' | '>' => {
                    let expected = expected_stack.pop();
                    match expected {
                        None => {
                            invalid_char_count += v_map.get(&char).unwrap();
                            break;
                        },
                        Some(c) => {
                            if c != &char {
                                invalid_char_count += v_map.get(&char).unwrap();
                                break;
                            }
                        }
                    }
                },
                _ => {unreachable!()}
            }
        }
    }
    invalid_char_count
}

pub(crate) fn b(input: &str) -> usize {
    let b_map = HashMap::from([
        ('(', ')'),
        ('{', '}'),
        ('[', ']'),
        ('<', '>'),
    ]);
    let v_map = HashMap::from([
        (')', 1_usize),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);
    let mut score_list = Vec::with_capacity(100);
    for line in input.lines() {
        let mut score = 0;
        let mut expected_stack = Vec::with_capacity(line.len() / 2);
        for char in line.chars() {
            match char {
                '(' | '{' | '[' | '<' => {
                    expected_stack.push(b_map.get(&char).unwrap());
                },
                ')' | '}' | ']' | '>' => {
                    let expected = expected_stack.pop();
                    match expected {
                        None => {
                            expected_stack.clear();
                            break;
                        },
                        Some(c) => {
                            if c != &char {
                                expected_stack.clear();
                                break;
                            }
                        }
                    }
                },
                _ => { unreachable!() }
            }
        }
        if !expected_stack.is_empty() {
            for bracket in expected_stack.iter().rev() {
                score *= 5;
                score += v_map.get(bracket).unwrap();
            }
            score_list.push(score);
        }
    }
    score_list.sort_unstable();
    score_list[score_list.len()/2]
}