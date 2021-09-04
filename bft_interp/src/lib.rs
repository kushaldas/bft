//! The interpreter for the language

use bft_types::Program;
use std::io::{Read, Seek, Write};

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
}

impl VirtualMachine {
    /// Creates a new virtual machine.
    /// You can set a size of the initialized machine, if you pass `0` as value, then it gets
    /// 30000 cells by default.
    /// If you pass `growable` as true, then the size of the system can grow dynamically.
    ///
    pub fn new(size: usize, growable: bool) -> Self {
        let size = match size {
            0 => 30000,
            _ => size,
        };

        VirtualMachine {
            size,
            growable,
            cells: vec![0u8, size as u8],
            ip: 0,
            head: 0,
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
    pub fn interpret(self, prog: &Program) {
        println!("{}", prog);
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
        W: Write + Seek,
    {
        w.write(&[self.cells[self.head]])?;
        // Increase IP
        self.ip += 1;
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
    use crate::CellKind;
    use crate::VirtualMachine;
    use std::io::Cursor;

    #[test]
    fn take_input_do_output() {
        let mut vm = VirtualMachine::new(3, false);

        let values: Vec<u8> = vec![42, 2, 1];
        let mut buff = Cursor::new(values);
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
        let mut vm = VirtualMachine::new(3, false);
        let _ = vm.move_head_right();
        let _ = vm.move_head_right();
        let _ = vm.move_head_left();
        let _ = vm.move_head_left();
    }

    #[test]
    fn check_invalid_right_move() {
        let mut vm = VirtualMachine::new(3, false);
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
        let mut vm = VirtualMachine::new(3, false);
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
