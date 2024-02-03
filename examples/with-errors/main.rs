use rst_common::with_errors::anyhow::{Result, Ok, bail};

fn is_okay() -> Result<String> {
    Ok("hello world".to_string())
}

fn is_error() -> Result<()> {
    bail!("hello error");
    Ok(())
}

fn main() {
    let okay = is_okay();
    println!("{}", okay.unwrap());

    let err = is_error();
    println!("{}", err.unwrap_err())
}