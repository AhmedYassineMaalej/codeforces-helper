use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::{Parser, Subcommand};
use clipboard_rs::{Clipboard, ClipboardContext};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Input,
    Output,
    Run { file: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            run_file(file);
        }
        Commands::Input => input(),
        Commands::Output => output(),
    }
}

fn run_file(file: PathBuf) {
    // compile passed file with output ./program
    let command = Command::new("g++")
        .arg(file.to_str().unwrap())
        .arg("-o")
        .arg("program")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // run ./program
    let mut command = Command::new("./program")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // read test input
    let input = fs::read_to_string("input.txt").unwrap();

    let mut stdin = command.stdin.as_mut().unwrap();
    stdin.write_all(input.as_bytes()).unwrap();

    let output_found = String::from_utf8(command.wait_with_output().unwrap().stdout).unwrap();
    let output_expected = fs::read_to_string("output.txt").unwrap();

    dbg!(&output_found);
    dbg!(&output_expected);

    if output_found == output_expected {
        println!("Tests can successfully!");
    } else {
        println!("Tests failed!");
    }
}

fn input() {
    // get content of copied input from clipboard
    let ctx = ClipboardContext::new().unwrap();
    let content = ctx.get_text().unwrap();
    dbg!(&content);

    fs::write("input.txt", content).unwrap();
}

fn output() {
    // get content of copied input from clipboard
    let ctx = ClipboardContext::new().unwrap();
    let mut content = ctx.get_text().unwrap();

    fs::write("output.txt", content).unwrap();
}
