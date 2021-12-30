#[derive(Debug, PartialEq)]
pub struct Rates {
    gamma: String,
    epsilon: String,
}

impl Rates {
    pub fn product(&self) -> i32 {
        let parse = |value| i32::from_str_radix(value, 2).unwrap();
        let gamma = parse(&self.gamma);
        let epsilon = parse(&self.epsilon);
        gamma * epsilon
    }
}

pub fn calculate_rates(report: &[&str]) -> Rates {
    fn gamma(counts: &Vec<i32>, max: usize) -> String {
        let half = i32::try_from(max / 2).unwrap();
        counts
            .iter()
            .map(|total| if *total > half { '1' } else { '0' })
            .collect::<String>()
    }
    assert!(report.len() > 0, "need at least one report entry");
    let counts = vec![0; report[0].len()];
    let counts = report
        .iter()
        .fold(counts, |counts, entry| add_entry(counts, entry));

    let gamma = gamma(&counts, report.len());
    let epsilon = gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();

    Rates {
        gamma: gamma,
        epsilon: epsilon,
    }
}

fn add_entry(counts: Vec<i32>, entry: &str) -> Vec<i32> {
    counts
        .iter()
        .zip(entry.chars().map(|bit| if bit == '1' { 1 } else { 0 }))
        .map(|(bit, count)| bit + count)
        .collect::<Vec<i32>>()
}

pub fn input() -> Vec<&'static str> {
    include_str!("day3_input.txt")
        .lines()
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_ones() {
        let entry = "10011";
        let counts = vec![3, 3, 3, 3, 3];

        assert_eq!(add_entry(counts, entry), vec![4, 3, 3, 4, 4]);
    }

    #[test]
    fn calculates_rates() {
        let report = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let rates = calculate_rates(&report);

        assert_eq!(
            rates,
            Rates {
                gamma: String::from("10110"),
                epsilon: String::from("01001"),
            }
        );
        assert_eq!(rates.product(), 198)
    }
}
