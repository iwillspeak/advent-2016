extern crate onig;

use std::fmt;

use self::onig::*;

/// A display command
trait Command {
    /// Apply a command to the display.
    fn apply(&self, display: &mut PixelBuffer);
}

/// A pixel buffer
trait PixelBuffer {
    fn get(&self, x: usize, y: usize) -> bool;
    fn set(&mut self, x: usize, y: usize, value: bool);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

/// Represents a display
pub struct Display {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

/// A command for drawing rectangles
#[derive(Debug, PartialEq)]
struct RectCommand(usize, usize);

/// A command for rotating columns
#[derive(Debug, PartialEq)]
struct RotateColCommand(usize, usize);

/// A command for rotating rows
#[derive(Debug, PartialEq)]
struct RotateRowCommand(usize, usize);

impl<'a> From<&'a str> for RectCommand {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"(\d+)x(\d+)").unwrap();
        let caps = re.captures(s).expect("not a rect command!");
        let x = caps.at(1).unwrap().parse().unwrap();
        let y = caps.at(2).unwrap().parse().unwrap();
        RectCommand(x, y)
    }
}

impl<'a> From<&'a str> for RotateColCommand {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"column x=(\d+) by (\d+)").unwrap();
        let caps = re.captures(s).expect("not a rotate col command!");
        let x = caps.at(1).unwrap().parse().unwrap();
        let dist = caps.at(2).unwrap().parse().unwrap();
        RotateColCommand(x, dist)
    }
}

impl<'a> From<&'a str> for RotateRowCommand {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"row y=(\d+) by (\d+)").unwrap();
        let caps = re.captures(s).expect("not a rotate col command!");
        let y = caps.at(1).unwrap().parse().unwrap();
        let dist = caps.at(2).unwrap().parse().unwrap();
        RotateRowCommand(y, dist)
    }
}

impl Command for RectCommand {
    fn apply(&self, buffer: &mut PixelBuffer) {
        for x in 0..self.0 {
            for y in 0..self.1 {
                buffer.set(x, y, true);
            }
        }
    }
}

impl Command for RotateColCommand {
    fn apply(&self, buffer: &mut PixelBuffer) {
        let dist = self.1 % buffer.height();
        for _ in 0..dist {
            let mut px = buffer.get(self.0, 0);
            for y in 1..buffer.height() {
                let cur_px = buffer.get(self.0, y);
                buffer.set(self.0, y, px);
                px = cur_px
            }
            buffer.set(self.0, 0, px);
        }
    }
}

impl Command for RotateRowCommand {
    fn apply(&self, buffer: &mut PixelBuffer) {
        let dist = self.1 % buffer.width();
        for _ in 0..dist {
            let mut px = buffer.get(0, self.0);
            for x in 1..buffer.width() {
                let cur_px = buffer.get(x, self.0);
                buffer.set(x, self.0, px);
                px = cur_px
            }
            buffer.set(0, self.0, px)
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new(50, 6)
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self.pixels
            .chunks(self.width)
            .map(|line| {
                line.iter()
                    .map(|px| if *px { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", formatted)
    }
}

impl PixelBuffer for Display {
    fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[(y * self.width) + x]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.pixels[(y * self.width) + x] = val
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Display {
    /// Create a new display.
    ///
    /// # Arguments
    ///  * `width` - The width of the display, in pixels
    ///  * `height` - The height of the display, in pixels
    fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        pixels.resize(width * height, false);
        Display {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn command_str(&mut self, s: &str) {
        let sp = s.find(" ").expect("invalid command");
        let (command, args) = s.split_at(sp);
        let args = &args[1..];
        match command {
            "rect" => self.command(RectCommand::from(args)),
            "rotate" => {
                let sp = args.find(" ").expect("invalid rotate args");
                let (ty, _)= args.split_at(sp);
                match ty {
                    "row" => self.command(RotateRowCommand::from(args)),
                    "column" => self.command(RotateColCommand::from(args)),
                    _ => panic!("invalid roration {} ({})", ty, args),
                }
            },
            _ => panic!("unknown comamnd!"),
        }
    }

    /// Run a command on the display
    fn command<C>(&mut self, cmd: C)
        where C: Command
    {
        cmd.apply(self)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use super::{RectCommand, RotateColCommand, RotateRowCommand};

    #[test]
    fn create_display() {
        let d = Display::new(100, 30);
        assert_eq!(100, d.width);
        assert_eq!(30, d.height);
    }

    #[test]
    fn default_display_size() {
        let display: Display = Default::default();
        assert_eq!(50, display.width);
        assert_eq!(6, display.height);
    }

    #[test]
    fn test_dump_empty_display() {
        let display = Display::new(5, 3);
        assert_eq!(".....
.....
.....",
                   display.to_string());
    }

    #[test]
    fn example_commands() {
        let mut display = Display::new(7, 3);
        display.command(RectCommand(3, 2));
        assert_eq!("###....
###....
.......",
                   display.to_string());
        display.command(RotateColCommand(1, 1));
        assert_eq!("#.#....
###....
.#.....",
                   display.to_string());
        display.command(RotateRowCommand(0, 4));
        assert_eq!("....#.#
###....
.#.....",
                   display.to_string());
        display.command(RotateColCommand(1, 1));
        assert_eq!(".#..#.#
#.#....
.#.....",
                   display.to_string());
    }

    #[test]
    fn parse_commands() {
        assert_eq!(RectCommand(3, 2), RectCommand::from("3x2"));
        assert_eq!(RotateColCommand(1, 1), RotateColCommand::from("column x=1 by 1"));
        assert_eq!(RotateRowCommand(0, 4), RotateRowCommand::from("row y=0 by 4"));
    }

    #[test]
    fn display_apply_command_string() {
        let mut display = Display::new(7, 3);
        display.command_str("rect 3x2");
        assert_eq!("###....
###....
.......",
                   display.to_string());
        display.command_str("rotate column x=1 by 1");
        assert_eq!("#.#....
###....
.#.....",
                   display.to_string());
        display.command_str("rotate row y=0 by 4");
        assert_eq!("....#.#
###....
.#.....",
                   display.to_string());
        display.command_str("rotate column x=1 by 1");
        assert_eq!(".#..#.#
#.#....
.#.....",
                   display.to_string());
    }
}
