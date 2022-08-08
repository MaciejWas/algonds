use clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "This is a programming algo & ds. Argument paraser made by clap crate :)"
)]
pub struct AppArgs {
    #[clap(subcommand)]
    pub sub: Action,

    /// Path to yaml file with settings
    #[clap(long, value_parser)]
    pub settings: Option<String>,

    /// By default unicode is turned on.
    #[clap(long, action, default_value_t = false)]
    pub disable_unicode: bool,

    /// Path to yaml file with all problems, can be a web link
    #[clap(long, value_parser)]
    pub db_path: Option<String>,

    #[clap(long, value_parser)]
    pub solution_path: Option<String>,
    
    #[clap(long, value_parser)]
    pub compilation_step: Option<String>,

    #[clap(long, value_parser)]
    pub run_step: Option<String>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    /// Gets list of problems from github and populates db
    Update,

    /// Runs the terminal UI
    Run,
}

#[derive(clap::Args, Debug)]
pub struct Run {
    // #[clap(value_parser)]
    // pub solution_path: String,
}
