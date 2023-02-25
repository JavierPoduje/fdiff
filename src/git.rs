use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct Commit {
    pub _id: String,
    pub date: String,
    pub summary: String,
}

impl Commit {
    pub fn new(msg: String) -> Self {
        let fields: Vec<&str> = msg.split('|').collect();
        let _id = fields[0].to_string();

        let summary = fields[2].to_string();
        let date = fields[1].to_string();

        Self { _id, date, summary }
    }
}

pub fn compare_branches(
    branch1: &str,
    branch2: &str,
    words_to_exclude: Option<Vec<String>>,
    maybe_repo_path: Option<PathBuf>,
) -> Result<Vec<Commit>, Box<dyn Error>> {
    let repo_path = match get_repo_path(maybe_repo_path) {
        Ok(repo_path) => repo_path,
        Err(err) => return Err(err),
    };

    let raw_branch1_output = get_branch_commits(&repo_path, branch1).unwrap();
    let raw_branch2_output = get_branch_commits(&repo_path, branch2).unwrap();

    let raw_branch1_commits = parse_git_output(raw_branch1_output);
    let raw_branch2_commits = parse_git_output(raw_branch2_output);

    let branch1_commits = raw_branch1_commits.into_iter().map(Commit::new).collect();
    let branch2_commits = raw_branch2_commits.into_iter().map(Commit::new).collect();

    let commits = exclude(subtract(branch1_commits, branch2_commits), words_to_exclude);

    Ok(commits)
}

fn get_repo_path(repo_path: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
    let mut git_log_cmd = Command::new("git");

    if let Some(repo_path) = repo_path {
        git_log_cmd.current_dir(repo_path);
    }

    let git_top_level_output = git_log_cmd
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !git_top_level_output.status.success() {
        return Err("Not inside a Git repository".into());
    }

    Ok(String::from_utf8_lossy(&git_top_level_output.stdout)
        .trim()
        .to_string())
}

fn exclude(mut commits: Vec<Commit>, exclusions: Option<Vec<String>>) -> Vec<Commit> {
    let to_exclude = if let Some(words) = exclusions {
        HashSet::from_iter(words)
    } else {
        HashSet::new()
    };
    commits.retain(|commit| !to_exclude.contains(&commit.summary));
    commits
}

fn subtract(commits1: Vec<Commit>, commits2: Vec<Commit>) -> Vec<Commit> {
    let hash = commits2.iter().fold(HashSet::new(), |mut hash, commit| {
        hash.insert(commit.summary.to_string());
        hash
    });

    let mut commits = Vec::new();

    for commit in commits1 {
        if !hash.contains(&commit.summary) {
            commits.push(commit);
        }
    }

    commits
}

fn get_branch_commits(repo_path: &str, branch: &str) -> Result<Output, std::io::Error> {
    Command::new("git")
        .current_dir(repo_path)
        .args([
            "log",
            branch,
            "--pretty=format:%h|%ad|%s",
            "--date=format:%Y-%m-%d",
        ])
        .output()
}

fn parse_git_output(raw_commits: Output) -> Vec<String> {
    let git_log_output_str = String::from_utf8_lossy(&raw_commits.stdout);
    git_log_output_str
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}
