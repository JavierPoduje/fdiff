// In the file src/main.rs

mod args;

use crate::args::get;
use git2::{Error, Repository};

fn main() -> Result<(), Error> {
    // Parse CLI arguments using Clap
    let matches = get();

    let repo_path = matches.get_one::<String>("REPO_PATH").unwrap();
    let branch1 = matches.get_one::<String>("BRANCH1").unwrap();
    let branch2 = matches.get_one::<String>("BRANCH2").unwrap();

    let repo = Repository::open(repo_path)?;
    let head1 = repo
        .revparse_single(&format!("refs/heads/{}", branch1))?
        .id();
    let head2 = repo
        .revparse_single(&format!("refs/heads/{}", branch2))?
        .id();

    let mut revwalk = repo.revwalk()?;
    revwalk.hide(head2)?;
    revwalk.push(head1)?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        println!("{}", commit.summary().unwrap());
    }

    Ok(())
}
