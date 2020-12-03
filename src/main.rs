mod inputs;

mod day1;
mod day2;
mod day3;

macro_rules! run_day {
    ($input:ident) => {
        println!("{} start {} {}", "=".repeat(10), stringify!($input), "=".repeat(10));
        $input::main()?;
        println!("{} end {} {}", "=".repeat(10), stringify!($input), "=".repeat(10));
    }
}
fn main() -> std::io::Result<()> {
    run_day!(day1);
    run_day!(day2);
    run_day!(day3);

    Ok(())
}
