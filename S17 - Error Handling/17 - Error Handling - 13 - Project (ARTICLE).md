Follow the instructions below to complete the coding challenge.

The instructions are written as a Rust comment, so you can copy and paste them into a Rust (`.rs`) file if you want to.

Afterwards, see the following lesson for a solution. The solution may be explained in a video, an article, or both.

```rust
/*
Define a `write_to_file` function. The function
should ask the user the following questions:

What file would you like to write to?
What would you like to write to the file?

Collect the user's two entries as Strings. If something
fails in either collection, propagate the error upwards;
the main function (the caller) will handle the error
later.

Then, use the file system module's `write` function
to write the user's specified contents to their
requested text file. The documentation for `write`
can be  found here:
https://doc.rust-lang.org/std/fs/fn.write.html

If the `write` function fails, propagate the error
upwards as well.

Your `write_to_file` function should return an
`io::Result`.

After you write to the file, return a `Ok` variant
storing the user's requested file name.

In the main function, use a match statement to react
to both variants of the returned Result enum. If
everything worked, output the text "Successfully
wrote to file { }" and interpolate the file name you
collected in the `write_to_file` function.

If there was any failure, output "There was an error:
{error}" to the standard error output and
interpolate the error. Then, exit the program with a
status code of 1.
 */

```
