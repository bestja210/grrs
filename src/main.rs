// The following application creates the rust equivalent of the command line command grep named grrs
// The `clap` crate is used for parsing command line arguments.
use anyhow::{Context, Result};
use clap::Parser;

// Search for a pattern in a file and display lines that contain it.
#[derive(Parser)]
struct Cli {
    // Struct has two fields which are required when calling grrs: the pattern and the path.
    pattern: String, // pattern is of type String and is what the computer is to look for.
    path: std::path::PathBuf, // path is of type path::PathBuf and is the path to the file to be read.
}

fn main() -> Result<()> {
    let args = Cli::parse(); // Parse the arguments into the Cli struct.
    let content =
        std::fs::read_to_string(&args.path) // Opens provided file and returns Could not read file and exit if file cannot be read.
            .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
