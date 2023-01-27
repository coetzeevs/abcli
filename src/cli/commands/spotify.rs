use ::clap::Args;


#[derive(Args)]
/// Returns Slack status for the given user
pub struct SpotifyArgs {
    /// For now - just returns the string in reverse
    #[arg(short = 'q', long = "query")]
    pub query: Option<String>,
}
