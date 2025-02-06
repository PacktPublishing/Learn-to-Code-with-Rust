use std::fs;
use std::io;
use std::process;

fn main() {
    match write_to_file() {
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => {
            eprintln!("There was an error: {error}");
            process::exit(1);
        }
    }
}

fn write_to_file() -> io::Result<String> {
    let input = io::stdin();

    println!("What file would you like to write to?");
    let mut requested_file = String::new();
    input.read_line(&mut requested_file)?;

    println!("What would you like to write to the file?");
    let mut content = String::new();
    input.read_line(&mut content)?;

    fs::write(requested_file.trim(), content.trim())?;

    Ok(requested_file)
}
