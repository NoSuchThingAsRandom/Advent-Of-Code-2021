mod day01;
mod day02;
mod misc;
mod day03;
mod day04;

pub const EXAMPLE_INPUT_PATH: &str = "example_inputs/day";
pub const INPUT_PATH: &str = "inputs/day";

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    /*    println!("Day 01, Part A: {:?}", day01::part_1(false));
        println!("Day 01, Part B: {:?}", day01::part_2(false)?);

        println!("Day 02, Part A: {:?}", day02::day02A::part_1(false));
        println!("Day 02, Part B: {:?}", day02::day02B::part_2(false));

    println!("Day 03, Part A: {:?}", day03::part_1(false));
    println!("Day 03, Part B: {:?}", day03::part_2(false));
    */
    println!("Day 04, Part A: {:?}", day04::part_1(false));
    println!("Day 04, Part B: {:?}", day04::part_2(false));
    Ok(())
}
