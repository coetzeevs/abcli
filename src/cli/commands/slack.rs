use ::clap::{Args,Subcommand};


#[derive(Args)]
/// Returns Slack status for the given user
pub struct SlackArgs {
    /// For now - just returns the string in reverse
    #[arg(short = 's', long = "string")]
    pub string: Option<String>,
}

// first tier subcommands - e.g. acbli slack or abcli evernote
#[derive(Subcommand)]
pub enum Commands {
    /// Slack command to interface with various methods from the Slack API
    Slack(SlackArgs),
}
