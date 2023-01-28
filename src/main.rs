#[allow(dead_code)]
mod adapters;
mod cli;
mod utils;

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

// tracing and other juicy stuff
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;

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

fn main() -> Result<(), Report> {
    setup()?;
    info!("Setup done...");

    info!("Parsing CLI...");
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
                    // TODO: need to change the spotify query function
                        // the query function needs to receive the query string, use the set headers
                        // call the spotify client (which uses the base client abstraction)
                        // recieve and parse the response
                        // forward the results here as an object of type APIResponse
                        // For help on futures: https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep
                        // For help on error handling: https://www.shuttle.rs/blog/2022/06/30/error-handling
                    let _res = spotify_client::query(&_args);
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

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
