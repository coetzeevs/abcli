mod adapters;
mod app;
mod cli;

mod prelude {
    // add adapters
    pub use crate::adapters::api::client as api_client;
    pub use crate::adapters::notion::client as notion_client;
    // add cli
    pub use crate::cli::commands::base::Commands;
}
use crate::prelude::*;

// add app
use crate::app::setup::setup;

// tracing and other juicy stuff
use color_eyre::Report;

// clap
use ::clap::Parser;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "A Basic CLI a.k.a. abcli - a simple CLI to do mundane things", long_about = "abcli is a super fancy CLI (kidding)
You can use abcli to do various things, but at the moment it does nothing...")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let parsed_cli = Cli::parse();

    match &parsed_cli.command {
        // This operates like a switch statement - this is the command level (e.g. slack or evernote)
        Some(Commands::Notion(args)) => {
            // This is the args level (for now) - e.g. slack status or slack set
            // args.string extracts the field value for the arg value "string"
            match args.page_title {
                Some(ref args) => {
                    notion_client::create(args).await;
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
    Ok(())
}
