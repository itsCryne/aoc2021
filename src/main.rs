/*
    aoc2021 - solutions for the "Advent of Code 2021" in rust
    Copyright (C) 2021 itsCryne <cryne@gmx.de>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod days;
mod util;

use days::*;
use std::time::Instant;
use util::get_puzzle_input;

fn fmt_time(ns: u128) -> String {
    let s = ns / 1_000_000_000;
    let ms = ns / 1_000_000 - s * 1_000;
    let mu_s = ns / 1_000 - ms * 1_000;
    let parsed_ns = ns - mu_s * 1_000 - ms * 1_000_000 - s * 1_000_000_000;

    let mut output = format!("{}s {}ms {}µs {}ns", s, ms, mu_s, parsed_ns);
    for char in output.clone().chars() {
        if char == '0' || char == ' ' || char.is_alphabetic() {
            output.remove(0);
        } else {
            break;
        }
    }
    output
}

#[tokio::main]
async fn main() {
    let day_vec = vec![
        [day_01::a, day_01::b],
        [day_02::a, day_02::b],
        [day_03::a, day_03::b]
    ];

    let mut cum_time: u128 = 0;

    for (day, tasks) in day_vec.iter().enumerate() {
        let input = get_puzzle_input((day+1) as i8).await;
        let a_start_time = Instant::now();
        let a_result = tasks[0](&input);
        let a_end_time = a_start_time.elapsed().as_nanos();

        let b_start_time = Instant::now();
        let b_result = tasks[1](&input);
        let b_end_time = b_start_time.elapsed().as_nanos();

        cum_time += a_end_time + b_end_time;

        println!("Day {:02}:\n A: {} in {}\n B: {} in {}", day+1,  a_result, fmt_time(a_end_time), b_result, fmt_time(b_end_time));
    }

    println!("----------");
    println!("Cumulative time: {}", fmt_time(cum_time));
}
