use clap::Parser;
use std::io::BufRead;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).expect("could not read file");
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line).expect("could not read line") > 0 {
        if line.contains(&args.pattern){
            println!("{}", line);
        }
        line.clear();
    }
    Ok(())
}
