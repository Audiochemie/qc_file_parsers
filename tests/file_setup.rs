//! Wrapper functions to setup required data for integration tests of qc_file_parsers
use std::{
    fs::File,
    io::{BufReader, Result},
};
pub fn setup_allene_symbolic() -> Result<BufReader<File>> {
    let test_file = File::open("tests/test_allene_symbolic.xyz")?;
    Ok(BufReader::new(test_file))
}
pub fn setup_acetaldehyde_numeric() -> Result<BufReader<File>> {
    let test_file = File::open("tests/test_acetaldehyde_numeric.xyz")?;
    Ok(BufReader::new(test_file))
}

pub fn setup_array_text() -> Result<BufReader<File>> {
    let test_file = File::open("tests/test_array_text.dat")?;
    Ok(BufReader::new(test_file))
}
