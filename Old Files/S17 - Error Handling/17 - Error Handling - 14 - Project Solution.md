```rust
/*
Define a 'write_to_file' function. The function
should ask the user the following questions:

What file would you like to write to?
What would you like to write to the file?

Collect the user's two entries as Strings. If something
fails in either collection, propagate the error upwards;
the main function (the caller) will handle the error later.

Then, use the file system module's write function
to write the user's specified contents to their requested
text file. The documentation can be found here:
https://doc.rust-lang.org/std/fs/fn.write.html

Your write_to_file function should return an io::Result.
Look into the function signature of fs::write to detetermine
the right return value for write_to_file.

In the main function, use a match statement to react to
both variants of the io::Result enum. If everything worked,
output the text "Successfully wrote to file". If there was
any failure, output "There was an error writing to the file"
to the standard error output and interpolate the error.
Then, exit the program with a status code of 1.
 */

use std::fs;
use std::io::{self, stdin};
use std::process;

fn main() {
    match write_to_file() {
        Ok(_) => println!("Successfully wrote to file"),
        Err(error) => {
            eprintln!("There was an error writing to the file: {error}");
            process::exit(1);
        }
    }
}

fn write_to_file() -> io::Result<()> {
    println!("What file would you like to write to?");
    let mut requested_file = String::new();
    stdin().read_line(&mut requested_file)?;

    println!("What would you like to write to the file?");
    let mut content = String::new();
    stdin().read_line(&mut content)?;

    fs::write(requested_file.trim(), content.trim())
}

```
