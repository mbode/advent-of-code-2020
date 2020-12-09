#[macro_use]
extern crate lazy_static;

mod day_1;
mod day_2;
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
    let day_1 = input::read_isize("input/day_1")?;
    if let Some(i) = day_1::day_1_1(&day_1) {
        println!("day_1_1: {}", i)
    }
    if let Some(i) = day_1::day_1_2(&day_1) {
        println!("day_1_2: {}", i)
    }

    let day_2 = input::read_lines("input/day_2")?;
    println! {"day_2_1: {}", day_2::day_2_1(&day_2)}
    println! {"day_2_2: {}", day_2::day_2_2(&day_2)}

    let day_3 = input::read_lines("input/day_3")?;
    println! {"day_3_1: {}", day_3::day_3_1(&day_3, 3, 1)}
    println! {"day_3_2: {}", day_3::day_3_2(&day_3)}

    let day_4 = input::read_lines("input/day_4")?;
    println! {"day_4_1: {}", day_4::day_4_1(&day_4)}
    println! {"day_4_2: {}", day_4::day_4_2(&day_4)}

    let day_5 = input::read_lines("input/day_5")?;
    println! {"day_5_1: {}", day_5::day_5_1(&day_5)}
    if let Some(i) = day_5::day_5_2(&day_5) {
        println!("day_5_2: {}", i)
    }

    let day_6 = input::read_lines("input/day_6")?;
    println! {"day_6_1: {}", day_6::day_6_1(&day_6)}
    println! {"day_6_2: {}", day_6::day_6_2(&day_6)}

    let day_7 = input::read_lines("input/day_7")?;
    println! {"day_7_1: {}", day_7::day_7_1(&day_7)}
    println! {"day_7_2: {}", day_7::day_7_2(&day_7)}

    let day_8 = input::read_lines("input/day_8")?;
    println! {"day_8_1: {}", day_8::day_8_1(&day_8)}
    println! {"day_8_2: {}", day_8::day_8_2(&day_8)}

    let day_9 = input::read_usize("input/day_9")?;
    let day_9_1 = day_9::day_9_1(&day_9, 25).unwrap();
    println! {"day_9_1: {}", day_9_1}
    println! {"day_9_2: {}", day_9::day_9_2(&day_9, day_9_1).unwrap()}

    Ok(())
}
