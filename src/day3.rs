//! Solution for Day 3
//!
//! Notes for part 2
//!
//! Think of each n-digit binary string as a path through a tree of nodes,
//! each with two children, 1 and 0. The binary string then walks the tree,
//! incrementing each node it touches.

pub fn input() -> Vec<&'static str> {
    include_str!("day3_input.txt")
        .lines()
        .collect::<Vec<&str>>()
}

pub mod part1 {

    #[derive(Debug, PartialEq)]
    pub struct Rates {
        pub gamma: String,
        pub epsilon: String,
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

    pub fn add_entry(counts: Vec<i32>, entry: &str) -> Vec<i32> {
        counts
            .iter()
            .zip(entry.chars().map(|bit| if bit == '1' { 1 } else { 0 }))
            .map(|(bit, count)| bit + count)
            .collect::<Vec<i32>>()
    }
}

pub mod part2 {

    #[derive(Debug, PartialEq, Default)]
    pub struct BitNode {
        pub count: i32,
        nodes: [Option<Box<BitNode>>; 2],
    }

    const EMPTY_NODE: BitNode = BitNode {
        count: 0,
        nodes: [None, None],
    };

    impl BitNode {
        pub fn get(&self, bit: usize) -> &BitNode {
            if let None = self.nodes[bit] {
                &EMPTY_NODE
            } else {
                self.nodes[bit].as_deref().unwrap()
            }
        }

        fn get_mut(&mut self, bit: usize) -> &mut BitNode {
            self.nodes[bit].get_or_insert_with(|| Box::new(BitNode::default()))
        }
    }

    fn add_entry_to_node_tree(root: &mut BitNode, entry: &str) {
        let mut node = root;
        node.count += 1;
        for ch in entry.chars() {
            let bit = usize::try_from(ch.to_digit(2).unwrap()).unwrap();
            node = node.get_mut(bit);
            node.count += 1;
        }
    }

    fn calculate_rating<'a>(
        bit_tree: &'a BitNode,
        num_bits: usize,
        choose_child: &dyn Fn(&(&'a BitNode, &'a BitNode)) -> (char, &'a BitNode),
    ) -> String {
        let mut result: Vec<char> = Vec::new();
        let mut node = bit_tree;
        for _ in 0..num_bits {
            let child: (char, &BitNode);
            let nodes = (node.get(0), node.get(1));

            child = if nodes.0.count + nodes.1.count == 1 {
                // only one node so choose it!
                if nodes.0.count == 1 {
                    ('0', nodes.0)
                } else {
                    ('1', nodes.1)
                }
            } else {
                choose_child(&nodes)
            };
            result.push(child.0);
            node = child.1;
        }
        String::from_iter(result)
    }

    pub fn o2_generator_rating(bit_tree: &BitNode, num_bits: usize) -> String {
        calculate_rating(bit_tree, num_bits, &choose_o2_generator)
    }

    fn choose_o2_generator<'a>(nodes: &(&'a BitNode, &'a BitNode)) -> (char, &'a BitNode) {
        if nodes.1.count >= nodes.0.count {
            ('1', nodes.1)
        } else {
            ('0', nodes.0)
        }
    }

    pub fn co2_scrubber_rating(bit_tree: &BitNode, num_bits: usize) -> String {
        calculate_rating(bit_tree, num_bits, &choose_co2_scrubber)
    }

    fn choose_co2_scrubber<'a>(nodes: &(&'a BitNode, &'a BitNode)) -> (char, &'a BitNode) {
        if nodes.0.count <= nodes.1.count {
            ('0', nodes.0)
        } else {
            ('1', nodes.1)
        }
    }

    pub fn life_support_rating(bit_tree: &BitNode, num_bits: usize) -> i32 {
        let co2_scrubber =
            i32::from_str_radix(&co2_scrubber_rating(bit_tree, num_bits), 2).unwrap();
        let o2_generator =
            i32::from_str_radix(&o2_generator_rating(bit_tree, num_bits), 2).unwrap();
        co2_scrubber * o2_generator
    }

    /// BitNode::from(["list", "of", "entries"])
    impl From<&Vec<&str>> for BitNode {
        fn from(vec: &Vec<&str>) -> Self {
            let mut result = BitNode::default();
            for entry in vec {
                add_entry_to_node_tree(&mut result, entry);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part1::*;
    use super::part2::*;

    const REPORT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn counts_ones() {
        let entry = "10011";
        let counts = vec![3, 3, 3, 3, 3];

        assert_eq!(add_entry(counts, entry), vec![4, 3, 3, 4, 4]);
    }

    #[test]
    fn calculates_rates() {
        let rates = calculate_rates(&REPORT);

        assert_eq!(
            rates,
            Rates {
                gamma: String::from("10110"),
                epsilon: String::from("01001"),
            }
        );
        assert_eq!(rates.product(), 198)
    }

    #[test]
    fn builds_empty_bit_tree() {
        let entries = &vec![];
        let bit_tree = BitNode::from(entries);
        assert_eq!(bit_tree, BitNode::default())
    }

    #[test]
    fn builds_bit_tree() {
        let entries = &vec!["10", "11", "01"];
        let bit_tree = BitNode::from(entries);

        assert_eq!(3, bit_tree.count);

        assert_eq!(1, bit_tree.get(0).count);
        assert_eq!(0, bit_tree.get(0).get(0).count);
        assert_eq!(1, bit_tree.get(0).get(1).count);
        assert_eq!(0, bit_tree.get(0).get(1).get(0).count);
        assert_eq!(0, bit_tree.get(0).get(1).get(1).count);

        assert_eq!(2, bit_tree.get(1).count);
        assert_eq!(1, bit_tree.get(1).get(0).count);
        assert_eq!(1, bit_tree.get(1).get(1).count);
        assert_eq!(0, bit_tree.get(1).get(1).get(0).count);
        assert_eq!(0, bit_tree.get(1).get(1).get(1).count);
    }

    #[test]
    fn calculates_oxygen_generator_rating() {
        let bit_tree = BitNode::from(&Vec::from(REPORT));
        assert_eq!(o2_generator_rating(&bit_tree, REPORT[0].len()), "10111");
    }

    #[test]
    fn calculates_co2_scrubber_rating() {
        let bit_tree = BitNode::from(&Vec::from(REPORT));
        assert_eq!(co2_scrubber_rating(&bit_tree, REPORT[0].len()), "01010");
    }

    #[test]
    fn calculates_life_support_rating() {
        let bit_tree = BitNode::from(&Vec::from(REPORT));
        assert_eq!(life_support_rating(&bit_tree, REPORT[0].len()), 230);
    }
}
