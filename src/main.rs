extern crate core;
use arboard::Clipboard;
use crate::utils::ErrorMsg;

mod utils;
mod day01;
mod day02;

fn main() {
    let response = ErrorMsg::result_to_string(day02::run_part_2(true));
    println!("{}", &response);
    copy_to_clipboard(&response).unwrap_or(());
}

fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;
    Ok(())
}
