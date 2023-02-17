use clap::{Arg, Command};

pub fn get() -> clap::ArgMatches {
    Command::new("Git Branch diff")
        .version("1.0")
        .author("JavierPoduje")
        .about("Compares the commits by name of two branches in a Git repository")
        .arg(
            Arg::new("REPO_PATH")
                .help("Sets the path to the Git repository")
                .required(true),
        )
        .arg(
            Arg::new("BRANCH1")
                .help("Sets the name of the first branch to compare")
                .required(true),
        )
        .arg(
            Arg::new("BRANCH2")
                .help("Sets the name of the second branch to compare")
                .required(true),
        )
        .get_matches()
}
