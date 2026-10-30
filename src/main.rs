#![windows_subsystem = "windows"]

use std::env;
use std::os::windows::process::CommandExt;
use std::process::{Command, ExitCode};

use anyhow::{Context, Error, anyhow};
use tracelogging::write_event;

tracelogging::define_provider!(TRACELOGGING_PROVIDER, "JesseStricker.RunHidden");

fn main() -> ExitCode {
    unsafe { TRACELOGGING_PROVIDER.register() };

    let exit_code = run().unwrap_or_else(|err| {
        let message = format!("{:?}", err);
        write_event!(
            TRACELOGGING_PROVIDER,
            "Error",
            level(Error),
            str8("Message", &message)
        );

        ExitCode::from(1)
    });

    TRACELOGGING_PROVIDER.unregister();
    exit_code
}

fn run() -> Result<ExitCode, Error> {
    // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut passed_command_line = env::args_os().skip(1);
    let program = passed_command_line
        .next()
        .ok_or_else(|| anyhow!("Command line arguments must provide program to run."))?;

    let status = Command::new(program)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .with_context(|| "Failed to start or wait on child process.")?;

    let exit_code = status.code().unwrap_or(1) as u8;
    Ok(exit_code.into())
}
