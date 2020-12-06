use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashSet;

#[cfg(test)]
use rstest::rstest;

#[derive(Debug, PartialEq, Clone)]
struct IndividualResult {
    answers : Vec<char>,
}

impl FromStr for IndividualResult {
    type Err = CustomsParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        Ok(IndividualResult {
            answers: s.chars().collect(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct GroupResult {
    results : Vec<IndividualResult>,
}

impl GroupResult {
    fn from_vec(results : Vec<Vec<char>>) -> Self {
        GroupResult {
            results: results.iter().map(|answers| IndividualResult {
                answers: answers.iter().map(|x| *x).collect(),
            }).collect(),
        }
    }

    fn part1_count(&self) -> usize {
        let set : HashSet<char> = self.results.iter()
            .flat_map(|x| x.answers.iter())
            .map(|x| *x)
            .collect();
        set.len()
    }
}

#[derive(Debug)]
struct CustomsParseError;

impl Error for CustomsParseError {}
impl fmt::Display for CustomsParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse group results")
    }
}

impl FromStr for GroupResult {
    type Err = CustomsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let members : Result<Vec<IndividualResult>, Self::Err> = s.split("\n")
            .map(IndividualResult::from_str)
            .collect();
        Ok(GroupResult {
            results: members?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Groups {
    results : Vec<GroupResult>
}

impl FromStr for Groups {
    type Err = CustomsParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let results : Result<Vec<GroupResult>, Self::Err> = s.split("\n\n")
                .map(GroupResult::from_str)
                .collect();
        Ok(Groups {
            results: results?
        })
    }
}

impl Groups {
    fn len(&self) -> usize {
        self.results.len()
    }
    fn part1_count(&self) -> usize {
        self.results.iter().map(GroupResult::part1_count).sum()
    }

    fn from_input() -> Result<Groups, Box<dyn std::error::Error>> {
        let contents = read_input(6)?;
        let groups = Groups::from_str(contents.as_str())?;
        Ok(groups)
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let groups = Groups::from_input()?;
    println!("loaded {} groups", groups.len());
    println!("group sum of part1 counts = {}", groups.part1_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest(group, part1_count, result,
        case("abcx\nabcy\nabcz", 6, vec![
            vec!['a', 'b', 'c', 'x'],
            vec!['a', 'b', 'c', 'y'],
            vec!['a', 'b', 'c', 'z']]),
    )]
    fn parse_group_result(group : &str, result : Vec<Vec<char>>, part1_count : usize) {
        let group = GroupResult::from_str(group).unwrap();
        assert_eq!(group, GroupResult::from_vec(result));
        assert_eq!(group.part1_count(), part1_count);
    }

    const TEST_INPUT : &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn parse_multiple() {
        let groups = Groups::from_str(TEST_INPUT).unwrap();
        assert_eq!(groups, Groups {
            results: vec![
                GroupResult::from_vec(vec![vec!['a', 'b', 'c']]),
                GroupResult::from_vec(vec![
                    vec!['a'],
                    vec!['b'],
                    vec!['c'],
                ]),
                GroupResult::from_vec(vec![
                    vec!['a', 'b'],
                    vec!['a', 'c'],
                ]),
                GroupResult::from_vec(vec![
                    vec!['a'],
                    vec!['a'],
                    vec!['a'],
                    vec!['a'],
                ]),
                GroupResult::from_vec(vec![
                    vec!['b']
                ]),
            ]
        });
        assert_eq!(groups.part1_count(), 11);
    }

    #[test]
    fn from_input() {
        let groups = Groups::from_input().unwrap();
        assert_eq!(groups.part1_count(), 6947);
    }

}
