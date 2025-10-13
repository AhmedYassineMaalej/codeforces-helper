#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use clipboard_rs::{Clipboard, ClipboardContext};

use crate::run::run;

mod run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run code with test input
    Run {
        #[arg(help = "path to the C++ file to run")]
        source: PathBuf,
        #[arg(short, long, help = "show execution output in terminal")]
        show: bool,
        #[arg(short, long, help = "compare execution output with expected output")]
        compare: bool,
    },
    /// Set the test's input from the clipboard
    Input {
        #[arg(short, long)]
        clear: bool,
    },
    /// Set the test's expected output from the clipboard
    Output {
        #[arg(short, long)]
        clear: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            source: file,
            show,
            compare,
        } => run(file, show, compare),
        Commands::Input { .. } => input(),
        Commands::Output { .. } => output(),
    }
}

fn input() {
    // get content of copied input from clipboard
    let Ok(ctx) = ClipboardContext::new() else {
        println!(
            "error: failed to access system clipboard!\nPlease manually create input.txt containing the desired input"
        );
        return;
    };

    let Ok(content) = ctx.get_text() else {
        println!("error: failed to read from system clipboard!");
        println!("Make sure the clipboard contains text data");
        return;
    };

    if content.is_empty() {
        println!("warning: found empty clipboard!");
        println!("make sure to copy the input before running this command");
        return;
    }

    if let Err(_e) = fs::write("input.txt", content) {
        println!("error: failed to write clipboard's content to input.txt");
    }
}

fn output() {
    // get content of copied input from clipboard
    let Ok(ctx) = ClipboardContext::new() else {
        println!(
            "error: failed to access system clipboard!\nPlease manually create input.txt containing the desired input"
        );
        return;
    };

    let Ok(content) = ctx.get_text() else {
        println!("error: failed to read from system clipboard!");
        println!("Make sure the clipboard contains text data");
        return;
    };

    if content.is_empty() {
        println!("warning: found empty clipboard!");
        println!("make sure to copy the input before running this command");
        return;
    }

    if let Err(_e) = fs::write("output.txt", content) {
        println!("error: failed to write clipboard's content to input.txt");
    }
}
