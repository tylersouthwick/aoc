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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    NOP {
        value : i32
    },
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
            "nop" => Ok(Instruction::NOP {
                value: tokens[1].parse::<i32>()?,
            }),
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

#[derive(Debug, PartialEq, Clone)]
struct GameConsole {
    instructions: Vec<Instruction>
}

impl FromStr for GameConsole {
    type Err = InstructionParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let instructions: Result<Vec<Instruction>, Self::Err> = s.split("\n")
                .map(str::trim)
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
    InstructionPointerExceedsProgram,
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

    fn fix(&self) -> Self {
        for i in 0..self.instructions.len() {
            let new_game = self.swap(i);
            let is_fixed = match new_game.execute() {
                Ok(_) => true,
                _ => false
            };
            if is_fixed {
                return new_game;
            }
        }
        self.clone()
    }

    fn swap(&self, index : usize) -> Self {
        //println!("swapping index={} len={}", index, self.instructions.len());
        let mut instructions : Vec<Instruction> = vec![];
        for i in 0..index {
            instructions.push(self.instructions[i]);
        }
        instructions.push(match self.instructions[index] {
            Instruction::NOP{value} => Instruction::JMP{offset: value},
            Instruction::JMP{offset} => Instruction::NOP{value: offset},
            Instruction::ACC{value} => Instruction::ACC{value: value},
        });
        for i in (index + 1)..self.instructions.len() {
            instructions.push(self.instructions[i]);
        }
        GameConsole {
            instructions: instructions,
        }
    }

    fn execute(&self) -> Result<i32, GameConsoleExecutionError>  {
        let mut accumulator : i32 = 0;
        let mut indexes_visited : HashSet<usize> = HashSet::new();
        let mut ip : usize = 0;

        loop {
            if ip == self.instructions.len() {
                return Ok(accumulator);
            }
            if ip > self.instructions.len() {
                return Err(GameConsoleExecutionError::InstructionPointerExceedsProgram);
            }
            if indexes_visited.contains(&ip) {
                return Err(GameConsoleExecutionError::InfiniteLoopError {last_accumulator: accumulator});
            }
            indexes_visited.insert(ip);
            //println!("executing ip={} {:?}", ip, self.instructions[ip]);
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
                Instruction::NOP{value : _} => {
                    ip = ip + 1;
                }
            }
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let game_console = GameConsole::from_input()?;
    println!("part1: execute game: {:?}", game_console.execute());

    let fixed_game = game_console.fix();
    println!("part2: fixed execute game: {:?}", fixed_game.execute());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest(rule, result,
        case("nop +0", Instruction::NOP{value: 0}),
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
            Instruction::NOP{value: 0},
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
                NOP {value: 0},
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

    mod execution {
        use super::*;

        #[test]
        fn full_game() -> Result<(), Box<dyn std::error::Error>> {
            let game = GameConsole::from_str("
            nop +0
            jmp +2
            acc +1
            acc +1
            acc +1").unwrap();

            let result = game.execute()?;
            assert_eq!(result, 2);
            Ok(())
        }

        #[test]
        fn full_game2() -> Result<(), Box<dyn std::error::Error>> {
            let game = GameConsole::from_str("
            acc +1
            acc +1
            acc +1").unwrap();

            let result = game.execute()?;
            assert_eq!(result, 3);
            Ok(())
        }
        #[test]
        fn ip_exceeds_length_of_program() -> Result<(), String> {
            let game = GameConsole::from_str("
            jmp +2").unwrap();

            match game.execute() {
                Err(GameConsoleExecutionError::InstructionPointerExceedsProgram) => {
                    Ok(())
                }
                _ => Err("should fail with InstructionPointerExceedsProgram".to_string()),
            }
        }

    }

    mod part2 {
        use super::*;

        #[rstest(index, output,
            case(0, 2),
            case(1, 2),
            case(2, 3),
        )]
        fn swap(index : usize, output : i32) -> Result<(), Box<dyn std::error::Error>> {
            let game = GameConsole::from_str("
            nop +1
            acc +1
            jmp +2
            acc +1
            acc +1").unwrap();

            let swapped_game = game.swap(index);
            println!("original: {:?}", game);
            println!("swapped_game: {:?}", swapped_game);
            let result = swapped_game.execute()?;
            assert_eq!(output, result);
            Ok(())
        }

        #[test]
        fn do_swap() -> Result<(), Box<dyn std::error::Error>> {
            let game = GameConsole::from_str("
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
            ").unwrap();
            let swapped_game = game.swap(7);
            println!("swapped_game");
            for instruction in &swapped_game.instructions {
                println!("\t{:?}", instruction);
            }

            let expected = GameConsole::from_str("
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            nop -4
            acc +6
            ").unwrap();
            println!("expected");
            for instruction in &expected.instructions {
                println!("\t{:?}", instruction);
            }
            assert_eq!(expected, swapped_game);
            Ok(())
        }

        #[test]
        fn find_bug() {
            let game = GameConsole::from_str("
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6").unwrap();

            let fixed_game = game.fix();
            println!("fixed_game");
            for instruction in &fixed_game.instructions {
                println!("\t{:?}", instruction);
            }
            let result = fixed_game.execute().unwrap();
            println!("game: {:?}", result);
            assert_eq!(8, result);
        }
    }

    #[test]
    fn part2() -> Result<(), Box<dyn std::error::Error>> {
        let game = GameConsole::from_input()?;
        let fixed_game = game.fix();
        assert_eq!(1000, fixed_game.execute()?);
        Ok(())
    }

}
