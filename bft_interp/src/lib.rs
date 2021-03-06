//! The interpreter for the language

use bft_types::{Instruction, Program};
use std::io::{Read, Write};

type VMError = std::io::Error;

/// VirtualMachine holding the cells of the system
///
/// It has the following members:
///
/// - size for the current size of the machine
/// - growable boolean value to tell us if the machine size is growable or not
/// - cells is a vector holding the memory cells, all initialized with 0.
/// - ip is the current Instruction Pointer.
#[derive(Debug)]
pub struct VirtualMachine {
    /// size of the machine
    size: usize,
    /// Boolean value to tells us if the machine is growable in size or not.
    growable: bool,
    /// The vector holding the memory cells.
    cells: Vec<u8>,
    /// Instruction pointer of the machine.
    ip: usize, // Instruction pointer
    /// head of the tape
    head: usize,
    /// The program to interpret
    prg: Program,
}

impl VirtualMachine {
    /// Creates a new virtual machine.
    /// You can set a size of the initialized machine, if you pass `0` as value, then it gets
    /// 30000 cells by default.
    /// If you pass `growable` as true, then the size of the system can grow dynamically.
    /// You can also pass the program which needs to be running on the virtual machine
    pub fn new(size: usize, growable: bool, prog: Program) -> Self {
        let size = match size {
            0 => 30000,
            _ => size,
        };

        VirtualMachine {
            size,
            growable,
            cells: vec![0u8; size],
            ip: 0,
            head: 0,
            prg: prog,
        }
    }

    /// Tell us if the VirtualMachine can grow in size or not.
    pub fn can_grow(self) -> bool {
        self.growable
    }

    /// To borrow the cells
    pub fn get_cells(&self) -> &[u8] {
        &self.cells[..]
    }

    /// Executes the given program structure
    pub fn interpret<R, W>(&mut self, input: &mut R, output: &mut W) -> Result<(), VMError>
    //pub fn interpret(&mut self) -> Result<(), VMError>
    where
        R: Read,
        W: Write,
    {
        if self.ip != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Program already executed.",
            ));
        }
        let length = self.prg.instructions().len();

        loop {
            if self.ip >= length {
                break;
            }
            let ins = self.prg.instructions()[self.ip];
            //dbg!(ins);
            match ins {
                Instruction::IncrementDP(_, _) => {
                    self.head += 1;
                    self.ip += 1
                }
                Instruction::DecrementDP(_, _) => {
                    self.head -= 1;
                    self.ip += 1
                }
                Instruction::IncrementByte(_, _) => {
                    self.cells[self.head].wrapping_increment();
                    self.ip += 1
                }
                Instruction::DecrementByte(_, _) => {
                    self.cells[self.head].wrapping_decrement();
                    self.ip += 1
                }
                Instruction::Output(_, _) => {
                    let _ = self.output(output);
                }
                Instruction::Input(_, _) => {
                    let _ = self.input(input);
                }
                Instruction::JumpForward(_, _) => {
                    self.start_loop()?;
                }
                Instruction::JumpBack(_, _) => {
                    self.end_loop()?;
                }
                Instruction::Comment(_, _, _) => self.ip += 1,
            }
        }
        Ok(())
    }

    /// Moves the head to left
    pub fn move_head_left(&mut self) -> Result<usize, VMError> {
        if self.head == 0 {
            // Means already at the beginning
            return Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "Already at the beginning of the tape.",
            ));
        }
        self.head -= 1;

        // Increase IP
        self.ip += 1;
        Ok(self.ip)
    }

    /// Moves the head to right
    pub fn move_head_right(&mut self) -> Result<usize, VMError> {
        if self.head == (self.cells.len() - 1) {
            // Means already at the end
            return Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "Already at the end of the tape.",
            ));
        }
        self.head += 1;
        // Increase IP
        self.ip += 1;
        Ok(self.ip)
    }

    /// Reads into current head of the tape
    ///
    /// Needs a Reader reference to read from.
    pub fn input<R>(&mut self, r: &mut R) -> Result<usize, VMError>
    where
        R: Read,
    {
        // Reading only 1 byte at a time
        let mut buf = vec![0u8; 1];
        r.read_exact(&mut buf)?;
        // Now assign the value
        self.cells[self.head] = buf[0];
        // Increase IP
        self.ip += 1;
        Ok(self.ip)
    }

    /// Writes one byte from the head of the tape to the Write
    pub fn output<W>(&mut self, w: &mut W) -> Result<usize, VMError>
    where
        W: Write,
    {
        //dbg!(self.cells[self.head]);
        w.write_all(&[self.cells[self.head]])?;
        w.flush().unwrap();
        // Increase IP
        self.ip += 1;
        Ok(self.ip)
    }

    /// Call this when you see a jump forward
    pub fn start_loop(&mut self) -> Result<usize, VMError> {
        let mut stack: Vec<usize> = vec![self.ip];
        // Check if head is 0
        if self.cells[self.head] == 0 {
            let ins = self.prg.instructions();
            loop {
                self.ip += 1;
                // Now check for the matching closing ]
                match ins[self.ip] {
                    Instruction::JumpForward(_, _) => {
                        // Push the IP value
                        stack.push(self.ip);
                    }
                    Instruction::JumpBack(_, _) => {
                        // Now we have to pop and see if we are in the right place
                        stack.pop();
                        // If the stack is empty means we are at the right place.
                        if stack.is_empty() {
                            self.ip += 1;
                            // Get out of the loop
                            break;
                        }
                    }

                    _ => (),
                }
            }
        } else {
            self.ip += 1;
        }
        Ok(self.ip)
    }
    /// Call this when you see a jump back
    pub fn end_loop(&mut self) -> Result<usize, VMError> {
        let mut stack: Vec<usize> = vec![self.ip];
        // Check if head is 0
        if self.cells[self.head] != 0 {
            let ins = self.prg.instructions();
            loop {
                self.ip -= 1;
                // Now check for the matching closing ]
                match ins[self.ip] {
                    Instruction::JumpBack(_, _) => {
                        // Push the IP value
                        stack.push(self.ip);
                    }
                    Instruction::JumpForward(_, _) => {
                        // Now we have to pop and see if we are in the right place
                        stack.pop();
                        // If the stack is empty means we are at the right place.
                        if stack.is_empty() {
                            self.ip += 1;
                            // Get out of the loop
                            break;
                        }
                    }

                    _ => (),
                }
            }
        } else {
            self.ip += 1;
        }
        Ok(self.ip)
    }
}

/// Our trait to handle Cell data
pub trait CellKind {
    /// We can increase a cell value
    fn wrapping_increment(&mut self);
    /// We can decrease a cell value
    fn wrapping_decrement(&mut self);
}

/// Implementing for u8
impl CellKind for u8 {
    fn wrapping_increment(&mut self) {
        *self = self.wrapping_add(1);
    }
    fn wrapping_decrement(&mut self) {
        *self = self.wrapping_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use bft_types::Program;

    use crate::CellKind;
    use crate::VirtualMachine;
    use std::io::Cursor;

    fn get_small_program() -> Program {
        let content = String::from("++> +++++ [<+>-]");
        Program::new("test.bf".to_string(), &content)
    }

    #[test]
    fn t_jump_forward() {
        let p = get_small_program();

        let mut vm = VirtualMachine::new(3, false, p);
        // now test
        vm.ip = 10;
        let ip = vm.start_loop().unwrap();
        assert_eq!(ip, 16);
    }

    #[test]
    fn t_jump_back() {
        let p = get_small_program();
        let mut vm = VirtualMachine::new(3, false, p);
        vm.cells = vec![1, 2, 3];
        // now test
        vm.ip = 15;
        let ip = vm.end_loop().unwrap();
        assert_eq!(ip, 11);
    }

    #[test]
    fn take_input_do_output() {
        let p = get_small_program();
        let mut vm = VirtualMachine::new(3, false, p);

        let mut buff = Cursor::new(vec![42, 1, 2]);
        let res = vm.input(&mut buff);
        assert_eq!(res.ok(), Some(1));

        {
            let cells = vm.get_cells();
            assert_eq!(cells[0], 42);
        }

        // Now let us test output
        let mut out_buffer = Cursor::new(Vec::new());
        let res = vm.output(&mut out_buffer);
        assert_eq!(res.ok(), Some(2));
        let o = out_buffer.get_ref()[0];
        assert_eq!(o, 42);
    }

    #[test]
    fn check_valid_left_right_move() {
        let p = get_small_program();
        let mut vm = VirtualMachine::new(3, false, p);
        let _ = vm.move_head_right();
        let _ = vm.move_head_right();
        let _ = vm.move_head_left();
        let _ = vm.move_head_left();
    }

    #[test]
    fn check_invalid_right_move() {
        let p = get_small_program();
        let mut vm = VirtualMachine::new(3, false, p);
        let _ = vm.move_head_right();
        let _ = vm.move_head_right();
        let _ = vm.move_head_right();
        let res = vm.move_head_right().err().unwrap();
        let expected = std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Already at the end of the tape.",
        );
        assert_eq!(res.to_string(), expected.to_string());
    }
    #[test]
    fn check_invalid_left_move() {
        let p = get_small_program();
        let mut vm = VirtualMachine::new(3, false, p);
        let res = vm.move_head_left().err().unwrap();
        let expected = std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Already at the beginning of the tape.",
        );
        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn do_u8_increment_big() {
        let mut num = 255 as u8;
        num.wrapping_increment();
        assert_eq!(num, 0);
    }
    #[test]
    fn do_u8_increment_small() {
        let mut num = 253 as u8;
        num.wrapping_increment();
        assert_eq!(num, 254);
    }
    #[test]
    fn do_u8_derement_big() {
        let mut num = 0 as u8;
        num.wrapping_decrement();
        assert_eq!(num, 255);
    }
    #[test]
    fn do_u8_decrement_small() {
        let mut num = 253 as u8;
        num.wrapping_decrement();
        assert_eq!(num, 252);
    }
}
