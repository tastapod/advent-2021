use std::str::FromStr;

type Distance = i32;

#[derive(Debug, PartialEq)]
pub struct Position {
    horizontal: Distance,
    depth: Distance,
    aim: Distance,
}

impl Position {
    pub fn product(&self) -> i32 {
        self.horizontal * self.depth
    }
}

const START_POSITION: Position = Position {
    horizontal: 0,
    depth: 0,
    aim: 0,
};

enum Step {
    Forward(Distance),
    Down(Distance),
    Up(Distance),
}

impl Step {
    fn apply_simple(&self, start: Position) -> Position {
        match self {
            Step::Forward(distance) => Position {
                horizontal: start.horizontal + distance,
                ..start
            },
            Step::Down(distance) => Position {
                depth: start.depth + distance,
                ..start
            },
            Step::Up(distance) => Position {
                depth: start.depth - distance,
                ..start
            },
        }
    }

    fn apply_with_aim(&self, start: Position) -> Position {
        match self {
            Step::Forward(dist) => Position {
                horizontal: start.horizontal + dist,
                depth: start.depth + start.aim * dist,
                ..start
            },
            Step::Down(dist) => Position {
                aim: start.aim + dist,
                ..start
            },
            Step::Up(dist) => Position {
                aim: start.aim - dist,
                ..start
            },
        }
    }
}

fn parse_step_def(step_def: &String) -> Step {
    let split = step_def.split_whitespace().collect::<Vec<&str>>();
    let step_type = split[0];
    let distance = i32::from_str(split[1]).unwrap();

    match step_type {
        "forward" => Step::Forward(distance),
        "up" => Step::Up(distance),
        "down" => Step::Down(distance),
        _ => panic!("Unknown step type: {}", step_type),
    }
}

pub fn navigate(step_defs: &Vec<String>) -> Position {
    navigate_with_strategy(step_defs, &Step::apply_simple)
}

pub fn navigate_with_aim(step_defs: &Vec<String>) -> Position {
    navigate_with_strategy(step_defs, &Step::apply_with_aim)
}

fn navigate_with_strategy(
    step_defs: &Vec<String>,
    strategy: &dyn Fn(&Step, Position) -> Position,
) -> Position {
    step_defs
        .iter()
        .map(parse_step_def)
        .fold(START_POSITION, |pos, step| strategy(&step, pos))
}

#[cfg(test)]
mod tests {
    use super::navigate;
    use super::Position;

    #[test]
    fn applies_steps() {
        let step_defs = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .map(|s| s.to_string())
        .to_vec();
        let end_pos = navigate(&step_defs);

        assert_eq!(
            end_pos,
            Position {
                horizontal: 15,
                depth: 10,
                aim: 0,
            }
        );
        assert_eq!(end_pos.product(), 150);
    }
}
