use ::clap::Subcommand;

use crate::cli::commands::slack::SlackArgs;
use crate::cli::commands::spotify::SpotifyArgs;

// first tier subcommands - e.g. acbli slack or abcli evernote
#[derive(Subcommand)]
pub enum Commands {
    /// Command to interface with various methods from the Slack API
    Slack(SlackArgs),
    /// Command to interface with various methods from the Spotify API
    Spotify(SpotifyArgs),
}