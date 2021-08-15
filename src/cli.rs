//! The code to handle all cli related parts
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bft", about = "A brainfuck interpreter")]
pub struct Opt {
    /// If we can increase the size of cells of the Virtual Machine, default false.
    #[structopt(short, long)]
    pub extensible: bool,

    /// Set the number of cells in the virtual machine, default 30000.
    #[structopt(short, long)]
    pub cells: Option<usize>,

    /// Input source code
    #[structopt(name = "PROGRAM", parse(from_os_str))]
    pub program: PathBuf,
}
