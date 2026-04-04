mod utils;

// mod day1;
// use day1::solve;

// mod day2;
// use day2::solve;

// mod day3;
// use day3::solve;

// mod day4;
// use day4::solve;

// mod day5;
// use day5::solve;

// mod day6;
// use day6::solve;

// mod day7;
// use day7::solve;

// mod day9;
// use day9::solve;

// mod day13;
// use day13::solve;

mod day14;
use day14::solve;

// mod day25;
// use day25::solve;

fn main() {
    match solve() {
        Err(err) => println!("{}", err),
        _ => {}
    }
}
