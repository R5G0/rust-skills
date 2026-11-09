fn main() -> Result<(), std::num::ParseIntError> {
    let port = layout_flat_tool::parse_port("8080")?;
    println!("port={port}");
    Ok(())
}
