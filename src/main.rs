use std::process;
use structopt::StructOpt;

mod cli;
mod git;

fn main() {
    let args = cli::CliArgs::from_args();

    let branch1 = args.branch1;
    let branch2 = args.branch2;
    let exclude = args.exclude;
    let repo = args.repo_path;

    let result = git::compare_branches(&branch1, &branch2, exclude, repo);

    match result {
        Ok(commits) => {
            for commit in commits {
                println!("{}: {}", commit.date, commit.summary);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
