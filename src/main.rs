use bft_interp::VirtualMachine;
use bft_types::Program;
use std::env::args;
use std::error::Error;

/// Generic Error for the code readability.
type GError = Box<dyn Error>;

/// Entry point to the bft command.
///
/// Requires a source code as the argument.
fn main() -> Result<(), GError> {
    let filename = args().nth(1).ok_or("Requies a filename.bf as input.")?;
    let program = Program::from_file(filename)?;

    let vm = VirtualMachine::new(0, false);
    vm.interpret(&program);
    Ok(())
}
