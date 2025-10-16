use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};

use clipboard_rs::{Clipboard, ClipboardContext};

use crate::errors::CliError;

pub fn run(source: &Path, show: bool, interactive: bool, compare: bool) {
    if let Err(e) = compile_file(source) {
        println!("{e}");
        return;
    }

    let output = match execute_code(interactive) {
        Ok(output) => output,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    if show {
        println!("execution output: ");
        print!("{output}");
    }

    if compare && let Err(e) = compare_outputs(&output, source) {
        println!("{e}");
        return;
    }

    if let Err(e) = write_output(&output) {
        println!("{e}");
    }
}

/// compile passed file with output ./program
fn compile_file(file: &Path) -> Result<(), CliError> {
    let mut command = Command::new("g++");
    command
        .arg(file.as_os_str())
        .arg("-o")
        .arg("program")
        .arg("-O3");

    let process = command
        .spawn()
        .map_err(|e| CliError::Io(String::from("failed to execute g++"), e))?;

    let output = process
        .wait_with_output()
        .map_err(|e| CliError::Io(String::from("compilation did not exit successfully"), e))?;

    if !output.status.success() {
        return Err(CliError::Compilation);
    }

    Ok(())
}

fn execute_code(interactive: bool) -> Result<String, CliError> {
    let mut command = Command::new("./program");
    command.stdout(Stdio::piped());

    if interactive {
        command.stdin(Stdio::inherit());
    } else {
        command.stdin(Stdio::piped());
    }

    let mut child = command
        .spawn()
        .map_err(|e| CliError::Io(String::from("failed to run program as child process"), e))?;

    if !interactive {
        write_input_to_process(&mut child)?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| CliError::Io(String::from("failed to execute program"), e))?;

    if !output.status.success() {
        return Err(CliError::Execution(String::from(
            "program did not exit successfully",
        )));
    }

    let output =
        String::from_utf8(output.stdout).expect("program output is not a valid UTF-8 string");

    Ok(output)
}

fn write_input_to_process(process: &mut Child) -> Result<(), CliError> {
    let input = fs::read_to_string("input.txt")
        .map_err(|e| CliError::Io(String::from("failed to read data from input.txt"), e))?;

    let stdin = process
        .stdin
        .as_mut()
        .ok_or(CliError::Execution(String::from(
            "failed to acquire child process stdin",
        )))?;

    stdin
        .write_all(input.as_bytes())
        .map_err(|e| CliError::Io(String::from("failed to write to child process stdin"), e))?;

    Ok(())
}

fn compare_outputs(output: &str, source: &Path) -> Result<(), CliError> {
    let expected = fs::read_to_string("output.txt")
        .map_err(|e| CliError::Io(String::from("failed to read data from input.txt"), e))?;

    if output == expected {
        println!("test case passed!");
        copy_code_to_clipboard(source)?;

        return Ok(());
    }

    Ok(())
}

fn write_output(output: &str) -> Result<(), CliError> {
    fs::write("output.txt", output)
        .map_err(|e| CliError::Io(String::from("failed to output to output.txt"), e))
}

fn copy_code_to_clipboard(code: &Path) -> Result<(), CliError> {
    let code = fs::read_to_string(code)
        .map_err(|e| CliError::Io(String::from("failed to read data from output.txt"), e))?;

    let ctx = ClipboardContext::new().map_err(|_e|
        CliError::Clipboard(String::from("failed to access system clipboard!\nPlease manually create input.txt containing the desired input")
    ))?;

    ctx.set_text(code)
        .map_err(|_e| CliError::Clipboard(String::from("failed to copy code to clipboard")))?;

    // this line is required to make sure the write to the clipboard is done
    ctx.get_text()
        .map_err(|_e| CliError::Clipboard(String::from("failed to verify copied code")))?;

    println!("copied code to clipboard!");
    Ok(())
}
