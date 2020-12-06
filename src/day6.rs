use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashSet;

#[cfg(test)]
use rstest::rstest;

#[derive(Debug, PartialEq, Clone)]
struct GroupResult {
    results : HashSet<char>,
}

impl GroupResult {
    fn count(&self) -> usize {
        self.results.len()
    }
}

#[derive(Debug)]
struct GroupResultParseError;

impl Error for GroupResultParseError {}
impl fmt::Display for GroupResultParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse group results")
    }
}

impl FromStr for GroupResult {
    type Err = GroupResultParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let members : HashSet<char> = s.split("\n")
            .flat_map(|x| x.chars())
            .collect();
        Ok(GroupResult {
            results: members,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Groups {
    results : Vec<GroupResult>
}

impl FromStr for Groups {
    type Err = GroupResultParseError;

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
    fn count(&self) -> usize {
        self.results.iter().map(GroupResult::count).sum()
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
    println!("group sum of counts = {}", groups.count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
            {
                let mut temp_set = HashSet::new();  // Create a mutable HashSet
                $(
                    temp_set.insert($x); // Insert each item matched into the HashSet
                )*
                    temp_set // Return the populated HashSet
            }
        };
    }

    #[rstest(group, result, count,
        case("abcx\nabcy\nabcz", set!['a', 'b', 'c', 'x', 'y', 'z'], 6),
    )]
    fn parse_group_result(group : &str, result : HashSet<char>, count : usize) {
        let group = GroupResult::from_str(group).unwrap();
        assert_eq!(group.results, result);
        assert_eq!(group.count(), count);
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
                GroupResult{
                    results: set!['a', 'b', 'c'],
                },
                GroupResult{
                    results: set!['a', 'b', 'c'],
                },
                GroupResult{
                    results: set!['a', 'b', 'c'],
                },
                GroupResult{
                    results: set!['a'],
                },
                GroupResult{
                    results: set!['b'],
                },
            ]
        });
        assert_eq!(groups.count(), 11);
    }

    #[test]
    fn from_input() {
        let groups = Groups::from_input().unwrap();
        assert_eq!(groups.count(), 6947);
    }

}
