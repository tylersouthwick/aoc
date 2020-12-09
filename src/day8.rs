use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashSet;
use std::convert::TryInto;

#[cfg(test)]
use rstest::rstest;

#[derive(Debug)]
struct InstructionParseError;

impl Error for InstructionParseError{}
impl fmt::Display for InstructionParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse instruction")
    }
}

impl std::convert::From<std::num::ParseIntError> for InstructionParseError {

    fn from(_parse_int_error : std::num::ParseIntError) -> InstructionParseError {
        InstructionParseError{}
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    NOP,
    ACC {
        value : i32,
    },
    JMP {
        offset : i32,
    },
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.trim().split(" ").collect();
        match tokens[0] {
            "nop" => Ok(Instruction::NOP),
            "acc" => Ok(Instruction::ACC{
                value: tokens[1].parse::<i32>()?,
            }),
            "jmp" => Ok(Instruction::JMP{
                offset: tokens[1].parse::<i32>()?,
            }),
            _ => Err(InstructionParseError{})
        }
    }
}

#[derive(Debug, PartialEq)]
struct GameConsole {
    instructions: Vec<Instruction>
}

impl FromStr for GameConsole {
    type Err = InstructionParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let instructions: Result<Vec<Instruction>, Self::Err> = s.split("\n")
                .filter(|x| !x.is_empty())
                .map(Instruction::from_str)
                .collect();
        Ok(GameConsole {
            instructions: instructions?
        })
    }
}

#[derive(Debug)]
enum GameConsoleExecutionError {
    InfiniteLoopError { last_accumulator : i32 },
}

impl Error for GameConsoleExecutionError{}
impl fmt::Display for GameConsoleExecutionError{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to play game")
    }
}

impl GameConsole {
    fn from_input() -> Result<GameConsole, Box<dyn std::error::Error>> {
        let contents = read_input(8)?;
        let game_console = GameConsole::from_str(contents.as_str())?;
        Ok(game_console)
    }

    fn execute(&self) -> Result<usize, GameConsoleExecutionError>  {
        let mut accumulator : i32 = 0;
        let mut indexes_visited : HashSet<usize> = HashSet::new();
        let mut ip : usize = 0;

        loop {
            //println!("checking ip={}", ip);
            if indexes_visited.contains(&ip) {
                return Err(GameConsoleExecutionError::InfiniteLoopError {last_accumulator: accumulator});
            }
            indexes_visited.insert(ip);
            match self.instructions[ip] {
                Instruction::ACC{value} => {
                    //println!("ip={} updating accumulator from={} to={}", ip, accumulator, accumulator + value);
                    accumulator = accumulator + value;
                    ip = ip + 1;
                },
                Instruction::JMP{offset} => {
                    if offset < 0 {
                        let as_usize : usize = (offset * -1).try_into().unwrap();
                        ip = ip - as_usize;
                    } else {
                        let as_usize : usize = offset.try_into().unwrap();
                        ip = ip + as_usize;
                    }
                },
                Instruction::NOP => {
                    ip = ip + 1;
                }
            }
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let game_console = GameConsole::from_input()?;
    println!("part1: execute game: {:?}", game_console.execute());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest(rule, result,
        case("nop +0", Instruction::NOP),
        case("acc +1", Instruction::ACC{value: 1}),
        case("acc -1", Instruction::ACC{value: -1}),
        case("jmp +1", Instruction::JMP{offset: 1}),
        case("jmp -1", Instruction::JMP{offset: -1}),
    )]
    fn parse_instruction(rule: &str, result : Instruction) -> Result<(), InstructionParseError> {
        let parsed = Instruction::from_str(rule)?;
        assert_eq!(parsed, result);
        Ok(())
    }

    #[test]
    fn parse_instructions() -> Result<(), InstructionParseError> {
        let input = "nop +0\nacc +1\njmp +2";
        let expected = vec![
            Instruction::NOP,
            Instruction::ACC{value: 1},
            Instruction::JMP{offset: 2},
        ];
        let parsed = GameConsole::from_str(input)?;
        assert_eq!(expected, parsed.instructions);
        Ok(())
    }

    use Instruction::*;

    #[test]
    fn infinite_loop() -> Result<(), String> {
        let game_console = GameConsole {
            instructions: vec![
                NOP,
                ACC {value: 1},
                JMP {offset: 4},
                ACC{ value: 3},
                JMP {offset: -3},
                ACC{ value: -99},
                ACC{ value: 1},
                JMP {offset: -4},
                ACC{ value: 6},
            ]
        };

        match game_console.execute() {
            Err(GameConsoleExecutionError::InfiniteLoopError{last_accumulator}) => {
                assert_eq!(last_accumulator, 5);
                Ok(())
            }
            _ => Err("should be infinite loop".to_string()),
        }
    }

    #[test]
    fn part1() -> Result<(), String> {
        let game_console = GameConsole::from_input().unwrap();

        match game_console.execute() {
            Err(GameConsoleExecutionError::InfiniteLoopError{last_accumulator}) => {
                assert_eq!(last_accumulator, 2058);
                Ok(())
            }
            _ => Err("should be infinite loop".to_string()),
        }
    }

}
