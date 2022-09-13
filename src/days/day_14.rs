use std::{collections::HashMap, iter::FromIterator};

pub(crate) fn a(input: &str) -> usize {
    let mut lines = input.lines();

    let mut template = Vec::<char>::from_iter(lines.next().unwrap().chars());
    lines.next();
    let mut insertions = HashMap::new();
    lines.for_each(|l| {insertions.insert(l.split(" -> ").next().unwrap(), l.split(" -> ").last().unwrap()); });

    for _ in 0..10 {
        let mut new_template = vec![template[0]];
        for window in template.clone().windows(2) {
            if let Some(insertion) = insertions.get(window.iter().collect::<String>().as_str()) {
                new_template.push(insertion.chars().next().unwrap());
            }
            new_template.push(window[1]);
        }
        template = new_template;
    }

    let mut template_map: HashMap<String, i32> = HashMap::new();

    for w in template.windows(2) {
        *template_map.entry(w.iter().collect()).or_insert(0) += 1;
    }


    let mut counts = HashMap::new();
    for c in template {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub(crate) fn b(input: &str) -> usize {
    let mut lines = input.lines();

    let mut combinations = HashMap::new();
    let template_line = lines.next().unwrap();

    let first_char = template_line.chars().next().unwrap();
    let last_char = template_line.chars().last().unwrap();


    for c in template_line.chars().collect::<Vec<char>>().windows(2) {
        *combinations.entry(c.into_iter().collect::<String>()).or_insert(0) += 1_usize;
    }
    
    let mut insertion_rules = HashMap::new();

    lines.next();

    for l in lines {
        let mut linesplit = l.split(" -> ");
        insertion_rules.insert(linesplit.next().unwrap().to_string(), linesplit.next().unwrap().to_string());
    }

    for _ in 0..40 {
        let mut added_polymers = HashMap::new();
        for (bond, count) in combinations.iter_mut() {
            if let Some(ins) = insertion_rules.get(bond) {
                *added_polymers.entry(format!("{}{}", bond.chars().next().unwrap(), ins)).or_insert(0) += *count;
                *added_polymers.entry(format!("{}{}", ins, bond.chars().last().unwrap())).or_insert(0) += *count;
                *count = 0
            }
        }
        combinations.extend(added_polymers.into_iter());
    }

    let mut letter_counts_raw = HashMap::new();
    for (bond, count) in combinations {
        for c in bond.chars() {
            *letter_counts_raw.entry(c).or_insert(0) += count;
        }
    }

    *letter_counts_raw.entry(last_char).or_insert(0) += 1;
    *letter_counts_raw.entry(first_char).or_insert(0) += 1;

    let letter_counts = letter_counts_raw.into_iter().map(|(k, v)| { (k, v / 2)}).collect::<HashMap<char, usize>>();

    letter_counts.values().max().unwrap() - letter_counts.values().min().unwrap()
}