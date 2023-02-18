use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct CliArgs {
    #[structopt(required = true, value_name = "BRANCH1")]
    pub branch1: String,

    #[structopt(required = true, value_name = "BRANCH2")]
    pub branch2: String,

    #[structopt(short = "e", long = "exclude", value_name = "EXCLUDE")]
    pub exclude: Option<Vec<String>>,

    #[structopt(parse(from_os_str))]
    pub repo_path: Option<PathBuf>,
}
