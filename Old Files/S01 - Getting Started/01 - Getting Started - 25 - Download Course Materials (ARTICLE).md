The course materials can be found at
https://github.com/paskhaver/learn-to-code-with-rust

To download the content, you can either clone the repository with Git or download the repository manually. A "repository" (repo) is just a technical term for a project managed by Git.

### To clone repository with Git:

In Terminal/PowerShell, navigate to the directory where you'd like to download the `learn-to-code-with-rust` course materials.

Execute `git clone git@github.com:paskhaver/learn-to-code-with-rust.git` to clone the repository to your computer.

### To download repository:

On the GitHub page, click the green `Code` button, then select "Download ZIP".

Unpack the ZIP and move the `learn-to-code-with-rust` directory to wherever you'd like.

To unpack the ZIP on macOS: Double-click the ZIP file.

To unpack the ZIP on Windows: Right-click the ZIP file, click Extract All..., and choose a directory to extract to.

![[GitHub Code Download.png]]

## Working Through the Course

The `learn-to-code-with-rust` folder contains multiple directories. Each folder is an independent Cargo project with a `Cargo.toml` file.

At the start of each course section, you'll find a note listing the Cargo project for that course.

For example, the next section is called "Variables and Mutability" and it has a corresponding `variables-and-mutability` folder.

At the start of each course section, open up the section's folder in VSCode. There are two ways to accomplish this:

- In VSCode, access the main menu. Select `File > Open Folder...`. Locate the section's project folder on your file system and select it.
- In Terminal/PowerShell, navigate into the section's folder with the `cd` command. Then, open the folder in VSCode with `code .`
