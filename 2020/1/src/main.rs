use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers : Vec<i32> = contents.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect();

    {
        let (num1, num2) = find_2_numbers_that_add_up_to(numbers.clone(), 2020)?;

        let result = num1 * num2;

        println!("part1: {}", result);
    }

    {
        let (num1, num2, num3) = find_3_numbers_that_add_up_to(numbers, 2020)?;

        let result = num1 * num2 * num3;

        println!("part2: {}", result);
    }

    Ok(())
}

fn find_2_numbers_that_add_up_to(numbers : Vec<i32>, sum : i32) -> std::io::Result<(i32, i32)> {
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x + y == sum {
                return Ok((*x, *y));
            }
        }
    }
    Ok((0, 0))
}

fn find_3_numbers_that_add_up_to(numbers : Vec<i32>, sum : i32) -> std::io::Result<(i32, i32, i32)> {
    for x in numbers.iter() {
        for y in numbers.iter() {
            for z in numbers.iter() {
                if x + y + z == sum {
                    return Ok((*x, *y, *z));
                }
            }
        }
    }
    Ok((0, 0, 0))
}
