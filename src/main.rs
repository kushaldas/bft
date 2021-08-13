use bft_types::Program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_file("add.bf")?;
    dbg!(program.instructions());
    Ok(())
}
