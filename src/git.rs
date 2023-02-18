use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct Commit {
    pub date: String,
    pub summary: String,
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
    let branch1_commits: Vec<Commit> = raw_branch1_commits
        .into_iter()
        .filter_map(|msg| parse_commit_message(&msg, words_to_exclude.clone()))
        .collect();
    let branch2_commits: Vec<Commit> = raw_branch2_commits
        .into_iter()
        .filter_map(|msg| parse_commit_message(&msg, words_to_exclude.clone()))
        .collect();

    let commits = exclude(compare(branch1_commits, branch2_commits), words_to_exclude);

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

fn compare(commits1: Vec<Commit>, commits2: Vec<Commit>) -> Vec<Commit> {
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
    let commit_messages = git_log_output_str
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    commit_messages
}

fn parse_commit_message(msg: &str, exclude: Option<Vec<String>>) -> Option<Commit> {
    let fields: Vec<&str> = msg.split('|').collect();
    let _commit_id = fields[0];
    let summary = fields[2].to_string();

    if let Some(exclude) = exclude {
        if exclude.iter().any(|word| summary.contains(word)) {
            return None;
        }
    }

    let date = fields[1].to_string();

    Some(Commit { date, summary })
}
