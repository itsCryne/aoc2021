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
use days::*;
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
    let day_vec = vec![
        [day_01::a, day_01::b],
        [day_02::a, day_02::b],
        [day_03::a, day_03::b]
    ];

    let mut cum_time: f64 = 0.0;

    for (day, tasks) in day_vec.iter().enumerate() {
        let input = get_puzzle_input((day+1) as i8).await;
        let a_start_time = Instant::now();
        let a_result = tasks[0](&input);
        let a_end_time = a_start_time.elapsed().as_secs_f64() * 1000.0;

        let b_start_time = Instant::now();
        let b_result = tasks[1](&input);
        let b_end_time = b_start_time.elapsed().as_secs_f64() * 1000.0;

        cum_time += a_end_time + b_end_time;

        println!("Day {:02}:\n A: {} in {}\n B: {} in {}", day+1,  a_result, fmt_time(a_end_time), b_result, fmt_time(b_end_time));
    }

    println!("----------");
    println!("Cumulative time: {}", fmt_time(cum_time));
}
