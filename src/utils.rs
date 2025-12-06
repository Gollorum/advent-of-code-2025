use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::num::ParseIntError;
use std::path::Path;

pub fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
        .map(|content| content.to_string())
        .map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to read '{}': {}", file_path, e),
            )
        })
}

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) struct ErrorMsg {
    pub(crate) wrapped: String
}

impl ErrorMsg {
    pub(crate) fn print(result: Result<(), ErrorMsg>) -> () {
        result.unwrap_or_else(|err| println!("Error: {}", err.wrapped))
    }
    pub(crate) fn result_to_string(result: Result<String, ErrorMsg>) -> String {
        result.unwrap_or_else(|err| err.wrapped)
    }
    pub(crate) fn new(string: &str) -> ErrorMsg {
        ErrorMsg { wrapped: string.to_string() }
    }
}

impl From<io::Error> for ErrorMsg {
    fn from(err: io::Error) -> Self {
        ErrorMsg { wrapped: format!("IO error: {}", err.to_string()) }
    }
}
impl From<ParseIntError> for ErrorMsg {
    fn from(err: ParseIntError) -> Self {
        ErrorMsg { wrapped: format!("ParseIntError: {}", err.to_string()) }
    }
}
impl From<String> for ErrorMsg {
    fn from(err: String) -> Self {
        ErrorMsg { wrapped: err }
    }
}
impl From<&str> for ErrorMsg {
    fn from(err: &str) -> Self {
        ErrorMsg { wrapped: err.to_string() }
    }
}