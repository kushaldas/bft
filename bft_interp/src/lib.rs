//! The interpreter for the language

/// VirtualMachine holding the cells of the system
#[derive(Debug)]
pub struct VirtualMachine {
    size: usize,
    growable: bool,
    cells: Vec<u8>,
    ip: usize, // Instruction pointer
}

impl VirtualMachine {
    /// Creates a new virtual machine.
    /// If you pass `growable` as true, then the size of the system can grow dynamically.
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
}
