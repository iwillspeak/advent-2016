/// A Direciton of Travel
#[derive(Debug,PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// A turn
#[derive(Debug,PartialEq,Clone)]
pub enum Turn {
    Left,
    Right,
}

/// A position in 2d space
#[derive(Debug,PartialEq)]
pub struct Position(i32, i32);

/// A 'move' in the path
#[derive(Debug,PartialEq)]
pub struct Move(Turn, i32);

impl Direction {
    /// Return the direciton after turning.
    pub fn turn(self, t: Turn) -> Self {
        use self::Direction::*;
        match t {
            Turn::Left => {
                match self {
                    North => West,
                    West => South,
                    South => East,
                    East => North,
                }
            }
            Turn::Right => {
                match self {
                    North => East,
                    West => North,
                    South => West,
                    East => South,
                }
            }
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Position(0, 0)
    }

    /// Move in a given direction by a distance.
    pub fn move_by(&self, direction: &Direction, distance: i32) -> Self {
        let &Position(northings, eastings) = self;
        match *direction {
            Direction::North => Position(northings + distance, eastings),
            Direction::South => Position(northings - distance, eastings),
            Direction::West => Position(northings, eastings - distance),
            Direction::East => Position(northings, eastings + distance),
        }
    }

    /// Get the travel distance
    pub fn travel_dist(&self) -> i32 {
        let &Position(northings, eastings) = self;
        northings.abs() + eastings.abs()
    }
}

impl Move {
    /// get the turn
    pub fn turn(&self) -> Turn {
        let &Move(ref turn, _) = self;
        turn.clone()
    }

    /// get the distance
    pub fn dist(&self) -> i32 {
        let &Move(_, ref dist) = self;
        dist.clone()
    }
}

impl<'a> From<&'a str> for Move {
    /// Parse a move from a string
    fn from(s: &'a str) -> Self {
        let (direction, dist) = s.split_at(1);
        let turn = match direction {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => panic!("Invalid move format!"),
        };
        let dist = dist.parse::<i32>().unwrap();
        Move(turn, dist)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_turn_left() {
        assert_eq!(Direction::West, Direction::North.turn(Turn::Left));
        assert_eq!(Direction::South, Direction::West.turn(Turn::Left));
        assert_eq!(Direction::East, Direction::South.turn(Turn::Left));
        assert_eq!(Direction::North, Direction::East.turn(Turn::Left));
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(Direction::East, Direction::North.turn(Turn::Right));
        assert_eq!(Direction::North, Direction::West.turn(Turn::Right));
        assert_eq!(Direction::West, Direction::South.turn(Turn::Right));
        assert_eq!(Direction::South, Direction::East.turn(Turn::Right));
    }

    #[test]
    fn move_in_direction() {
        let pos = Position(0, 0);
        assert_eq!(Position(1, 0), pos.move_by(&Direction::North, 1));
        assert_eq!(Position(10, 0), pos.move_by(&Direction::North, 10));
        assert_eq!(Position(-7, 0), pos.move_by(&Direction::South, 7));
        assert_eq!(Position(-7, 5),
                   pos.move_by(&Direction::South, 7)
                       .move_by(&Direction::East, 10)
                       .move_by(&Direction::West, 5));
    }

    #[test]
    fn position_travel_distance() {
        assert_eq!(5, Position(3, 2).travel_dist());
        assert_eq!(5, Position(3, -2).travel_dist());
        assert_eq!(10, Position(-7, 3).travel_dist());
        assert_eq!(13, Position(-10, -3).travel_dist());
    }

    #[test]
    fn parse_moves() {
        assert_eq!(Move(Turn::Left, 10), Move::from("L10"));
        assert_eq!(Move(Turn::Right, 100), Move::from("R100"));
    }
}
