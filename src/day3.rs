use std::ops::Add;
use crate::inputs::read_input;

#[derive(Debug, PartialEq, Copy,Clone)]
enum ForestItem {
    Open,
    Tree,
    Unknown,
}
#[derive(Debug, PartialEq, Clone)]
struct Forest {
    rows: Vec<Vec<ForestItem>>,
    width: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x : usize,
    y : usize
}

impl Add for Position {
    type Output = Self;

    fn add(self, pos : Position)-> Position {
        Position {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
}

impl Forest {

    fn multiply_all(&self) -> usize {
        let slopes = vec! (
            Position{x: 1, y: 1},
            Position{x: 3, y: 1},
            Position{x: 5, y: 1},
            Position{x: 7, y: 1},
            Position{x: 1, y: 2},
        );
        let trees_hit : Vec<usize> = slopes.iter()
            .map(|x| self.how_many_trees_do_you_encounter(*x))
            .collect();

        let mut total = 1;
        for tree_hit in trees_hit {
            total = total * tree_hit;
        }
        total
    }
    fn parse_row(s : &str) -> Vec<ForestItem> {
        let mut row : Vec<ForestItem> = vec!();
        for c in s.chars() {
            let item = if c == '#' {
                ForestItem::Tree
            } else if c == '.' {
                ForestItem::Open
            } else {
                ForestItem::Unknown
            };
            row.push(item);
        }
        row
    }

    fn from_str(s:&str) -> Forest {
        let rows : Vec<Vec<ForestItem>> = s.to_string().split("\n")
            .filter(|x| !x.is_empty())
            .map(Forest::parse_row)
            .collect();

        Forest {
            rows: rows.clone(),
            width: rows[0].len(),
        }
    }

    fn find_item(&self, pos: Position) -> ForestItem {
        self.rows[pos.y][pos.x % self.width]
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn contains(&self, pos : Position) -> bool {
        pos.y < self.height()
    }

    fn how_many_trees_do_you_encounter(&self, slope : Position) -> usize {
        let mut current = Position{x: 0, y: 0};
        let mut trees = 0;
        loop {
            if !self.contains(current) {
                break;
            }
            let item = self.find_item(current);
            if item == ForestItem::Tree {
                trees = trees + 1;
            }
            current = current + slope;
        }

        trees
    }
}
pub fn main() -> std::io::Result<()> {
    let contents = read_input(3)?;

    let forest = Forest::from_str(contents.as_str());
    //println!("{:?}", forest);

    let trees = forest.how_many_trees_do_you_encounter(Position{x:3, y: 1});
    println!("trees hit: {}", trees);

    println!("all trees hit, multiplied: {}", forest.multiply_all());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ForestItem::*;

    const FOREST_SAMPLE :&str= "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    #[test]
    fn parse_forest() {
        assert_eq!(Forest::from_str(FOREST_SAMPLE), Forest {
            width: 11,
            rows: vec!(
                vec!(Open,Open,Tree,Tree,Open,Open,Open,Open,Open,Open,Open),
                vec!(Tree,Open,Open,Open,Tree,Open,Open,Open,Tree,Open,Open),
                vec!(Open,Tree,Open,Open,Open,Open,Tree,Open,Open,Tree,Open),
                vec!(Open,Open,Tree,Open,Tree,Open,Open,Open,Tree,Open,Tree),
                vec!(Open,Tree,Open,Open,Open,Tree,Tree,Open,Open,Tree,Open),
                vec!(Open,Open,Tree,Open,Tree,Tree,Open,Open,Open,Open,Open),
                vec!(Open,Tree,Open,Tree,Open,Tree,Open,Open,Open,Open,Tree),
                vec!(Open,Tree,Open,Open,Open,Open,Open,Open,Open,Open,Tree),
                vec!(Tree,Open,Tree,Tree,Open,Open,Open,Tree,Open,Open,Open),
                vec!(Tree,Open,Open,Open,Tree,Tree,Open,Open,Open,Open,Tree),
                vec!(Open,Tree,Open,Open,Tree,Open,Open,Open,Tree,Open,Tree),
            )
        })
    }

    #[test]
    fn forest_get_item() {
        let forest = Forest::from_str("..##.\n#..##");
        assert_eq!(forest.find_item(Position{x: 0, y: 0}), Open);
        assert_eq!(forest.find_item(Position{x: 0, y: 1}), Tree);

        //repeating
        assert_eq!(forest.find_item(Position{x: 5, y: 0}), Open);
        assert_eq!(forest.find_item(Position{x: 5, y: 1}), Tree);
    }

    #[test]
    fn check_slope() {
        let forest = Forest::from_str(FOREST_SAMPLE);
        assert_eq!(7, forest.how_many_trees_do_you_encounter(Position{x: 3, y: 1}));
    }

    #[test]
    fn check_slopes() {
        let forest = Forest::from_str(FOREST_SAMPLE);
        assert_eq!(336, forest.multiply_all());
    }
}
