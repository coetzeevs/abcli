use ::clap::{Args,Subcommand};


#[derive(Args)]
/// Returns Slack status for the given user
pub struct Status {
    /// For now - just returns the string in reverse
    #[arg(short = 's', long = "status")]
    pub status: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Returns the given status string in reverse
    Status(Status),
}
