use std::result::Result as CoreResult;
use rst_common::with_errors::anyhow::{Result, Ok, bail};
use rst_common::with_errors::thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("something went wrong: `{0}`")]
    Msg(String) 
}

fn is_okay() -> Result<String> {
    Ok("hello world".to_string())
}

fn is_error() -> Result<()> {
    bail!("hello error");
    Ok(())
}

fn from_thiserror() -> CoreResult<(), MyError> {
    Err(MyError::Msg("testing".to_string()))
} 

fn main() {
    let okay = is_okay();
    println!("{}", okay.unwrap());

    let err = is_error();
    println!("{}", err.unwrap_err());

    let thiserr_err = from_thiserror();
    println!("{}", thiserr_err.unwrap_err())
}