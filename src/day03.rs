use std::usize;

use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH, misc};

#[derive(Clone, Copy, Debug)]
struct BitCount {
    zeros: usize,
    ones: usize,
}

impl BitCount {
    pub fn get_most_common(&self) -> char {
        if self.zeros > self.ones {
            '0'
        } else {
            '1'
        }
    }
    pub fn get_least_common(&self) -> char {
        if self.zeros > self.ones {
            '1'
        } else {
            '0'
        }
    }
    pub fn add_bit(&mut self, bit: char) {
        match bit {
            '0' => self.zeros += 1,
            '1' => self.ones += 1,
            _ => {
                panic!("Unexpected char '{}' in bit string", bit);
            }
        }
    }
}

impl Default for BitCount {
    fn default() -> Self {
        BitCount { zeros: 0, ones: 0 }
    }
}

pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "03A.txt"
    } else {
        INPUT_PATH.to_owned() + "03.txt"
    };
    let input = misc::read_vec_string(filename)?;
    let number_length = input.get(0).unwrap().len();
    let mut bit_counts = Vec::with_capacity(number_length);
    for _ in 0..number_length {
        bit_counts.push(BitCount::default());
    }
    for bit_index in 0..number_length {
        let current_bit = bit_counts.get_mut(bit_index).unwrap();
        for current_number in &input {
            let bit = current_number.chars().nth(bit_index).unwrap();
            current_bit.add_bit(bit);
        }
    }
    let gamma_str = bit_counts.clone().iter().map(|b| b.get_most_common()).collect::<String>();
    let gamma: usize = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon_str = bit_counts.clone().iter().map(|b| b.get_least_common()).collect::<String>();
    let epsilon: usize = usize::from_str_radix(&epsilon_str, 2).unwrap();
    Ok(gamma * epsilon)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "03A.txt"
    } else {
        INPUT_PATH.to_owned() + "03.txt"
    };
    let input = misc::read_vec_string(filename)?;
    let mut oxygen = input.clone();
    let mut co2 = input.clone();


    let number_length = input.get(0).unwrap().len();
    let mut bit_counts = Vec::with_capacity(number_length);
    for _ in 0..number_length {
        bit_counts.push(BitCount::default());
    }
    for bit_index in 0..number_length {
        if oxygen.len() > 1 {
            oxygen = filter_by_bit(oxygen, bit_index, true);
        }
        if co2.len() > 1 {
            co2 = filter_by_bit(co2, bit_index, false);
        }
    }
    let oxygen_str = oxygen.first().unwrap();
    let co2_str = co2.first().unwrap();
    let oxygen: usize = usize::from_str_radix(&oxygen_str, 2).unwrap();
    let co2: usize = usize::from_str_radix(&co2_str, 2).unwrap();
    Ok(oxygen * co2)
}

fn filter_by_bit(mut data: Vec<String>, bit_index: usize, use_most_common: bool) -> Vec<String> {
    let mut bit_count = BitCount::default();

    for line in &data {
        let bit = line.chars().nth(bit_index).unwrap();
        bit_count.add_bit(bit);
    }

    data.retain(|line| line.chars().nth(bit_index).unwrap() == if use_most_common { bit_count.get_most_common() } else { bit_count.get_least_common() });
    data
}