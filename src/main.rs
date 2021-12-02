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
use days::{day_01, day_02};
use std::time::Instant;
use crate::days::util::get_puzzle_input;

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

#[tokio::main]
async fn main() {
    let one_input = get_puzzle_input(1).await;

    let one_a_start_time = Instant::now();
    let one_a_result = day_01::a(&one_input);
    let one_a_end_time = one_a_start_time.elapsed().as_secs_f64() * 1000.0;

    let one_b_start_time = Instant::now();
    let one_b_result = day_01::b(&one_input);
    let one_b_end_time = one_b_start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Day 1:\n A: {} in {}\n B: {} in {}", one_a_result, fmt_time(one_a_end_time), one_b_result, fmt_time(one_b_end_time));

    
    let two_input = get_puzzle_input(2).await;

    let two_a_start_time = Instant::now();
    let two_a_result = day_02::a(&two_input);
    let two_a_end_time = two_a_start_time.elapsed().as_secs_f64() * 1000.0;

    let two_b_start_time = Instant::now();
    let two_b_result = day_02::b(&two_input);
    let two_b_end_time = two_b_start_time.elapsed().as_secs_f64() * 1000.0;

    println!("Day 2:\n A: {} in {}\n B: {} in {}", two_a_result, fmt_time(two_a_end_time), two_b_result, fmt_time(two_b_end_time));
}
