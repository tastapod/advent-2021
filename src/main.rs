mod day1;

fn main() {
    println!("{}", count_increases(day1::part1::report));
}

pub fn count_increases(report: &[i32]) -> i32 {
    report
        .iter()
        .zip(report[1..].iter())
        .fold(0, |acc, (l, r)| if r > l { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::count_increases;

    #[test]
    fn counts_number_of_increases() {
        let report: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(report), 7);
    }
}
