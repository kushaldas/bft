use bft_interp::VirtualMachine;
use bft_types::Program;
use std::error::Error;
use structopt::StructOpt;

mod cli;
/// Generic Error for the code readability.
type GError = Box<dyn Error>;

/// Entry point to the bft command.
///
/// Requires a source code as the argument.
fn main() -> Result<(), GError> {
    let options = cli::Opt::from_args();
    let filename = options.program;
    let program = Program::from_file(filename)?;

    let size = match options.cells {
        Some(value) => value,
        None => 0,
    };

    let vm = VirtualMachine::new(size, options.extensible);
    vm.interpret(&program);
    Ok(())
}
