mod inputs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

extern crate paste;

macro_rules! run_day {
    ($input:ident) => {
        println!("{} start {} {}", "=".repeat(10), stringify!($input), "=".repeat(10));
        $input::main()?;
        println!("{} end {} {}", "=".repeat(10), stringify!($input), "=".repeat(10));
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_day!(day1);
    run_day!(day2);
    run_day!(day3);
    run_day!(day4);
    run_day!(day5);
    run_day!(day6);
    run_day!(day7);
    run_day!(day8);

    Ok(())
}
