use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::day02::{Instruction, InstructionDirection};
use crate::misc::error::AoCResult;
use crate::misc::read_vec_string;

struct AimPosition {
    vertical: usize,
    horizontal: usize,
    aim: usize,
}

impl AimPosition {
    pub fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction.direction {
            InstructionDirection::Forward => {
                self.horizontal += instruction.amount;
                self.vertical += self.aim * instruction.amount;
            }
            InstructionDirection::Down => self.aim += instruction.amount,
            InstructionDirection::Up => self.aim -= instruction.amount,
        }
    }
    pub fn output(&self) -> usize {
        self.horizontal * self.vertical
    }
}

impl Default for AimPosition {
    fn default() -> Self {
        AimPosition {
            vertical: 0,
            horizontal: 0,
            aim: 0,
        }
    }
}

fn get_input(use_example_input: bool) -> anyhow::Result<Vec<String>> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "02A.txt"
    } else {
        INPUT_PATH.to_owned() + "02.txt"
    };
    read_vec_string(filename)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let input: AoCResult<Vec<Instruction>> = get_input(use_example_input)?
        .iter()
        .map(|instruction| Instruction::try_from(instruction.to_string()))
        .collect();
    let input = input?; //.collect::<Vec<Result<Instruction,AoCError>>>()?;
    let mut position = AimPosition::default();
    for instruction in input {
        position.apply_instruction(instruction);
    }
    Ok(position.output())
}
