//! The interpreter for the language

use bft_types::Program;

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
    pub fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.head == 0 {
            // Means already at the beginning
            return Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "Already at the beginning of the tape.",
            ));
        }
        self.head -= 1;
        Ok(())
    }

    /// Moves the head to right
    pub fn move_head_right(&mut self) -> Result<(), VMError> {
        if self.head == (self.cells.len() - 1) {
            // Means already at the end
            return Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                "Already at the end of the tape.",
            ));
        }
        self.head += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::VirtualMachine;

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
}
