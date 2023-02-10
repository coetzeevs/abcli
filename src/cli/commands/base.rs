use clap::Subcommand;

use super::notion::NotionArgs;


// first tier subcommands - e.g. acbli slack or abcli evernote
#[derive(Subcommand)]
pub enum Commands {
    /// Command to interface with various method from the Notion API
    Notion(NotionArgs)
}
