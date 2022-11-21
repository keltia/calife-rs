use clap::{crate_name, crate_description, crate_authors, crate_version, Parser};

/// CLI options
#[derive(Parser, Debug)]
#[command(disable_version_flag = true)]
#[clap(name = crate_name!(), about = crate_description!())]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
    /// Debug mode
    #[clap(short = 'D', long)]
    pub debug: bool,
    /// Username to become (default = "root")
    pub who: Option<String>,
}
