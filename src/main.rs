#![allow(unused)]

mod adapters;
mod cli;

mod prelude {
    // add adapters
    pub use crate::adapters::api::client as api_client;
    pub use crate::adapters::slack::client as slack_client;
    pub use crate::adapters::slack::stringer as slack_stringer;
    // add cli
    pub use crate::cli::commands::slack::Commands;
}
use crate::prelude::*;


use ::clap::{Parser, Subcommand};
use crate::cli::commands::slack::Status;


#[derive(Parser)]
#[command(author, version)]
#[command(about = "A Basic CLI a.k.a. abcli - a simple CLI to do mundane things", long_about = "abcli is a super fancy CLI (kidding)
You can use abcli to do various things, but at the moment it does nothing...")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let parsed_cli = Cli::parse();

    match &parsed_cli.command {
        Some(Commands::Status(name)) => {
            match name.status {
                Some(ref _name) => {
                    let reverse = slack_stringer::reverse(_name);
                    println!("{}", reverse);
                }
                None => {
                    println!("Please provide a string to reverse");
                }
            }
        }
        None => {}
    }
}

// saving this for later reference
// let _d = API {};
// let _f = Slack {};

// use anyhow::{Context, Result};


// let content = std::fs::read_to_string(&args.path)
//     .with_context(|| format!("Could not read file `{}`", &args.path.display()))?;

// for line in content.lines() {
//     if line.contains(&args.pattern) {
//         println!("{}", line);
//     }
// }
// Ok(())