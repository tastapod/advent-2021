fn extract_output_values<'a>(line: &'a str) -> Vec<&'a str> {
    line.split(" | ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
}

fn count_lengths(values: &[&str], lengths: &[usize]) -> usize {
    values
        .iter()
        .filter(|v| lengths.contains(&v.chars().count()))
        .count()
}

pub fn count_outputs_with_lengths(input: &[&str], lengths: &[usize]) -> usize {
    input
        .iter()
        .map(|line| extract_output_values(line))
        .map(|values| count_lengths(&values, lengths))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ]
    }

    #[test]
    fn extracts_output_values() {
        let line = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        assert_eq!(
            vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"],
            extract_output_values(line)
        );
    }

    #[test]
    fn counts_lengths() {
        let values = vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"];
        assert_eq!(3, count_lengths(&values, &[4, 6, 7]));
        assert_eq!(1, count_lengths(&values, &[3, 6, 8]));
    }

    #[test]
    fn counts_unusual_digits() {
        assert_eq!(26, count_outputs_with_lengths(&input(), &[2, 4, 3, 7]));
    }
}
