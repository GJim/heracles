mod command;
mod config;
mod error;

use crate::error::CommandError;

mod shadow {
    use shadow_rs::shadow;
    shadow!(build);

    pub use self::build::*;
}

use self::command::Cli;

fn main() {
    if let Err(err) = Cli::default().run() {
        eprintln!("Error: {err}");
        std::process::exit(err.exit_code());
    }
}
