use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::{File, read};
use std::io::{BufRead, BufReader, Lines};

use anyhow::Context;
use itertools::Itertools;
use log::debug;

use crate::{EXAMPLE_INPUT_PATH, INPUT_PATH, misc};

#[derive(Debug)]
pub struct Board {
    /// Value -> (Row, Col)
    number_index: HashMap<u32, (usize, usize)>,
    board: [[(u32, bool); 5]; 5],
}

impl Board {
    pub fn new(data: Vec<String>) -> Board {
        let mut lookup = HashMap::with_capacity(25);
        let mut board = [[(0, false); 5]; 5];
        for (row_index, row) in data.iter().enumerate() {
            for (col_index, entry) in (&row.chars().chunks(3)).into_iter().enumerate() {
                let value = entry.collect::<String>();
                let value = value.trim().parse().unwrap();
                lookup.insert(value, (row_index, col_index));
                board[row_index][col_index] = (value, false);
            }
        }
        Board { number_index: lookup, board }
    }
    pub fn add_number(&mut self, number: u32) {
        if let Some(pos) = self.number_index.get(&number) {
            self.board[pos.0][pos.1].1 = true;
        }
    }
    pub fn has_won(&self) -> bool {
        for row in &self.board {
            let mut won = true;
            for col in row {
                if !col.1 {
                    won = false;
                    break;
                }
            }
            if won {
                return true;
            }
        }
        for col in 0..5 {
            let mut won = true;
            for row in 0..5 {
                if !&self.board[row][col].1 {
                    won = false;
                    break;
                }
            }
            if won {
                return true;
            }
        }
        false
    }
    pub fn get_score(&self) -> u32 {
        let mut sum = 0;
        for row in &self.board {
            let mut won = true;
            for col in row {
                if !col.1 {
                    sum += col.0
                }
            }
        }
        sum
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (value, pos) in &self.number_index {
            writeln!(f, "{} is at position ({}, {})", value, pos.0, pos.1)?;
        }
        writeln!(f)?;
        for row in &self.board {
            for col in row {
                write!(f, "({}, {})  ", col.0, col.1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn read_board(filename: String) -> anyhow::Result<(Vec<u32>, Vec<Board>)> {
    debug!("Opening file {}", filename);
    let file = File::open(filename.to_string()).context(format!("Failed to file {}", filename))?;
    let reader = BufReader::new(file);
    let mut lines: Lines<BufReader<File>> = reader.lines();
    let number_order = lines.next().unwrap().unwrap();
    let number_order: Vec<u32> = number_order.split(',').map(|val| val.parse::<u32>().unwrap()).collect();
    // Skip Blank Line
    let mut boards = Vec::new();
    lines.next();
    loop {
        let mut current_board = Vec::with_capacity(5);
        for _index in 0..5 {
            let line = lines.next().unwrap().unwrap();
            current_board.push(line);
        }
        boards.push(Board::new(current_board));
        let line = lines.next();
        if line.is_none() {
            break;
        }
    }
    Ok((number_order, boards))
}


pub fn part_1(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "04A.txt"
    } else {
        INPUT_PATH.to_owned() + "04.txt"
    };
    let (numbers, mut boards) = read_board(filename)?;
    for number in numbers {
        print!("{}, ", number);
        for board in &mut boards {
            board.add_number(number);
            if board.has_won() {
                println!("Board has won!");
                //println!("{}",board);
                let sum = board.get_score();
                println!("Sum {}, Number: {}", sum, number);
                println!("{}", sum * number);
                return Ok((sum * number) as usize);
            }
        }
    }
    Ok(1)
}

pub fn part_2(use_example_input: bool) -> anyhow::Result<usize> {
    let filename = if use_example_input {
        EXAMPLE_INPUT_PATH.to_owned() + "04A.txt"
    } else {
        INPUT_PATH.to_owned() + "04.txt"
    };
    let (numbers, mut boards) = read_board(filename)?;
    for number in numbers {
        print!("{}, ", number);
        for board in &mut boards {
            board.add_number(number);
        }
        if boards.len() == 1 {
            let board = boards.get(0).unwrap();
            if board.has_won() {
                println!("Board has won!");
                println!("{}", board);
                let sum = board.get_score();
                println!("Sum {}, Number: {}", sum, number);
                println!("{}", sum * number);
                return Ok((sum * number) as usize);
            }
        } else {
            boards.retain(|board| !board.has_won());
        }
    }

    Ok(1)
}
