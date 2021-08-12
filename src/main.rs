use bft_types::{Instruction, Program};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_file("add.bf")?;
    dbg!(program);
    Ok(())
}
