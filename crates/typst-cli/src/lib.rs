mod args;
mod compile;
mod download;
mod fonts;
mod package;
mod query;
#[cfg(feature = "self-update")]
mod tracing;
mod watch;
mod world;

use std::cell::Cell;
use std::io::{self, IsTerminal, Write};

use clap::Parser;
use codespan_reporting::term::{self, termcolor};
use once_cell::sync::Lazy;
use termcolor::{ColorChoice, WriteColor};
use crate::args::{CliArguments, Command};


/// The parsed commandline arguments.
static ARGS: Lazy<CliArguments> = Lazy::new(CliArguments::parse);

#[no_mangle]
pub extern fn run(command: *const u8) -> i32 {
    // let arguments = CliArguments::parse();
    let command = unsafe { std::ffi::CStr::from_ptr(command as *const i8) };
    let command = command.to_str().unwrap();
    let arguments = CliArguments::parse_from(command.split(" ").collect::<Vec<&str>>());

    let res = match arguments.command {
        Command::Compile(command) => crate::compile::compile(command.clone()),
        Command::Watch(command) => crate::watch::watch(command.clone()),
        Command::Query(command) => crate::query::query(&command),
        Command::Fonts(command) => crate::fonts::fonts(&command),
        Command::Update(command) => crate::update::update(&command),
    };

    if let Err(msg) = res {
        print_error(&msg).expect("failed to print error");
    }

    // EXIT.with(|cell| cell.get())
    0
}

/// Ensure a failure exit code.
fn set_failed() {
    // EXIT.with(|cell| cell.set(ExitCode::FAILURE));
}

/// Print an application-level error (independent from a source file).
fn print_error(msg: &str) -> io::Result<()> {
    let mut w = color_stream();
    let styles = term::Styles::default();

    w.set_color(&styles.header_error)?;
    write!(w, "error")?;

    w.reset()?;
    writeln!(w, ": {msg}.")
}

/// Get stderr with color support if desirable.
fn color_stream() -> termcolor::StandardStream {
    termcolor::StandardStream::stderr(if std::io::stderr().is_terminal() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    })
}

/// Used by `args.rs`.
fn typst_version() -> &'static str {
    env!("TYPST_VERSION")
}

#[cfg(not(feature = "self-update"))]
mod update {
    use crate::args::UpdateCommand;
    use typst::diag::{bail, StrResult};

    pub fn update(_: &UpdateCommand) -> StrResult<()> {
        bail!(
            "self-updating is not enabled for this executable, \
             please update with the package manager or mechanism \
             used for initial installation"
        )
    }
}