//! Day 2

/// Direction (up, down, left, right)
#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Keypad Position
#[derive(Debug, PartialEq, Clone)]
pub struct KeypadPos {
    accross: usize,
    down: usize,
}

static DIGITS: [[u32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

impl From<char> for Direction {
    /// Convert a string into a `Direction`
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction {}", c),
        }
    }
}

impl KeypadPos {
    /// Create a keypad position.
    pub fn new() -> Self {
        KeypadPos {
            accross: 1,
            down: 1,
        }
    }

    /// Move to the next key
    pub fn move_key(&self, dir: Direction) -> Self {
        use std::cmp;
        match dir {
            Direction::Up => {
                KeypadPos {
                    accross: self.accross,
                    down: cmp::max(self.down, 1) - 1,
                }
            }
            Direction::Down => {
                KeypadPos {
                    accross: self.accross,
                    down: cmp::min(self.down + 1, 2),
                }
            }
            Direction::Left => {
                KeypadPos {
                    accross: cmp::max(self.accross, 1) - 1,
                    down: self.down,
                }
            }
            Direction::Right => {
                KeypadPos {
                    accross: cmp::min(self.accross + 1, 2),
                    down: self.down,
                }
            }
        }
    }

    /// Move by a direction string
    pub fn move_by(&self, directions: &str) -> Self {
        directions.chars()
            .map(|c| Direction::from(c))
            .fold(self.clone(), |pos, dir| pos.move_key(dir))
    }

    /// Get the digit at the keypad's current position.
    pub fn digit(&self) -> u32 {
        DIGITS[self.down][self.accross]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_direction() {
        assert_eq!(Direction::Up, Direction::from('U'));
        assert_eq!(Direction::Down, Direction::from('D'));
        assert_eq!(Direction::Left, Direction::from('L'));
        assert_eq!(Direction::Right, Direction::from('R'));
    }

    #[test]
    #[should_panic]
    fn parse_invalid_direction() {
        Direction::from('H');
    }

    #[test]
    fn keypad_pos_defaults_to_5() {
        let pos = KeypadPos::new();
        assert_eq!(5, pos.digit());
    }

    #[test]
    fn move_keypad_pos() {
        let pos = KeypadPos::new();
        assert_eq!(2, pos.move_key(Direction::Up).digit());
        assert_eq!(4, pos.move_key(Direction::Left).digit());
        assert_eq!(6, pos.move_key(Direction::Right).digit());
        assert_eq!(8, pos.move_key(Direction::Down).digit());
    }

    #[test]
    fn move_keypad_pos_saturates() {
        let pos = KeypadPos::new();
        assert_eq!(2,
                   pos.move_key(Direction::Up).move_key(Direction::Up).digit());
        assert_eq!(7,
                   pos.move_key(Direction::Down)
                       .move_key(Direction::Down)
                       .move_key(Direction::Left)
                       .move_key(Direction::Left)
                       .digit());
        assert_eq!(6,
                   pos.move_key(Direction::Right).move_key(Direction::Right).digit());
        assert_eq!(4,
                   pos.move_key(Direction::Left)
                       .move_key(Direction::Left)
                       .move_key(Direction::Left)
                       .digit());
    }

    #[test]
    fn move_by_direction_string() {
        let pos = KeypadPos::new();
        assert_eq!(1, pos.move_by("ULL").digit());
    }
}
