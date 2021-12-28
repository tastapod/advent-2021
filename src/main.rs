mod day1;

fn day1() {
    println!("Day 1 part 1: {}", day1::count_increases(day1::REPORT)); // 1393
    println!(
        "Day 1 part 2: {}",
        day1::count_increases(day1::sum_triples(day1::REPORT).as_slice())
    ); // 1359
}

fn main() {
    day1();
}
