pub fn histogram(input: &str) -> Vec<isize> {
    let crabs = input
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let max = crabs.iter().max().unwrap();
    crabs.iter().fold(vec![0; *max + 1], |mut acc, crab| {
        acc[*crab] += 1;
        acc
    })
}

type FuelCostFn = fn(isize, isize) -> isize;

fn linear_cost(a: isize, b: isize) -> isize {
    (a - b).abs()
}

fn stepped_cost(a: isize, b: isize) -> isize {
    let dist = (a - b).abs();
    dist * (dist + 1) / 2
}

pub fn fuel_cost(histogram: &[isize], pos: isize, fuel_cost_fn: FuelCostFn) -> isize {
    histogram.iter().enumerate().fold(0, |acc, (i, count)| {
        acc + count * fuel_cost_fn(pos, i as isize)
    })
}

fn least_cost(histogram: &[isize], fuel_cost_fn: FuelCostFn) -> (usize, isize) {
    (0..histogram.len())
        .map(|i| (i, fuel_cost(histogram, i as isize, fuel_cost_fn)))
        .min_by(|(_, cost1), (_, cost2)| cost1.cmp(cost2))
        .unwrap()
}

pub fn linear_least_cost(histogram: &[isize]) -> (usize, isize) {
    least_cost(histogram, linear_cost)
}

pub fn stepped_least_cost(histogram: &[isize]) -> (usize, isize) {
    least_cost(histogram, stepped_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn builds_histogram() {
        let histogram = histogram(input());
        assert_eq!(17, histogram.len());
        assert_eq!([1, 2, 3], histogram[0..=2]);
    }

    #[test]
    fn calculates_linear_fuel_cost() {
        let histogram = histogram(input());

        assert_eq!(37, fuel_cost(&histogram, 2, linear_cost));
        assert_eq!(41, fuel_cost(&histogram, 1, linear_cost));
        assert_eq!(39, fuel_cost(&histogram, 3, linear_cost));
    }

    #[test]
    fn calculates_stepped_fuel_cost() {
        let histogram = histogram(input());
        assert_eq!(206, fuel_cost(&histogram, 2, stepped_cost));
        assert_eq!(168, fuel_cost(&histogram, 5, stepped_cost));
    }

    #[test]
    fn finds_linear_least_cost() {
        let histogram = histogram(input());
        assert_eq!((2, 37), linear_least_cost(&histogram));
    }

    #[test]
    fn finds_stepped_least_cost() {
        let histogram = histogram(input());
        assert_eq!((5, 168), stepped_least_cost(&histogram));
    }
}
