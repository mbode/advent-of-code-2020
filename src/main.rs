mod day_1;
mod day_2;
mod input;

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
    Ok(())
}

