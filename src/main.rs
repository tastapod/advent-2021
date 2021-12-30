mod day1;
mod day2;
mod day3;

fn day1() {
    let report = day1::input();
    println!("Day 1 part 1: {}", day1::count_increases(&report)); // 1393

    println!(
        "Day 1 part 2: {}",
        day1::count_increases(&day1::sum_triples(&report))
    ); // 1359
}

fn day2() {
    let input = day2::input();
    let pos = day2::navigate(&input);
    println!("Day 2 part 1: {:?} has product {}", pos, pos.product());

    let pos = day2::navigate_with_aim(&input);
    println!("Day 2 part 2: {:?} has product {}", pos, pos.product());
}

fn day3() {
    let input = day3::input();
    let rates = day3::calculate_rates(&input);
    println!("Day 3 part 1: power consumption = {}", rates.product());
}

fn main() {
    day1();
    day2();
    day3();
}
