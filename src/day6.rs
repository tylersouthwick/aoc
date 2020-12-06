use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashSet;

#[cfg(test)]
use rstest::rstest;

#[derive(Debug, PartialEq, Clone)]
struct IndividualResult {
    answers : HashSet<char>,
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

    #[cfg(test)]
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

    fn part2_count(&self) -> usize {
        let all : HashSet<char> = self.results.iter()
            .flat_map(|x| x.answers.iter())
            .map(|x| *x)
            .collect();
        let intersected : HashSet<char> = self.results.iter()
            .fold(all, |a, b| a.intersection(&b.answers).map(|x| *x).collect());
        intersected.len()
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
            .filter(|x| !x.is_empty())
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
                .filter(|x| !x.is_empty())
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

    fn part2_count(&self) -> usize {
        self.results.iter().map(GroupResult::part2_count).sum()
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
    println!("group sum of part2 counts = {}", groups.part2_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest(group, part1_count, part2_count, result,
        case("abcx\nabcy\nabcz", 6, 3, vec![
            vec!['a', 'b', 'c', 'x'],
            vec!['a', 'b', 'c', 'y'],
            vec!['a', 'b', 'c', 'z']]),
    )]
    fn parse_group_result(group : &str, result : Vec<Vec<char>>, part1_count : usize, part2_count : usize) {
        let group = GroupResult::from_str(group).unwrap();
        assert_eq!(group, GroupResult::from_vec(result));
        assert_eq!(group.part1_count(), part1_count);
        assert_eq!(group.part2_count(), part2_count);
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
        assert_eq!(groups.part2_count(), 6);
    }

    #[test]
    fn from_input_part1_count() {
        let groups = Groups::from_input().unwrap();
        assert_eq!(groups.part1_count(), 6947);
    }

    #[test]
    fn from_input_part2_count() {
        let groups = Groups::from_input().unwrap();
        println!("{:?}", groups);
        assert_eq!(groups.part2_count(), 3398);
    }

    #[rstest(group, count,
        case("abc", 3),
        case("a\nb\nc", 0),
        case("ab\nac", 1),
        case("a\na\na\na", 1),
        case("b", 1),
        case("xav\nuavx\nxavsi\nyavx", 3),
        case("xyqbn\ncxypns\nhkgylf", 1),
        case("abcdefghijklmnopqrstuvwxyz\nabcdefghijklmnopqrstuvwxyz\nabcdefghijklmnopqrstuvwxyz\n", 26),
        case("abcdefghijklmnopqrstuvwxyz\nab", 2),
        case("abcdefghijklmnopqrstuvwxyz\nzyxwvut", 7),
    )]
    fn part2_group_count(group : &str, count: usize) {
        let group = GroupResult::from_str(group).unwrap();
        assert_eq!(group.part2_count(), count);
    }

}
