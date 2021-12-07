use log::debug;

use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::day02::{Instruction, InstructionDirection};
use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_string;

struct Position {
    vertical: usize,
    horizontal: usize,
}

impl Position {
    pub fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction.direction {
            InstructionDirection::Forward => self.horizontal += instruction.amount,
            InstructionDirection::Down => self.vertical += instruction.amount,
            InstructionDirection::Up => self.vertical -= instruction.amount,
        }
    }
    pub fn output(&self) -> usize {
        self.horizontal * self.vertical
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            vertical: 0,
            horizontal: 0,
        }
    }
}

impl TryFrom<String> for Instruction {
    type Error = AoCError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split = value.split(' ').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(AoCError::new(format!(
                "Submarine Instruction has too many components {}, Instruction '{}'",
                split.len(),
                value
            )));
        }
        debug!("Parsing '{:?}' to Instruction ", split);
        let direction = serde_plain::from_str::<InstructionDirection>(AoCError::from_option(
            split.get(0),
            Some(format!(
                "Failed to retrieve instruction direction from {:?}",
                split
            )),
        )?)?;
        let amount: usize = AoCError::from_option(
            split.get(1),
            Some(format!(
                "Failed to retrieve instruction amount from {:?}",
                split
            )),
        )?
            .parse()?;
        Ok(Instruction { direction, amount })
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

pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let input: AoCResult<Vec<Instruction>> = get_input(use_example_input)?
        .iter()
        .map(|instruction| Instruction::try_from(instruction.to_string()))
        .collect();
    let input = input?; //.collect::<Vec<Result<Instruction,AoCError>>>()?;
    let mut position = Position::default();
    for instruction in input {
        position.apply_instruction(instruction);
    }
    Ok(position.output())
}
