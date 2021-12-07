//mod day02B;
use serde::Deserialize;

pub mod day02A;
pub mod day02B;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstructionDirection {
    Forward,
    Down,
    Up,
}

pub struct Instruction {
    direction: InstructionDirection,
    amount: usize,
}
