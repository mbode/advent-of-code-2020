#[macro_use] extern crate lazy_static;

mod input;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

use std::io::Error;


fn main() -> Result<(), Error> {
    let day_1 = input::read_numbers("input/day_1")?;
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

    Ok(())
}

