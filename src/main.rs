mod adapters;
mod app;
mod cli;

mod prelude {
    // add adapters
    pub use crate::adapters::api::client as api_client;
    pub use crate::adapters::notion::client as notion_client;
    // add cli
    pub use crate::cli::commands::base::Commands;
    pub use crate::cli::commands::notion;
}
use crate::prelude::*;

// tracing and other juicy stuff
use color_eyre::Report;

// clap parser
use clap::Parser;

// add app setup
use crate::app::setup::setup;

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(
    about = "A Basic CLI a.k.a. abcli - a simple CLI to do mundane things",
    long_about = "abcli is a super fancy CLI (kidding)
You can use abcli to do various things, but at the moment it does nothing..."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let args = Cli::parse();
    match args.command {
        Some(Commands::Notion(notion)) => {
            let notion_cmd = notion.command.unwrap();
            match notion_cmd {
                notion::NotionCommands::Create(page) => {
                    let create_cmd = page.command.unwrap();
                    match create_cmd {
                        notion::NotionSubCommands::Page(create_cmd) => {
                            notion_client::create(create_cmd).await;
                        }
                    }
                }
            }
        }
        None => {
            println!("No commands passed - please provide one of the available commands");
        }
    }
    Ok(())
}

// some sample commands to work towards
// abcli notion create page --title "Title" --database "database_id"
// abcli notion create database --title "Title" --page "page_id" --type "list"
