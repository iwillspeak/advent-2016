/// Simple Mathematics encoded as Robot Dancing

use onig::*;

type Value = usize;
type RobotId = usize;

/// Represets a command to a robot. Robots will make a decision on
/// what to do with each value they have when they have two of them.
struct Robot {
    left: Option<Value>,
    right: Option<Value>,
}

/// A command representing passing an input value to a robot.
#[derive(Debug, PartialEq)]
struct ValueCommand(RobotId, Value);

/// A command representing what a robot should do when it has two
/// inputs.
#[derive(Debug, PartialEq)]
struct RobotCommand{
    robot: RobotId,
    high: RobotId,
    low: RobotId,
}

impl<'a> From<&'a str> for ValueCommand {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        ValueCommand(
            caps.at(1).unwrap().parse().unwrap(),
            caps.at(2).unwrap().parse().unwrap(),
        )
    }
}

impl<'a> From<&'a str> for RobotCommand {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"bot (\d+) gives low to () (\d+) and high to bot (\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        RobotCommand {
            robot: caps.at(1).unwrap().parse().unwrap(),
            low: caps.at(2).unwrap().parse().unwrap(),
            high: caps.at(3).unwrap().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::{RobotCommand, ValueCommand};

    #[test]
    fn parse_value_command() {
        assert_eq!(ValueCommand(7, 107), ValueCommand::from("value 7 goes to bot 107"));
    }

    #[test]
    fn parse_robot_command() {
        assert_eq!(RobotCommand{ robot: 2, low: 1, high: 0 }, RobotCommand::from("bot 2 gives low to bot 1 and high to bot 0"));
    }

    #[test]
    fn run_example_bot_state() {
        let state = BotState::from("value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
");
        state.run();
    }
}
