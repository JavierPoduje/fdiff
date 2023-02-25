use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "fdiff",
    about = "Print the commits in the `branch1` that aren't present in the `branch2`"
)]
pub struct CliArgs {
    #[structopt(required = true, value_name = "branch1")]
    pub branch1: String,

    #[structopt(required = true, value_name = "branch2")]
    pub branch2: String,

    #[structopt(short = "e", long = "exclude", value_name = "exclude")]
    pub exclude: Option<Vec<String>>,

    #[structopt(parse(from_os_str))]
    pub repo_path: Option<PathBuf>,
}
