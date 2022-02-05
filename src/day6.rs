pub struct School {
    pub day: i32,
    pub fish_counts: [i64; 9],
}

impl School {
    pub fn from_string(fish_str: &str) -> School {
        let mut fish_counts = [0; 9];
        for n in fish_str.split(",") {
            fish_counts[n.parse::<usize>().unwrap()] += 1;
        }

        School {
            day: 0,
            fish_counts: fish_counts,
        }
    }

    pub fn next_day(&mut self) -> &School {
        let mut next_counts = [0; 9];

        // gestation periods get a day shorter
        for i in 1..=8 {
            next_counts[i - 1] = self.fish_counts[i];
        }

        // day 0 fish have babies and reset their cycle
        next_counts[8] = self.fish_counts[0];
        next_counts[6] += self.fish_counts[0];

        self.day += 1;
        self.fish_counts = next_counts;

        self
    }

    pub fn to_day(&mut self, end_day: i32) -> &School {
        while self.day < end_day {
            self.next_day();
        }
        self
    }

    pub fn count(&self) -> i64 {
        self.fish_counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_generation() {
        let mut school = School::from_string("3,4,3,1,2");
        assert_eq!(5, school.next_day().count());
        assert_eq!(6, school.next_day().count());
        assert_eq!(26, school.to_day(18).count());
        assert_eq!(5934, school.to_day(80).count());
        assert_eq!(26984457539, school.to_day(256).count());
    }
}
