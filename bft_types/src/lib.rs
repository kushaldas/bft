//! Instructions for the brainfuck interpreter.

use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

/// Instruction enum represents each instruction from our code.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
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
        let content = fs::read_to_string(&path)?;
        let filename = match path.as_ref().to_str() {
            Some(name) => name.to_string(),
            None => {
                return Err(io::Error::new(io::ErrorKind::Other, "Filename not unicode"));
            }
        };
        Ok(Program::new(filename, &content))
    }

    /// Creates a new instance of the Program structure
    pub fn new(filename: String, content: &str) -> Self {
        let ins = content
            .chars()
            .filter_map(|c| Instruction::try_from(c).ok())
            .collect();

        Program { filename, ins }
    }

    /// Returns the source code filename as String
    pub fn source_file(self) -> String {
        self.filename
    }

    /// Rerturns a slice to the internal instructions
    pub fn instructions(&self) -> &[Instruction] {
        &self.ins[..]
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ins in self.instructions() {
            let _ = match ins {
                Instruction::IncrementDP => write!(f, ">"),
                Instruction::DecrementDP => write!(f, "<"),
                Instruction::IncrementByte => write!(f, "+"),
                Instruction::DecrementByte => write!(f, "-"),
                Instruction::Output => write!(f, "."),
                Instruction::Input => write!(f, ","),
                Instruction::JumpForward => write!(f, "["),
                Instruction::JumpBack => write!(f, "]"),
                _ => Ok(()),
            };
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Program};

    #[test]
    fn parse_source() {
        let input = ">
<+-.,
[]"
        .to_string();

        // Now create the program
        let p = Program::new("test.bf".to_string(), &input);
        let ins = p.instructions();
        assert_eq!(ins[0], Instruction::IncrementDP);
        assert_eq!(ins[1], Instruction::Comment('\n'));
        assert_eq!(ins[2], Instruction::DecrementDP);
        assert_eq!(ins[3], Instruction::IncrementByte);
        assert_eq!(ins[4], Instruction::DecrementByte);
        assert_eq!(ins[5], Instruction::Output);
        assert_eq!(ins[6], Instruction::Input);
        assert_eq!(ins[7], Instruction::Comment('\n'));
    }
}
