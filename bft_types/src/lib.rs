//! Instructions for the brainfuck interpreter.

use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

type SourceInput = (usize, usize, char);

/// Instruction enum represents each instruction from our code.
///
/// This even stores any code comments as `char`. The language has
/// 8 single byte long characters as commands/instructions. Every value
/// also stores the line number, and the character number on the line.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Increment the data pointer (to point to the next cell to the right).
    IncrementDP(usize, usize),
    /// Decrement the data pointer (to point to the next cell to the left).
    DecrementDP(usize, usize),
    /// Increment (increase by one) the byte at the data pointer.
    IncrementByte(usize, usize),
    /// Decrement (decrease by one) the byte at the data pointer.
    DecrementByte(usize, usize),
    /// Output the byte at the data pointer.
    Output(usize, usize),
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    Input(usize, usize),
    /// If the byte at the data pointer is zero, then instead of moving the instruction
    /// pointer forward to the next command, jump it forward to the command after the matching `]` command.
    JumpForward(usize, usize),
    /// If the byte at the data pointer is nonzero, then instead of moving the instruction
    /// pointer forward to the next command, jump it back to the command after the matching `[` command.
    JumpBack(usize, usize),
    /// Any other character on the source code.
    Comment(usize, usize, char),
}

impl TryFrom<SourceInput> for Instruction {
    type Error = Box<dyn std::error::Error>;

    /// Converts a given `char` to the corresponding Brainfuck instruction.
    ///
    /// Other than the primary 8 chars, everything else is considered as comments
    /// including newline characters.
    fn try_from(source: SourceInput) -> Result<Self, Self::Error> {
        match source {
            (linenumber, charnumber, '>') => Ok(Instruction::IncrementDP(linenumber, charnumber)),
            (linenumber, charnumber, '<') => Ok(Instruction::DecrementDP(linenumber, charnumber)),
            (linenumber, charnumber, '+') => Ok(Instruction::IncrementByte(linenumber, charnumber)),
            (linenumber, charnumber, '-') => Ok(Instruction::DecrementByte(linenumber, charnumber)),
            (linenumber, charnumber, '.') => Ok(Instruction::Output(linenumber, charnumber)),
            (linenumber, charnumber, ',') => Ok(Instruction::Input(linenumber, charnumber)),
            (linenumber, charnumber, '[') => Ok(Instruction::JumpForward(linenumber, charnumber)),
            (linenumber, charnumber, ']') => Ok(Instruction::JumpBack(linenumber, charnumber)),
            (linenumber, charnumber, value) => {
                Ok(Instruction::Comment(linenumber, charnumber, value))
            }
        }
    }
}

/// Stores the full program instruction set in a Vector and also the filename of the
/// source code.
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
    ///
    /// # Example
    ///
    /// ```
    /// # use bft_types::Program;
    ///
    /// let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    /// let ins = Program::new("test.bf".to_string(), &code)
    /// ```
    pub fn new(filename: String, content: &str) -> Self {
        let ins = content
            .lines() // Get all the lines
            .enumerate() // We want go through each line
            .map(|(linenumber, line)| {
                line.chars() // Now for each character in the line
                    .enumerate()
                    .map(|(charnumber, ch)| {
                        let source: SourceInput = (linenumber + 1, charnumber + 1, ch); // Create a tuple with line number, character number, and the actual character.
                        Instruction::try_from(source).ok().unwrap()
                    })
                    .collect::<Vec<Instruction>>()
            })
            .flatten()
            .collect::<Vec<Instruction>>();

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

    /// Validates the instructions for bracket matching
    pub fn validate(&self) -> Result<(), std::io::Error> {
        let mut stack: Vec<(usize, usize)> = Vec::new();

        for instruction in self.ins.iter() {
            match instruction {
                Instruction::JumpForward(l, c) => {
                    stack.push((*l, *c));
                }
                Instruction::JumpBack(l, c) => {
                    // Means extra closing bracket
                    if stack.is_empty() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Extra close bracket at line {} character {}.", *l, *c),
                        ));
                    } else {
                        stack.pop();
                    }
                }
                _ => (),
            }
        }

        if !stack.is_empty() {
            // Means extra open brackets in our code
            let (l, c) = stack.pop().unwrap();
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Extra open bracket at line {} character {}.", l, c),
            ));
        }

        Ok(())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ins in self.instructions() {
            let _ = match ins {
                Instruction::IncrementDP(_, _) => write!(f, ">"),
                Instruction::DecrementDP(_, _) => write!(f, "<"),
                Instruction::IncrementByte(_, _) => write!(f, "+"),
                Instruction::DecrementByte(_, _) => write!(f, "-"),
                Instruction::Output(_, _) => write!(f, "."),
                Instruction::Input(_, _) => write!(f, ","),
                Instruction::JumpForward(_, _) => write!(f, "["),
                Instruction::JumpBack(_, _) => write!(f, "]"),
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
        assert_eq!(ins[0], Instruction::IncrementDP(1, 1));
        assert_eq!(ins[1], Instruction::DecrementDP(2, 1));
        assert_eq!(ins[2], Instruction::IncrementByte(2, 2));
        assert_eq!(ins[3], Instruction::DecrementByte(2, 3));
        assert_eq!(ins[4], Instruction::Output(2, 4));
        assert_eq!(ins[5], Instruction::Input(2, 5));
    }
}
