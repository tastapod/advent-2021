
pub fn build_histogram(input: &str) -> Vec<isize> {
    let crabs = input.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let max = crabs.iter().max().unwrap();
    crabs.iter().fold(vec![0; *max+1], |mut acc, crab| {acc[*crab] += 1; acc})
}

pub fn fuel_cost(histogram: &[isize], pos: isize) -> isize {
    (0..).zip(histogram).fold(0, |acc, (i, count)| acc + count * (pos - i).abs())
}

pub fn least_cost(histogram: &[isize]) -> (usize, isize) {
    (0..histogram.len()).map(|i| (i, fuel_cost(histogram, i as isize))).min_by(|(_, cost1), (_, cost2)| cost1.cmp(cost2)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn builds_histogram() {
        let histogram = build_histogram(input());
        assert_eq!(17, histogram.len());
        assert_eq!([1,2, 3], histogram[0..=2]);
    }

    #[test]
    fn calculates_fuel_cost() {
        let histogram = build_histogram(input());

        assert_eq!(37, fuel_cost(&histogram, 2));
        assert_eq!(41, fuel_cost(&histogram, 1));
        assert_eq!(39, fuel_cost(&histogram, 3));
    }

    #[test]
    fn finds_least_cost() {
        let histogram = build_histogram(input());
        assert_eq!((2, 37), least_cost(&histogram));
    }
}
