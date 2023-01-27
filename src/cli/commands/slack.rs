use ::clap::Args;


#[derive(Args)]
/// Returns Slack status for the given user
pub struct SlackArgs {
    /// For now - just returns the string in reverse
    #[arg(short = 's', long = "string")]
    pub string: Option<String>,
}
