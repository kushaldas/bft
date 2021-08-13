use bft_interp::VirtualMachine;
use bft_types::Program;
use std::env::args;
use std::error::Error;

type GError = Box<dyn Error>;

fn main() -> Result<(), GError> {
    let filename = args().nth(1).ok_or("Requies a filename.bf as input.")?;
    let program = Program::from_file(filename)?;

    let vm = VirtualMachine::new(0, false);
    vm.interpret(&program);
    Ok(())
}
