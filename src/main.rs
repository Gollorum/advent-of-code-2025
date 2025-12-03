use arboard::Clipboard;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        println!("Example: {} input.txt", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read the file
    match read_file(file_path) {
        Ok(content) => {
            // Print to console
            println!("File content:");
            println!("─────────────");
            println!("{}", content);
            println!("─────────────");

            // Try to copy to clipboard
            match copy_to_clipboard(&content) {
                Ok(_) => println!("\n✓ Content copied to clipboard!"),
                Err(e) => println!("\n⚠ Could not copy to clipboard: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        ));
    }

    fs::read_to_string(path)
}

fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;
    Ok(())
}
