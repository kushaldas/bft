//! The interpreter for the language

use bft_types::Program;

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
}
