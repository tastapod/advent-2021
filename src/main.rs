mod input;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    day1();
    day2();
    day3();
    day4();
    day5();
    day6();
    day7();
    day8();
}

fn day1() {
    let report = day1::input();
    println!("Day 1 part 1: {}", day1::count_increases(&report)); // 1393

    println!(
        "Day 1 part 2: {}",
        day1::count_increases(&day1::sum_triples(&report))
    ); // 1359
}

fn day2() {
    let input = &input::for_day(2);
    let pos = day2::navigate(&input);
    println!("Day 2 part 1: {:?} has product {}", pos, pos.product());
    
    let pos = day2::navigate_with_aim(&input);
    println!("Day 2 part 2: {:?} has product {}", pos, pos.product());
}

fn day3() {
    use day3::part1;
    use day3::part2;
    
    let input = input::for_day(3);
    let rates = part1::calculate_rates(&input);
    println!("Day 3 part 1: power consumption = {}", rates.product());
    
    let nodes = part2::BitNode::from(&input);
    println!("Day 3 part 2: life support rating = {}", part2::life_support_rating(&nodes, input[0].len()))
}

fn day4() {
    let input = input::for_day(4);
    let mut game = day4::Game::from_strings(&input);
    println!("Day 4 part 1: score = {}", game.play_to_win().unwrap());

    let mut game = day4::Game::from_strings(&input);
    println!("Day 4 part 2: score = {}", game.play_to_lose().unwrap());
}

fn day5() {
    let input = input::for_day(5);
    let vents_map = day5::VentsMap::from_strings(&input, false);
    println!("Day 5 part 1: found {} dangerous points", vents_map.count_dangerous_areas());

    let vents_map = day5::VentsMap::from_strings(&input, true);
    println!("Day 5 part 2: found {} dangerous points", vents_map.count_dangerous_areas());
}

fn day6() {
    let input = input::for_day(6);

    let mut school = day6::School::from_string(&input[0]);
    println!("Day 6 part 1: school has {} fish", school.to_day(80).count());

    let mut school = day6::School::from_string(&input[0]);
    println!("Day 6 part 2: school has {} fish", school.to_day(256).count());
}

fn day7() {
    let histogram = day7::histogram(&input::for_day(7)[0]);

    let (col, cost) = day7::linear_least_cost(&histogram);
    println!("Day 7 part 1: column {} has cost {}", col, cost);
    
    let (col, cost) = day7::stepped_least_cost(&histogram);
    println!("Day 7 part 2: column {} has cost {}", col, cost);
}

fn day8() {
    let input = input::for_day(8);
    let lines = input.iter().map(|s| &s as &str).collect::<Vec<&str>>();
    println!("Day 8 part 1: found {} unusual lengths", day8::count_outputs_with_lengths(&lines, &[2, 4, 3, 7]));
}
