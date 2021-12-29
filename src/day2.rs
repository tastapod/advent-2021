use std::str::FromStr;

type Distance = i32;

#[derive(Debug, Eq, PartialEq)]
pub struct Position {
    horizontal: Distance,
    depth: Distance,
}

impl Position {
    pub fn product(&self) -> i32 {
        self.horizontal * self.depth
    }
}

enum Step {
    Forward(Distance),
    Down(Distance),
    Up(Distance),
}

impl Step {
    fn apply(&self, start: Position) -> Position {
        match self {
            Step::Forward(dist) => Position {
                horizontal: start.horizontal + dist,
                ..start
            },
            Step::Down(dist) => Position {
                depth: start.depth + dist,
                ..start
            },
            Step::Up(dist) => Position {
                depth: start.depth - dist,
                ..start
            },
        }
    }
}

fn parse_step_def(step_def: &&str) -> Step {
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

pub fn navigate(step_defs: &[&str]) -> Position {
    step_defs.iter().map(parse_step_def).fold(
        Position {
            horizontal: 0,
            depth: 0,
        },
        |pos, step| step.apply(pos),
    )
}

pub fn input() -> Vec<&'static str> {
    include_str!("day2_input.txt")
        .lines()
        .collect::<Vec<&str>>()
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
        ];
        let end_pos = navigate(&step_defs);

        assert_eq!(
            end_pos,
            Position {
                horizontal: 15,
                depth: 10
            }
        );
        assert_eq!(end_pos.product(), 150);
    }
}
