use clap::{Subcommand, ValueEnum};

use super::notion::Notion;

#[derive(Debug, Subcommand)]
pub enum Commands {
    // /// Clones repos
    // #[command(arg_required_else_help = true)]
    // Clone {
    //     /// The remote to clone
    //     remote: String,
    // },
    // /// Compare two commits
    // Diff {
    //     #[arg(value_name = "COMMIT")]
    //     base: Option<OsString>,
    //     #[arg(value_name = "COMMIT")]
    //     head: Option<OsString>,
    //     #[arg(last = true)]
    //     path: Option<OsString>,
    //     #[arg(
    //         long,
    //         require_equals = true,
    //         value_name = "WHEN",
    //         num_args = 0..=1,
    //         default_value_t = ColorWhen::Auto,
    //         default_missing_value = "always",
    //         value_enum
    //     )]
    //     color: ColorWhen,
    // },
    // /// pushes things
    // #[command(arg_required_else_help = true)]
    // Push {
    //     /// The remote to target
    //     remote: String,
    // },
    // /// adds things
    // #[command(arg_required_else_help = true)]
    // Add {
    //     /// Stuff to add
    //     #[arg(required = true)]
    //     path: Vec<PathBuf>,
    // },
    Notion(Notion),
    // #[command(external_subcommand)]
    // External(Vec<OsString>),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
