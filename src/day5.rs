use crate::inputs::read_input;
use std::str::FromStr;
use std::{error::Error, fmt};

#[cfg(test)]
use rstest::rstest;

#[derive(Debug, PartialEq, Clone)]
enum RowSelector {
    Front,
    Back,
}
impl RowSelector {
    fn from_str(o : Option<char>) -> RowSelector {
        match o {
            Some('B') => RowSelector::Back,
            Some('F') => RowSelector::Front,
            _ => panic!("invalid column selector: {:?}", o),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ColumnSelector {
    Left,
    Right,
}

impl ColumnSelector {
    fn from_str(o : Option<char>) -> ColumnSelector {
        match o {
            Some('L') => ColumnSelector::Left,
            Some('R') => ColumnSelector::Right,
            _ => panic!("invalid column selector: {:?}", o),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Seat {
    columns : [ColumnSelector; 3],
    rows : [RowSelector; 7],
}

#[derive(Debug)]
struct SeatParseError;

impl Error for SeatParseError {}
impl fmt::Display for SeatParseError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse seat")
    }
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row() * 8 + self.column()
    }
    fn row(&self) -> usize {
        let mut min = 0;
        let mut max = 127;
        //println!("row min={}, max={}", min, max);
        for i in 0..self.rows.len() {
            let half = min + (max - min) / 2;
            let (local_min, local_max) = if self.rows[i] == RowSelector::Front {
                (min, half)
            } else {
                (half, max)
            };
            max = local_max;
            min = local_min;
            //println!("row min={}, max={} param={:?}", min, max, self.rows[i]);
        }
        max
    }
    fn column(&self) -> usize {
        let mut min = 0;
        let mut max = 7;
        //println!("column min={}, max={}", min, max);
        for i in 0..self.columns.len() {
            let half = min + (max - min) / 2;
            let (local_min, local_max) = if self.columns[i] == ColumnSelector::Left {
                (min, half)
            } else {
                (half, max)
            };
            max = local_max;
            min = local_min;
            //println!("column min={}, max={} param={:?}", min, max, self.columns[i]);
        }
        max
    }
}
impl FromStr for Seat {
    type Err = SeatParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("parsing {}", s);
        let rows = &s[0..7];
        let columns = &s[7..10];

        //println!("columns: {}", columns);
        //println!("rows: {}", rows);

        Ok(Seat{
            columns: [
                ColumnSelector::from_str(columns.chars().nth(0)),
                ColumnSelector::from_str(columns.chars().nth(1)),
                ColumnSelector::from_str(columns.chars().nth(2)),
            ],
            rows: [
                RowSelector::from_str(rows.chars().nth(0)),
                RowSelector::from_str(rows.chars().nth(1)),
                RowSelector::from_str(rows.chars().nth(2)),
                RowSelector::from_str(rows.chars().nth(3)),
                RowSelector::from_str(rows.chars().nth(4)),
                RowSelector::from_str(rows.chars().nth(5)),
                RowSelector::from_str(rows.chars().nth(6)),
            ]
        })
    }
}

struct Seats {
    seats : Vec<Seat>
}

impl FromStr for Seats {
    type Err = SeatParseError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let mut seats : Vec<Seat> = vec![];
        for line in s.split("\n") {
            if !line.is_empty() {
                let seat = Seat::from_str(line)?;
                seats.push(seat);
            }
        }
        Ok(Seats {
            seats: seats,
        })
    }
}

impl Seats {
    fn len(&self) -> usize {
        self.seats.len()
    }

    fn from_input() -> Result<Seats, Box<dyn std::error::Error>> {
        let contents = read_input(5)?;
        let seats = Seats::from_str(contents.as_str())?;
        println!("found {} seats", seats.len());
        Ok(seats)
    }

    fn highest_seat_id(&self) -> usize {
        match self.seats.iter().map(Seat::seat_id).max() {
            Some(x) => x,
            None => 0,
        }
    }

    fn my_seat(&self) -> Vec<usize> {
        let all_seat_ids : Vec<usize> = self.seats.iter()
            .map(Seat::seat_id)
            .collect();
        let highest_seat_id = self.highest_seat_id();
        (0..=highest_seat_id)
            .filter(|seat_id| !all_seat_ids.contains(seat_id))
            .filter(|seat_id| all_seat_ids.contains(&(*seat_id + 1)) && all_seat_ids.contains(&(*seat_id - 1)))
            .collect()
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    let seats = Seats::from_input()?;
    println!("highest seat_id: {}", seats.highest_seat_id());

    let my_seat = seats.my_seat();
    println!("my seat: {:?}", my_seat);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ColumnSelector::*;
    use RowSelector::*;

    #[test]
    fn parse_seat() {
        let seat = Seat::from_str("BFFFBBFRRR").unwrap();
        assert_eq!(seat.columns, [Right, Right, Right]);
        assert_eq!(seat.rows, [Back, Front, Front, Front, Back, Back, Front]);
    }

    #[rstest(location, row, column, seat_id,
        case("FBFBBFFRLR", 44, 5, 357),
        case("BFFFBBFRRR", 70, 7, 567),
        case("FFFBBBFRRR", 14, 7, 119),
        case("BBFFBBFRLL", 102, 4, 820),
        ::trace
    )]
    fn calculate_seat_position(location : &str, row : usize, column : usize, seat_id : usize) {
        let seat = Seat::from_str(location).unwrap();

        assert_eq!(seat.row(), row);
        assert_eq!(seat.column(), column);
        assert_eq!(seat.seat_id(), seat_id);
    }

    #[test]
    fn highest_seat_id() {
        let seats = Seats::from_str("
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL").unwrap();

        assert_eq!(seats.highest_seat_id(), 820);
    }

    #[test]
    fn my_seat() {
        let seats = Seats::from_input().unwrap();
        let my_seat = seats.my_seat();

        assert_eq!(my_seat, vec![617]);
    }
}
