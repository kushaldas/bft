//! Instructions for the brainfuck interpreter.

use std::convert::TryFrom;
use std::fs;
use std::path::Path;

/// Instruction enum represents each instruction from our code.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Instruction {
    IncrementDP,
    DecrementDP,
    IncrementByte,
    DecrementByte,
    Output,
    Input,
    JumpForward,
    JumpBack,
    Comment(char),
}

impl TryFrom<char> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '>' => Ok(Instruction::IncrementDP),
            '<' => Ok(Instruction::DecrementDP),
            '+' => Ok(Instruction::IncrementByte),
            '-' => Ok(Instruction::DecrementByte),
            '.' => Ok(Instruction::Output),
            ',' => Ok(Instruction::Input),
            '[' => Ok(Instruction::JumpForward),
            ']' => Ok(Instruction::JumpBack),
            _ => Ok(Instruction::Comment(input)),
        }
    }
}

/// Stores the full program instruction set.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Program {
    filename: String,
    ins: Vec<Instruction>,
}

impl Program {
    /// Takes a program path, and returns a `Program` structure holding all the instructions.
    /// We also strip the actual comments, and mark that in the instruction set.
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let contents = fs::read_to_string(&path)?;
        let ins = contents.chars().map(Instruction::try_from).collect();

        let only_ins = match ins {
            Ok(value) => value,
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    err.to_string(),
                ));
            }
        };

        Ok(Program {
            filename: path.as_ref().to_str().unwrap().to_string(),
            ins: only_ins,
        })
    }
}
