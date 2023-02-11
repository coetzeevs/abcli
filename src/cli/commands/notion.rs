use clap::Args;
use clap::Subcommand;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Notion {
    #[command(subcommand)]
    pub command: Option<NotionCommands>,
}

#[derive(Debug, Subcommand)]
pub enum NotionCommands {
    Create(NotionSubcommands),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NotionSubcommands {
    #[command(subcommand)]
    pub command: Option<NotionSubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum NotionSubCommands {
    Page(NotionArgs),
}

#[derive(Debug, Args)]
/// Returns Slack status for the given user
pub struct NotionArgs {
    /// Title needed for the page to be created
    #[arg(short = 't', long = "title")]
    // for now this won't actually pass to the function to create the page
    pub page_title: String,

    /// Database ID where the page object should be created.
    #[arg(short = 'd', long = "database_id", env = "NOTION_DATABASE_ID")]
    pub database_id: String,
}
