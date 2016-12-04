//! Day 2

use std::marker::PhantomData;

pub trait Keypad
    where Self: Clone
{
    /// Check if the given position is valid on this keypad.
    fn is_on_keypad(pos: &KeypadPos<Self>) -> bool;

    /// Get the starting position on the Keypad.
    ///
    /// Returns a new `KeypadPos` on this `Keypad` pointing at the
    /// digit '5'.
    fn start_pos() -> KeypadPos<Self>;

    /// Get the digit at the given position
    fn digit_at(pos: &KeypadPos<Self>) -> char;

    fn pin_from(instructions: &str) -> String {
        instructions.split("\n")
            .scan(Self::start_pos(), |pos, line| {
                *pos = pos.move_by(line);
                Some(pos.digit())
            })
            .collect::<String>()
    }
}

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
pub struct KeypadPos<K: ?Sized + Keypad + Clone> {
    accross: i32,
    down: i32,
    _keypad: PhantomData<K>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NineDigitKeypad;

#[derive(Debug, PartialEq, Clone)]
pub struct DiamondKeypad;

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

impl<K> KeypadPos<K>
    where K: Keypad + Clone
{
    /// Move to the next key
    pub fn move_key(&self, dir: Direction) -> Self {
        let next_pos = match dir {
            Direction::Up => KeypadPos { down: self.down - 1, ..*self },
            Direction::Down => KeypadPos { down: self.down + 1, ..*self },
            Direction::Left => KeypadPos { accross: self.accross - 1, ..*self },
            Direction::Right => KeypadPos { accross: self.accross + 1, ..*self },
        };
        if K::is_on_keypad(&next_pos) {
            next_pos
        } else {
            self.clone()
        }
    }

    /// Move by a direction string
    pub fn move_by(&self, directions: &str) -> Self {
        directions.chars()
            .map(|c| Direction::from(c))
            .fold(self.clone(), |pos, dir| pos.move_key(dir))
    }

    /// Get the digit at the keypad's current position.
    pub fn digit(&self) -> char {
        K::digit_at(self)
    }
}

impl Keypad for NineDigitKeypad {
    fn is_on_keypad(pos: &KeypadPos<Self>) -> bool {
        if pos.accross < 0 || pos.down < 0 {
            return false;
        }
        if pos.accross > 2 || pos.down > 2 {
            return false;
        }
        true
    }

    fn start_pos() -> KeypadPos<Self> {
        KeypadPos {
            accross: 1,
            down: 1,
            _keypad: PhantomData,
        }
    }

    fn digit_at(pos: &KeypadPos<Self>) -> char {
        const DIGITS: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
        DIGITS[pos.down as usize][pos.accross as usize]
    }
}

impl Keypad for DiamondKeypad {
    fn is_on_keypad(pos: &KeypadPos<Self>) -> bool {
        let h = pos.accross - 2;
        let v = pos.down - 2;
        let abs = h.abs() + v.abs();
        abs <= 2
    }

    fn start_pos() -> KeypadPos<Self> {
        KeypadPos {
            accross: 0,
            down: 2,
            _keypad: PhantomData,
        }
    }

    fn digit_at(pos: &KeypadPos<Self>) -> char {
        const DIGITS: [[char; 5]; 5] = [[' ', ' ', '1', ' ', ' '],
                                        [' ', '2', '3', '4', ' '],
                                        ['5', '6', '7', '8', '9'],
                                        [' ', 'A', 'B', 'C', ' '],
                                        [' ', ' ', 'D', ' ', ' ']];
        let digit = DIGITS[pos.down as usize][pos.accross as usize];
        assert_ne!(digit, ' ', "at {:?}", pos);
        digit
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn create_pos() -> KeypadPos<NineDigitKeypad> {
        NineDigitKeypad::start_pos()
    }

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
        let pos = create_pos();
        assert_eq!('5', pos.digit());
    }

    #[test]
    fn diamond_keypad_start_at_5() {
        let pos = DiamondKeypad::start_pos();
        assert_eq!('5', pos.digit());
    }

    #[test]
    fn move_keypad_pos() {
        let pos = create_pos();
        assert_eq!('2', pos.move_key(Direction::Up).digit());
        assert_eq!('4', pos.move_key(Direction::Left).digit());
        assert_eq!('6', pos.move_key(Direction::Right).digit());
        assert_eq!('8', pos.move_key(Direction::Down).digit());
    }

    #[test]
    fn move_keypad_pos_saturates() {
        let pos = create_pos();
        assert_eq!('2',
                   pos.move_key(Direction::Up).move_key(Direction::Up).digit());
        assert_eq!('7',
                   pos.move_key(Direction::Down)
                       .move_key(Direction::Down)
                       .move_key(Direction::Left)
                       .move_key(Direction::Left)
                       .digit());
        assert_eq!('6',
                   pos.move_key(Direction::Right).move_key(Direction::Right).digit());
        assert_eq!('4',
                   pos.move_key(Direction::Left)
                       .move_key(Direction::Left)
                       .move_key(Direction::Left)
                       .digit());
    }

    #[test]
    fn move_diamond_saturates() {
        let pos = DiamondKeypad::start_pos();
        assert_eq!('5', pos.move_by("L").digit());
    }

    #[test]
    fn move_by_direction_string() {
        let pos = create_pos();
        assert_eq!('1', pos.move_by("ULL").digit());
    }

    const INSTRUCTIONS: &'static str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn example_works() {
        let pin = NineDigitKeypad::pin_from(INSTRUCTIONS);
        assert_eq!("1985", pin);
    }

    #[test]
    fn example_diamond_works() {
        let pin = DiamondKeypad::pin_from(INSTRUCTIONS);
        assert_eq!("5DB3", pin);
    }

}
