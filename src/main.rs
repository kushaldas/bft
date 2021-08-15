use bft_interp::VirtualMachine;
use bft_types::Program;
use std::env;
use std::error::Error;
use std::process;
use structopt::StructOpt;

mod cli;
/// Generic Error for the code readability.
type GError = Box<dyn Error>;

/// Entry point to the bft command.
///
/// Requires a source code as the argument.
fn main() {
    let options = cli::Opt::from_args();
    let res = run_bft(options);
    if res.is_err() {
        if let Ok(path) = env::current_exe() {
            let exe = path.file_name().unwrap();
            let msg = format!("{:?}: {}", exe, res.err().unwrap().to_string());
            eprintln!("{}", msg);
        }
        process::exit(1);
    } else {
        // All good
        process::exit(0);
    }
}

/// Entry point to the bft code base
fn run_bft(options: cli::Opt) -> Result<(), GError> {
    let filename = options.program;
    let program = Program::from_file(filename)?;

    program.validate()?;

    let size = options.cells.unwrap_or(0);

    let vm = VirtualMachine::new(size, options.extensible);
    vm.interpret(&program);
    Ok(())
}
