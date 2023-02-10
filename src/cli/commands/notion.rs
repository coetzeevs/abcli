use clap::Args;
use clap::Subcommand;


// TODO: Look into sub sub commands and how to do this
#[derive(Subcommand)]
/// Returns Slack status for the given user
pub enum NotionCommands {
    /// Subcommand for Notion to create objects
    Create(NotionArgs)
}

#[derive(Args)]
/// Returns Slack status for the given user
pub struct NotionArgs {
    /// Title needed for the page to be created
    #[arg(short = 't', long = "title")]
    // for now this won't actually pass to the function to create the page
    pub page_title: Option<String>,
}
