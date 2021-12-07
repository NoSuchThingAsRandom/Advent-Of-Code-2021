mod day01;
mod misc;

pub const EXAMPLE_INPUT_PATH: &str = "example_inputs/day";
pub const INPUT_PATH: &str = "inputs/day";

fn main() {
    pretty_env_logger::init();
    println!("Hello, world!");
    //println!("Day 01, Part A: {:?}",day01::part_1(false));
    println!("Day 01, Part B: {:?}", day01::part_2(false));
}
