#[allow(dead_code)]

mod adapters;
mod cli;

mod prelude {
    // add adapters
    pub use crate::adapters::api::client as api_client;
    pub use crate::adapters::slack::client as slack_client;
    pub use crate::adapters::spotify::client as spotify_client;
    pub use crate::adapters::slack::stringer as slack_stringer;
    pub use crate::adapters::spotify::helpers as spotify_hlprs;
    // add cli
    pub use crate::cli::commands::base::Commands;
}
use crate::prelude::*;


use ::clap::Parser;


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
        // This operates like a switch statement - this is the command level (e.g. slack or evernote)
        Some(Commands::Slack(args)) => {
            // This is the args level (for now) - e.g. slack status or slack set
            // args.string extracts the field value for the arg value "string"
            match args.string {
                Some(ref _args) => {
                    let reverse = slack_stringer::reverse(_args);
                    println!("{}", reverse);
                }
                None => {
                    println!("Please provide a string to reverse..");
                }
            }
        }
        Some(Commands::Spotify(args)) => {
            // This is the args level (for now) - e.g. slack status or slack set
            // args.string extracts the field value for the arg value "string"
            match args.query {
                Some(ref _args) => {
                    let reverse = spotify_client::query(&_args);
                    println!("{:#?}", reverse);
                }
                None => {
                    println!("Please provide a query to search for...");
                }
            }
        }
        None => {
            println!("No commands passed - please provide one of the available commands");
        }
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
