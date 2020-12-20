#[macro_use]
extern crate lazy_static;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod input;

use std::io::Error;

fn main() -> Result<(), Error> {
    let day = 20;

    match day {
        1 => {
            let day_1 = input::read_isize_per_line("input/day_1")?;
            if let Some(i) = day_1::day_1_1(&day_1) {
                println!("day_1_1: {}", i)
            }
            if let Some(i) = day_1::day_1_2(&day_1) {
                println!("day_1_2: {}", i)
            }
        }
        2 => {
            let day_2 = input::read_lines("input/day_2")?;
            println! {"day_2_1: {}", day_2::day_2_1(&day_2)}
            println! {"day_2_2: {}", day_2::day_2_2(&day_2)}
        }
        3 => {
            let day_3 = input::read_lines("input/day_3")?;
            println! {"day_3_1: {}", day_3::day_3_1(&day_3, 3, 1)}
            println! {"day_3_2: {}", day_3::day_3_2(&day_3)}
        }
        4 => {
            let day_4 = input::read_lines("input/day_4")?;
            println! {"day_4_1: {}", day_4::day_4_1(&day_4)}
            println! {"day_4_2: {}", day_4::day_4_2(&day_4)}
        }
        5 => {
            let day_5 = input::read_lines("input/day_5")?;
            println! {"day_5_1: {}", day_5::day_5_1(&day_5)}
            if let Some(i) = day_5::day_5_2(&day_5) {
                println!("day_5_2: {}", i)
            }
        }
        6 => {
            let day_6 = input::read_lines("input/day_6")?;
            println! {"day_6_1: {}", day_6::day_6_1(&day_6)}
            println! {"day_6_2: {}", day_6::day_6_2(&day_6)}
        }
        7 => {
            let day_7 = input::read_lines("input/day_7")?;
            println! {"day_7_1: {}", day_7::day_7_1(&day_7)}
            println! {"day_7_2: {}", day_7::day_7_2(&day_7)}
        }
        8 => {
            let day_8 = input::read_lines("input/day_8")?;
            println! {"day_8_1: {}", day_8::day_8_1(&day_8)}
            println! {"day_8_2: {}", day_8::day_8_2(&day_8)}
        }
        9 => {
            let day_9 = input::read_usize_per_line("input/day_9")?;
            let day_9_1 = day_9::day_9_1(&day_9, 25).unwrap();
            println! {"day_9_1: {}", day_9_1}
            println! {"day_9_2: {}", day_9::day_9_2(&day_9, day_9_1).unwrap()}
        }
        10 => {
            let day_10 = input::read_isize_per_line("input/day_10")?;
            println! {"day_10_1: {}", day_10::day_10_1(&day_10)}
            println! {"day_10_2: {}", day_10::day_10_2(&day_10)}
        }
        11 => {
            let day_11 = input::read_lines("input/day_11")?;
            println! {"day_11_1: {}", day_11::day_11_1(&day_11)}
            println! {"day_11_2: {}", day_11::day_11_2(&day_11)}
        }
        12 => {
            let day_12 = input::read_lines("input/day_12")?;
            println! {"day_12_1: {}", day_12::day_12_1(&day_12)}
            println! {"day_12_2: {}", day_12::day_12_2(&day_12)}
        }
        13 => {
            let day_13 = input::read_lines("input/day_13")?;
            println! {"day_13_1: {}", day_13::day_13_1(&day_13)}
            println! {"day_13_2: {}", day_13::day_13_2(&day_13)}
        }
        14 => {
            let day_14 = input::read_lines("input/day_14")?;
            println! {"day_14_1: {}", day_14::day_14_1(&day_14)}
            println! {"day_14_2: {}", day_14::day_14_2(&day_14)}
        }
        15 => {
            let day_15 = input::read_usize_one_line("input/day_15")?;
            println! {"day_15_1: {}", day_15::day_15_1(&day_15)}
            println! {"day_15_2: {}", day_15::day_15_2(&day_15)}
        }
        16 => {
            let day_16 = input::read_lines("input/day_16")?;
            println! {"day_16_1: {}", day_16::day_16_1(&day_16)}
            println! {"day_16_2: {}", day_16::day_16_2(&day_16)}
        }
        17 => {
            let day_17 = input::read_lines("input/day_17")?;
            println! {"day_17_1: {}", day_17::day_17_1(&day_17)}
            println! {"day_17_2: {}", day_17::day_17_2(&day_17)}
        }
        18 => {
            let day_18 = input::read_lines("input/day_18")?;
            println! {"day_18_1: {}", day_18::day_18_1(&day_18)}
            println! {"day_18_2: {}", day_18::day_18_2(&day_18)}
        }
        19 => {
            let day_19 = input::read_lines("input/day_19")?;
            println! {"day_19_1: {}", day_19::day_19_1(&day_19)}
            println! {"day_19_2: {}", day_19::day_19_2(&day_19)}
        }
        20 => {
            let day_20 = input::read_lines("input/day_20")?;
            println! {"day_20_1: {}", day_20::day_20_1(&day_20)}
            println! {"day_20_2: {}", day_20::day_20_2(&day_20, 2593)}
        }
        _ => {}
    }

    Ok(())
}
