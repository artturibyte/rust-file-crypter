use clap::Parser;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn alternate_file(file_path: &std::path::PathBuf) {
    let content = std::fs::read_to_string(file_path).expect("could not read file");
    std::fs::write(file_path, content + "test").expect("Could not write to file!");
}

fn main() {
    let args = Cli::parse();
    alternate_file(&args.path);
}
