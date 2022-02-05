use std::collections::HashMap;

type Point = (isize, isize);

#[derive(Default)]
pub struct VentsMap {
    pub points: HashMap<Point, i32>,
}

impl VentsMap {
    pub fn add(&mut self, point: Point) {
        let entry = self.points.entry(point).or_insert(0);
        *entry += 1;
    }

    pub fn from_strings(input: &Vec<String>, include_diagonals: bool) -> VentsMap {
        let mut result = VentsMap::default();

        for line in input {
            let ends = line
                .split(" -> ")
                .map(|s| parse_point(s))
                .collect::<Vec<Point>>();

            let (start, end) = (ends[0], ends[1]);

            if start.0 == end.0 || start.1 == end.1 {
                // horizontal or vertical line
                for x in abs_range(start.0, end.0) {
                    for y in abs_range(start.1, end.1) {
                        result.add((x, y));
                    }
                }
            } else {
                // diagonal line
                if include_diagonals {
                    for point in Diagonal::new(start, end) {
                        result.add(point);
                    }
                }
            }
        }
        result
    }

    pub fn count_dangerous_areas(&self) -> usize {
        self.points.values().filter(|c| *c > &1).count()
    }
}

fn parse_point(point_str: &str) -> Point {
    let coords = point_str
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    (coords[0], coords[1])
}

fn abs_range(a: isize, b: isize) -> std::ops::RangeInclusive<isize> {
    a.min(b)..=a.max(b)
}

#[derive(Debug)]
struct Diagonal {
    end: Point,
    x_delta: isize,
    y_delta: isize,
    current: Point,
}

impl Diagonal {
    pub fn new(start: Point, end: Point) -> Diagonal {
        let x_delta = (end.0 - start.0).signum();
        let y_delta = (end.1 - start.1).signum();

        Diagonal {
            end: end,
            x_delta: x_delta,
            y_delta: y_delta,
            current: (start.0 - x_delta, start.1 - y_delta),
        }
    }
}

impl Iterator for Diagonal {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            self.current = (self.current.0 + self.x_delta, self.current.1 + self.y_delta);
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .map(|s| String::from(s))
        .to_vec()
    }

    #[test]
    fn creates_map_of_points() {
        let input = input();
        let vents_map = VentsMap::from_strings(&input, false);

        assert_eq!(vents_map.points[&(1, 9)], 2);
        assert_eq!(5, vents_map.count_dangerous_areas());
    }

    #[test]
    fn generates_diagonal_points() {
        assert_eq!(
            Diagonal::new((6, 4), (2, 0)).collect::<Vec<Point>>(),
            vec![(6, 4), (5, 3), (4, 2), (3, 1), (2, 0)]
        );
    }

    #[test]
    fn finds_dangerous_areas_with_diagonals() {
        let input = input();
        let vents_map = VentsMap::from_strings(&input, true);
        assert_eq!(12, vents_map.count_dangerous_areas());
    }
}
