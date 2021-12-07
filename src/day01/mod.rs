use log::debug;

use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH};
use crate::misc::error::{AoCError, AoCResult};
use crate::misc::read_vec_ints;

fn get_input(use_example_input: bool) -> AoCResult<Vec<usize>> {
    let filename = if use_example_input { EXAMPLE_INPUT_PATH.to_owned() + "01A.txt" } else { INPUT_PATH.to_owned() + "01.txt" };
    read_vec_ints(filename)
}

pub fn part_1(use_example_input: bool) -> AoCResult<usize> {
    let input = get_input(use_example_input)?;
    let mut depth_increased_count = 0;
    let mut current_depth = *AoCError::from_option(input.get(0))?;
    for depth in input {
        if depth > current_depth {
            debug!("Depth increased to {} from {}",depth,current_depth);
            depth_increased_count += 1;
        }
        current_depth = depth;
    }
    Ok(depth_increased_count)
}

pub fn part_2(use_example_input: bool) -> AoCResult<usize> {
    let input = get_input(use_example_input)?;
    let input = input.as_slice();
    let mut current_window: usize = input[0..3].iter().sum();
    let mut depth_increased_count = 0;
    let mut index = 1;
    while index + 2 < input.len() {
        let new_window = input[index..index + 3].iter().sum();
        debug!("Index {}, Values: {:?}",index,input[index..index + 2].to_vec());
        if new_window > current_window {
            debug!("Index {} Depth increased to {} from {}",index,new_window,current_window);
            depth_increased_count += 1;
        }
        current_window = new_window;
        index += 1;
    }
    Ok(depth_increased_count)
}