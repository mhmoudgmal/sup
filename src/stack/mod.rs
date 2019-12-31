pub mod parser;

use std::error::Error;

use parser::Stack;

pub fn up(_stack: Stack) -> Result<(), Box<dyn Error>> {
    Ok(())
}
