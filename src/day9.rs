use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};

#[cfg(test)]
use rstest::rstest;

#[derive(Debug)]
struct XmasParseError;

impl Error for XmasParseError {}
impl fmt::Display for XmasParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse instruction")
    }
}

impl std::convert::From<std::num::ParseIntError> for XmasParseError{

    fn from(_parse_int_error : std::num::ParseIntError) -> XmasParseError{
        XmasParseError{}
    }
}

#[derive(Debug, PartialEq, Clone)]
struct XMASData {
    data : Vec<u64>
}

impl FromStr for XMASData {
    type Err = XmasParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let data: Result<Vec<u64>, std::num::ParseIntError> = s.split("\n")
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>())
                .collect();
        Ok(XMASData {
            data: data?,
        })
    }
}

impl XMASData {
    fn from_input() -> Result<XMASData, Box<dyn std::error::Error>> {
        let contents = read_input(9)?;
        let data = XMASData::from_str(contents.as_str())?;
        Ok(data)
    }

    fn is_valid(&self, preamble_len : usize, i : usize) -> bool {
        //println!("checking index={}", i);
        for x in (i-preamble_len)..i {
            for y in (i-preamble_len)..i {
                if x != y {
                    if self.data[x] + self.data[y] == self.data[i] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn find_first_failure(&self, preamble_len : usize) -> Option<u64> {
        self.do_find_first_failure(preamble_len).map(|x| x.1)
    }

    fn do_find_first_failure(&self, preamble_len : usize) -> Option<(usize, u64)> {
        for i in preamble_len..self.data.len() {
            if !self.is_valid(preamble_len, i) {
                //println!("index={} is invalid", i);
                return Some((i, self.data[i]));
            }
        }
        None
    }

    fn find_weakness_range(&self, i : usize, value: u64) -> Option<Vec<u64>> {
        for x in 0..i {
            println!("==== NEW LOOP ====");
            let mut result = 0;
            for y in x..i {
                result = result + self.data[y];
                println!("current={} total={}", self.data[y], result);
                if result > value {
                    break;
                } else if result == value {
                    return Some(self.data[x..y + 1].to_vec());
                }
            }
        }
        None
    }
    fn find_encryption_weakness(&self, preamble_len : usize) -> Option<u64> {
        match self.do_find_first_failure(preamble_len) {
            Some((i, value)) => match self.find_weakness_range(i, value) {
                Some(range) => Some(range.iter().min().unwrap() + range.iter().max().unwrap()),
                None => None
            },
            None => None
        }
    }

}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = XMASData::from_input()?;
    println!("part1: data: {:?}", data.find_first_failure(25));
    println!("part2: encryption weakness: {:?}", data.find_encryption_weakness(25));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest(input, result,
        case("35\n23\n 37\n\n42", XMASData {
            data: vec![35, 23, 37, 42],
        })
    )]
    fn parse_data(input: &str, result : XMASData) -> Result<(), Box<dyn std::error::Error>> {
        let parsed = XMASData::from_str(input)?;
        assert_eq!(parsed, result);
        Ok(())
    }

    const TEST_DATA : &str = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

    #[test]
    fn find_invalid_number() -> Result<(), Box<dyn std::error::Error>> {
        let xmas_data = XMASData::from_str(TEST_DATA)?;
        let first_failure = xmas_data.find_first_failure(5);
        assert_eq!(Some(127), first_failure);
        Ok(())
    }

    #[test]
    fn part1() -> Result<(), Box<dyn std::error::Error>> {
        let xmas_data = XMASData::from_input()?;
        let first_failure = xmas_data.find_first_failure(25);
        assert_eq!(Some(248131121), first_failure);
        Ok(())
    }

    mod part2 {
        use super::*;

        #[test]
        fn encryption_weakness() -> Result<(), Box<dyn std::error::Error>> {
            let xmas_data = XMASData::from_str(TEST_DATA)?;
            let encryption_weakness = xmas_data.find_encryption_weakness(5);
            assert_eq!(Some(62), encryption_weakness);
            Ok(())
        }
    }

    #[test]
    fn part2() -> Result<(), Box<dyn std::error::Error>> {
        let xmas_data = XMASData::from_input()?;
        let encryption_weakness = xmas_data.find_encryption_weakness(25);
        assert_eq!(Some(31580383), encryption_weakness);
        Ok(())
    }

}
