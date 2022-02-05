pub struct School {
    pub day: i32,
    pub fish: Vec<i32>,
}

impl School {
    pub fn from_string(fish_str: &str) -> School {
        School {
            day: 0,
            fish: fish_str
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        }
    }

    pub fn next_day(&mut self) -> &School {
        let mut new_fish: Vec<i32> = vec![];
        for i in 0..self.fish.len() {
            if self.fish[i] == 0 {
                self.fish[i] = 6;
                new_fish.push(8);
            } else {
                self.fish[i] -= 1;
            }
        }
        self.fish.extend(new_fish);
        self.day += 1;
        self
    }

    pub fn to_day(&mut self, end_day: i32) -> &School {
        while self.day < end_day {
            self.next_day();
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_generation() {
        let mut school = School::from_string("3,4,3,1,2");
        assert_eq!(vec![2, 3, 2, 0, 1], school.next_day().fish);
        assert_eq!(vec![1, 2, 1, 6, 0, 8], school.next_day().fish);
        assert_eq!(26, school.to_day(18).fish.len());
        assert_eq!(5934, school.to_day(80).fish.len());
    }
}
