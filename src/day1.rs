use itertools::izip;
use std::str::FromStr;

pub fn input() -> Vec<i32> {
    include_str!("day1_input.txt")
        .split_whitespace()
        .map(|s| i32::from_str(s).unwrap())
        .collect::<Vec<i32>>()
}

pub fn count_increases(report: &[i32]) -> i32 {
    report
        .iter()
        .zip(report[1..].iter())
        .fold(0, |acc, (l, r)| if r > l { acc + 1 } else { acc })
}

pub fn sum_triples(report: &[i32]) -> Vec<i32> {
    assert!(
        report.len() >= 3,
        "require at least three elements, found {}",
        report.len()
    );
    izip!(report, report[1..].iter(), report[2..].iter())
        .map(|(a, b, c)| a + b + c)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::count_increases;
    use super::sum_triples;

    const TEST_REPORT: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn counts_number_of_increases() {
        assert_eq!(count_increases(TEST_REPORT), 7);
    }

    #[test]
    fn creates_three_item_window() {
        let expected = [607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(sum_triples(TEST_REPORT).as_slice(), expected);
    }
}
