use std::error::Error;
use std::fs;

struct Dial {
    current_position: i32,
    max_num_on_dial: i32,
    dial_pointed_zero: i32,
    dial_passed_zero: i32,
}

impl Dial {
    fn new(starting_position: i32, max_dial_position: i32) -> Self {
        Dial {
            current_position: starting_position,
            max_num_on_dial: max_dial_position,
            dial_pointed_zero: 0,
            dial_passed_zero: 0,
        }
    }

    fn turn(&mut self, ticks: i32) -> i32 {
        for _v in 0..ticks.abs() {
            if self.current_position == 0 {
                self.dial_passed_zero += 1;
            }
            self.current_position += ticks / ticks.abs();
            if self.current_position == 100 {
                self.current_position = 0
            }
            if self.current_position == -1 {
                self.current_position = 99
            }
        }
        if self.current_position == 0 {
            self.dial_pointed_zero += 1;
        }
        self.current_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right() {
        let mut dial = Dial::new(50, 99);
        assert_eq!(dial.turn(20), 70);
    }

    #[test]
    fn test_turn_right_to_zero() {
        let mut dial = Dial::new(95, 99);
        assert_eq!(dial.turn(5), 0);
    }

    #[test]
    fn test_turn_right_over_max() {
        let mut dial = Dial::new(95, 99);
        assert_eq!(dial.turn(6), 1);
    }

    #[test]
    fn test_turn_right_over_max_2() {
        let mut dial = Dial::new(95, 99);
        assert_eq!(dial.turn(406), 1);
    }

    #[test]
    fn test_turn_left() {
        let mut dial = Dial::new(50, 99);
        assert_eq!(dial.turn(-20), 30);
    }

    #[test]
    fn test_turn_left_to_zero() {
        let mut dial = Dial::new(5, 99);
        assert_eq!(dial.turn(-5), 0);
    }

    #[test]
    fn test_turn_left_over_min() {
        let mut dial = Dial::new(10, 99);
        assert_eq!(dial.turn(-11), 99);
    }

    #[test]
    fn test_turn_1000_ticks_and_check_passed_zero() {
        let mut dial = Dial::new(50, 99);
        assert_eq!(dial.turn(1000), 50);
        assert_eq!(dial.dial_passed_zero, 10);
    }

    #[test]
    fn test_turn_1000_ticks_back_and_check_passed_zero() {
        let mut dial = Dial::new(50, 99);
        assert_eq!(dial.turn(-1000), 50);
        assert_eq!(dial.dial_passed_zero, 10);
    }

    #[test]
    fn test_turn_1000_ticks_and_check_passed_zero_then_pointed_zero() {
        let mut dial = Dial::new(0, 99);
        assert_eq!(dial.turn(1000), 0);
        assert_eq!(dial.dial_passed_zero, 10);
    }

    #[test]
    fn test_turn_1000_ticks_back_and_check_passed_zero_then_pointed_zero() {
        let mut dial = Dial::new(0, 99);
        assert_eq!(dial.turn(-1000), 0);
        assert_eq!(dial.dial_passed_zero, 10);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input";
    let f = fs::read_to_string(path)?;
    let mut dial = Dial::new(50, 99);
    for line in f.lines() {
        let (direction, amount) = line.split_at(1);
        let amount: i32 = amount.parse()?;
        let direction = match direction {
            "R" => 1,
            "L" => -1,
            &_ => todo!(),
        };
        dial.turn(direction * amount);
    }

    println!("{}", dial.dial_pointed_zero);
    println!("{}", dial.dial_passed_zero);

    Ok(())
}
