use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clipboard_rs::{Clipboard, ClipboardContext};

pub fn run(file: PathBuf, _show: bool, _test: bool) {
    // compile passed file with output ./program
    let mut command = Command::new("g++");
    command
        .arg(file.as_os_str())
        .arg("-o")
        .arg("program")
        .arg("-O3");

    let Ok(process) = command.spawn() else {
        println!("error: failed to execute g++");
        return;
    };

    match process.wait_with_output() {
        Ok(output) if !output.status.success() => {
            println!("error: compilation did not exit successfully");
            let output = String::from_utf8(output.stderr).expect("failed to decode g++ output");
            println!("{output}");
        }
        Err(e) => {
            println!("error: failed to execute compilation command");
            println!("{e}");
            return;
        }
        // do nothing in case of success
        Ok(_output) => {}
    }

    // read test input
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(e) => {
            println!("error: failed to read data from input.txt");
            println!("{e}");
            return;
        }
    };

    // run ./program with test input
    let mut command = Command::new("./program");
    command.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(process) => process,
        Err(e) => {
            println!("error: failed to run program as child process");
            println!("{e}");
            return;
        }
    };

    let Some(stdin) = child.stdin.as_mut() else {
        println!("error: failed to acquire child process stdin");
        return;
    };

    if let Err(e) = stdin.write_all(input.as_bytes()) {
        println!("error: failed to write to child process stdin");
        println!("{e}");
        return;
    }

    let output = match child.wait_with_output() {
        Ok(output) if !output.status.success() => {
            println!("error: program did exit successfully");
            let output = String::from_utf8(output.stderr).expect("failed to decode program output");
            println!("{output}");
            return;
        }
        Err(e) => {
            println!("error: failed to execute program");
            println!("{e}");
            return;
        }
        Ok(output) => output,
    };

    let output =
        String::from_utf8(output.stdout).expect("program output is not a valid UTF-8 string");

    let expected_output = match fs::read_to_string("output.txt") {
        Ok(content) => content,
        Err(e) => {
            println!("error: failed to read data from output.txt");
            println!("{e}");
            return;
        }
    };

    if output == expected_output {
        println!("Tests ran successfully!");
        copy_code_to_clipboard(file);
    } else {
        println!("Tests failed!");
    }
}

fn copy_code_to_clipboard(code: PathBuf) {
    let code = match fs::read_to_string(code) {
        Ok(content) => content,
        Err(e) => {
            println!("error: failed to read data from output.txt");
            println!("{e}");
            return;
        }
    };

    let Ok(ctx) = ClipboardContext::new() else {
        println!(
            "error: failed to access system clipboard!\nPlease manually create input.txt containing the desired input"
        );
        return;
    };

    if let Err(e) = ctx.set_text(code) {
        println!("error: failed to copy code to clipboard");
        println!("{e}");
        return;
    }

    // this line is required to make sure the write to the clipboard is done
    if let Err(e) = ctx.get_text() {
        println!("error: failed to verify copied code");
        println!("{e}");
        return;
    }
    println!("copied code to clipboard!");
}
