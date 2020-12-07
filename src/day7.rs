use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashSet;
use std::collections::HashMap;

#[cfg(test)]
use rstest::rstest;

#[derive(Debug)]
struct BaggageRuleParseError;

impl Error for BaggageRuleParseError {}
impl fmt::Display for BaggageRuleParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse baggage rules")
    }
}

impl std::convert::From<std::num::ParseIntError> for BaggageRuleParseError {

    fn from(_parse_int_error : std::num::ParseIntError) -> BaggageRuleParseError {
        BaggageRuleParseError{}
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BagRule {
    color : String,
    contains : HashMap<String, usize>,
}

#[derive(Debug, PartialEq, Clone)]
struct BagCount {
    color : String,
    count : usize,
}
impl FromStr for BagCount {
    type Err = BaggageRuleParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.trim().split(" ").collect();
        let color = &tokens[1..tokens.len() - 1];
        Ok(BagCount {
            color: color.join(" "),
            count: tokens[0].parse::<usize>()?,
        })
    }
}
impl FromStr for BagRule {
    type Err = BaggageRuleParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(" bags contain ").collect();
        //println!("tokens: {:?}", tokens);
        let contains = if tokens[1].contains("no other bags.") {
            HashMap::new()
        } else {
            let rhs : Result<Vec<BagCount>, Self::Err> = tokens[1].split(",")
                .map(BagCount::from_str)
                .collect();
            let mut map = ::std::collections::HashMap::new();
            for bag_count in rhs? {
                map.insert(bag_count.color, bag_count.count);
            }
            map
        };
        Ok(BagRule {
            color: tokens[0].to_string(),
            contains: contains,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BagRules {
    bag_rules : Vec<BagRule>
}

impl FromStr for BagRules {
    type Err = BaggageRuleParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let results : Result<Vec<BagRule>, Self::Err> = s.split("\n")
                .filter(|x| !x.is_empty())
                .map(BagRule::from_str)
                .collect();
        Ok(BagRules {
            bag_rules: results?
        })
    }
}

impl BagRules{

    fn from_input() -> Result<BagRules, Box<dyn std::error::Error>> {
        let contents = read_input(7)?;
        let groups = BagRules::from_str(contents.as_str())?;
        Ok(groups)
    }

    fn len(&self) -> usize {
        self.bag_rules.len()
    }

    fn node_for_color(&self, cached_nodes : &mut HashMap<String, Option<BagNode>>, color : &str) -> Option<BagNode> {
        //println!("creating node for color={}", color);
        match cached_nodes.get(&color.to_string()) {
            Some(cached_value) => cached_value.as_ref().map(|x| x.clone()),
            None => {
                for bag_rule in &self.bag_rules {
                    if bag_rule.color == color.to_string() {
                        let contains : Vec<BagNode> = bag_rule.contains.iter()
                            .map(|bag_rule| self.node_for_color(cached_nodes, bag_rule.0).unwrap())
                            .collect();
                        let result = Some(BagNode {
                            color: color.to_string(),
                            contains: contains,
                        });
                        cached_nodes.insert(color.to_string(), result.clone());
                        return result;
                    }
                }
                None
            }
        }
    }

    fn bags_that_can_contain(&self, color : &str) -> HashSet<String> {
        let mut cached_map : HashMap<String, Option<BagNode>> = HashMap::new();
        let bag_nodes : Vec<BagNode> = self.bag_rules.iter()
            .map(|bag_rule| self.node_for_color(&mut cached_map, bag_rule.color.as_str()).unwrap())
            .collect();
        //println!("created tree. count={}", bag_nodes.len());
        let can_contain : HashSet<String> = bag_nodes.iter()
            .filter(|node| node.can_contain(color))
            .map(|node| node.color.clone())
            .filter(|node_color| *node_color != color.to_string())
            .collect();
        can_contain
    }

}

#[derive(Debug, PartialEq, Clone)]
struct BagNode {
    color : String,
    contains : Vec<BagNode>,
}

impl BagNode {
    fn can_contain(&self, color : &str) -> bool {
        //println!("checking can_contain: {}", color);
        if self.color == color.to_string() {
            true
        } else {
            let can_contain : Vec<&BagNode> = self.contains.iter()
                .filter(|node| node.can_contain(color))
                .collect();
            can_contain.len() > 0
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let rules = BagRules::from_input()?;
    println!("loaded {} rules", rules.len());
    println!("part1: {} bags can contain at least 1 shiny gold", rules.bags_that_can_contain("shiny gold").len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
                                                 {
                                                     let mut m = ::std::collections::HashMap::new();
                                                     $(
                                                         m.insert($key, $value);
                                                     )+
                                                         m
                                                 }
                                             };
    );
    #[rstest(rule, result,
        case("light red bags contain 1 bright white bag, 2 muted yellow bags.", BagRule {
            color: "light red".to_string(),
            contains: map!{
                "bright white".to_string() => 1,
                "muted yellow".to_string() => 2
            }
        }),
        case("dark silver bags contain 1 dotted orange bag, 3 plaid beige bags, 5 faded white bags.", BagRule {
            color: "dark silver".to_string(),
            contains: map!{
                "dotted orange".to_string() => 1,
                "plaid beige".to_string() => 3,
                "faded white".to_string() => 5
            },
        }),
        case("dark silver bags contain no other bags.", BagRule {
            color: "dark silver".to_string(),
            contains: HashMap::new(),
        }),
    )]
    fn parse_bag_rule(rule: &str, result : BagRule) {
        let parsed = BagRule::from_str(rule).unwrap();
        assert_eq!(parsed, result);
    }

    #[test]
    fn parse_bag_rules() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark silver bags contain 1 dotted orange bag, 3 plaid beige bags, 5 faded white bags.";
        let expected = BagRules {
            bag_rules: vec![
                BagRule {
                    color: "light red".to_string(),
                    contains: map!{
                        "bright white".to_string() => 1,
                        "muted yellow".to_string() => 2
                    }
                },
                BagRule {
                    color: "dark silver".to_string(),
                    contains: map!{
                        "dotted orange".to_string() => 1,
                        "plaid beige".to_string() => 3,
                        "faded white".to_string() => 5
                    },
                },
            ],
        };
        let parsed = BagRules::from_str(input).unwrap();
        assert_eq!(expected, parsed);
    }

    const TEST_DATA : &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

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

    #[test]
    fn find_bag_colors_that_can_contain() {
        let rules = BagRules::from_str(TEST_DATA).unwrap();
        let bags_that_can_contain_shiny_gold = rules.bags_that_can_contain("shiny gold");
        assert_eq!(set![
            "bright white".to_string(),
            "muted yellow".to_string(),
            "dark orange".to_string(),
            "light red".to_string()
        ], bags_that_can_contain_shiny_gold);
    }

    #[test]
    fn part1() {
        let rules = BagRules::from_input().unwrap();
        assert_eq!(126, rules.bags_that_can_contain("shiny gold").len());
    }
}
