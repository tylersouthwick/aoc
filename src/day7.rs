use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};
use std::collections::HashSet;
use std::collections::HashMap;

#[cfg(test)]
use rstest::rstest;

trait BagContents {
    fn can_contain(&self, s : &str) -> bool;
    fn number_of_bags(&self) -> usize;
}

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
struct BagRule<A : Clone> {
    color : String,
    contains : Vec<BagCount<A>>,
}

impl <A : Clone> BagRule<A> {

    fn map<B : Clone, F>(&self, f: &mut F) -> BagRule<B> where F: FnMut(A) -> B {
        BagRule {
            color: self.color.clone(),
            contains: self.contains.iter()
                .map(|bag_count| bag_count.map(f))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BagCount<A : Clone> {
    color : A,
    count : usize,
}

impl <A : Clone> BagCount<A> {
    fn map<B : Clone, F>(&self, f: &mut F) -> BagCount<B> where F: FnMut(A) -> B {
        BagCount {
            color: f(self.color.clone()),
            count: self.count,
        }
    }
}
impl FromStr for BagCount<String> {
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
impl <A> BagContents for BagCount<A> where A : BagContents + Clone + std::fmt::Debug {
    fn can_contain(&self, s : &str) -> bool {
        self.color.can_contain(s)
    }

    fn number_of_bags(&self) -> usize {
        let number_of_bags = self.color.number_of_bags();
        let result = self.count * number_of_bags;
        //println!("BagCount count={} color={:?} number_of_bags={} result={}", self.count, self.color, number_of_bags, result);
        result
    }

}
impl FromStr for BagRule<String> {
    type Err = BaggageRuleParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(" bags contain ").collect();
        //println!("tokens: {:?}", tokens);
        let contains = if tokens[1].contains("no other bags.") {
            vec![]
        } else {
            let rhs : Result<Vec<BagCount<String>>, Self::Err> = tokens[1].split(",")
                .map(BagCount::from_str)
                .collect();
            rhs?
        };
        Ok(BagRule {
            color: tokens[0].to_string(),
            contains: contains,
        })
    }
}
impl <A> BagContents for BagRule<A> where A: BagContents + Clone + std::fmt::Debug  {
    fn can_contain(&self, s : &str) -> bool {
        let can_contain = self.contains.iter()
            .map(|bag_count| bag_count.can_contain(s))
            .fold(false, |a, b| a || b);
        //println!("BagRule checking if {} can contain {}: {}", self.color, s, can_contain);
        can_contain || self.color == s.to_string()
    }

    fn number_of_bags(&self) -> usize {
        //println!("BagRule:: looking for bags in bag rule: {}", self.color);
        let num : usize = self.contains.iter()
            .map(BagContents::number_of_bags)
            .sum();
        //println!("BagRule:: found {} bags in bag rule: {}", num, self.color);
        num + 1
    }

}

#[derive(Debug, PartialEq)]
struct BagRules<A : Clone> {
    bag_rules : Vec<BagRule<A>>
}

impl FromStr for BagRules<String> {
    type Err = BaggageRuleParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let results : Result<Vec<BagRule<String>>, Self::Err> = s.split("\n")
                .filter(|x| !x.is_empty())
                .map(BagRule::from_str)
                .collect();
        Ok(BagRules {
            bag_rules: results?
        })
    }
}

impl BagRules<String> {

    fn from_input() -> Result<BagRules<String>, Box<dyn std::error::Error>> {
        let contents = read_input(7)?;
        let groups = BagRules::from_str(contents.as_str())?;
        Ok(groups)
    }

    fn as_nodes(&self) -> BagRules<BagNode> {
        let mut cached_map : HashMap<String, BagNode> = HashMap::new();
        self.map(&mut |color : String| self.node_for_color(&mut cached_map, color.as_str()))
    }

    fn node_for_color(&self, cached_map : &mut HashMap<String, BagNode>, color : &str) -> BagNode {
        match cached_map.get(&color.to_string()) {
            Some(node) => node.clone(),
            None => {
                //println!("creating node for color=[{}]", color);
                for bag_rule in &self.bag_rules {
                    //println!("checking bag_rule.color=[{}] color=[{}]", bag_rule.color, color.to_string());
                    if bag_rule.color == color.to_string() {
                        //println!("found {}", color.to_string());
                        let contains : Vec<BagCount<BagNode>> = bag_rule.contains.iter()
                            .map(|bag_count| bag_count.map(&mut |bag_color : String| self.node_for_color(cached_map, bag_color.as_str())))
                            .collect();
                        let node = BagNode {
                            bag_rule: BagRule {
                                color: bag_rule.color.clone(),
                                contains: contains,
                            }
                        };
                        cached_map.insert(color.to_string(), node.clone());
                        return node;
                    }
                }
                panic!("could not find node for color=[{}]", color);
            }
        }
    }

}

impl <A : Clone> BagRules<A> {
    fn map<B : Clone, F>(&self, f: &mut F) -> BagRules<B> where F: FnMut(A) -> B {
        BagRules {
            bag_rules: self.bag_rules.iter()
                .map(|bag_rule| bag_rule.map(f))
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.bag_rules.len()
    }

}

impl BagRules<BagNode> {
    fn bags_that_can_contain(&self, color : &str) -> HashSet<String> {
        let can_contain : HashSet<String> = self.bag_rules
            .iter()
            .filter(|bag_rule| bag_rule.can_contain(color))
            .map(|bag_rule| bag_rule.color.clone())
            .filter(|node_color| *node_color != color.to_string())
            .collect();
        can_contain
    }

    fn count_number_of_bags_contained_within(&self, color : &str) -> usize {
        let total : usize = self.bag_rules.iter()
            .filter(|bag_rule| bag_rule.color == color.to_string())
            .map(BagContents::number_of_bags)
            .sum();
        total - 1 //this includes the one at the top
    }

}

#[derive(Debug, PartialEq, Clone)]
struct BagNode {
    bag_rule: BagRule<BagNode>,
}

impl BagContents for BagNode {
    fn can_contain(&self, color : &str) -> bool {
        self.bag_rule.can_contain(color)
    }

    fn number_of_bags(&self) -> usize {
        self.bag_rule.number_of_bags()
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let rules = BagRules::from_input()?.as_nodes();
    println!("loaded {} rules", rules.len());
    println!("part1: {} bags can contain at least 1 shiny gold", rules.bags_that_can_contain("shiny gold").len());
    println!("part2: {} individual bags are required in a shiny gold bag", rules.count_number_of_bags_contained_within("shiny gold"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest(rule, result,
        case("light red bags contain 1 bright white bag, 2 muted yellow bags.", BagRule {
            color: "light red".to_string(),
            contains: vec![
                BagCount {
                    color: "bright white".to_string(),
                    count: 1,
                },
                BagCount {
                    color: "muted yellow".to_string(),
                    count: 2,
                },
            ]
        }),
        case("dark silver bags contain 1 dotted orange bag, 3 plaid beige bags, 5 faded white bags.", BagRule {
            color: "dark silver".to_string(),
            contains: vec![
                BagCount {
                    color: "dotted orange".to_string(),
                    count: 1,
                },
                BagCount {
                    color: "plaid beige".to_string(),
                    count: 3,
                },
                BagCount {
                    color: "faded white".to_string(),
                    count: 5
                },
            ],
        }),
        case("dark silver bags contain no other bags.", BagRule {
            color: "dark silver".to_string(),
            contains: vec![],
        }),
    )]
    fn parse_bag_rule(rule: &str, result : BagRule<String>) {
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
                    contains: vec![
                        BagCount {
                            color: "bright white".to_string(),
                            count: 1,
                        },
                        BagCount {
                            color: "muted yellow".to_string(),
                            count: 2,
                        },
                    ],
                },
                BagRule {
                    color: "dark silver".to_string(),
                    contains: vec![
                        BagCount {
                            color: "dotted orange".to_string(),
                            count: 1,
                        },
                        BagCount {
                            color: "plaid beige".to_string(),
                            count: 3,
                        },
                        BagCount {
                            color: "faded white".to_string(),
                            count: 5,
                        },
                    ],
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
        let rules = BagRules::from_str(TEST_DATA).unwrap().as_nodes();
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
        let rules = BagRules::from_input().unwrap().as_nodes();
        assert_eq!(126, rules.bags_that_can_contain("shiny gold").len());
    }

    const PART2_TEST_DATA : &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[rstest(data, expected_count,
        case(TEST_DATA, 32),
        case(PART2_TEST_DATA, 126),
        ::trace
    )]
    fn count_number_of_bags_contained_within(data : &str, expected_count : usize) {
        let rules = BagRules::from_str(data).unwrap().as_nodes();
        //println!("{:?}", rules);
        let count = rules.count_number_of_bags_contained_within("shiny gold");
        assert_eq!(expected_count, count);
    }

}
