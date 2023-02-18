use chrono::prelude::*;
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
    exclude: Option<Vec<String>>,
    repo_path: Option<PathBuf>,
) -> Result<Vec<Commit>, Box<dyn Error>> {
    let mut git_log_cmd = Command::new("git");

    if let Some(repo_path) = repo_path {
        git_log_cmd.current_dir(repo_path);
    }

    let git_top_level_output = git_log_cmd
        .args(&["rev-parse", "--show-toplevel"])
        .output()?;

    if !git_top_level_output.status.success() {
        return Err("Not inside a Git repository".into());
    }

    let repo_path = String::from_utf8_lossy(&git_top_level_output.stdout)
        .trim()
        .to_string();

    let git_log_output = Command::new("git")
        .current_dir(&repo_path)
        .args(&[
            "log",
            &format!("{}..{}", branch2, branch1),
            "--pretty=format:%h|%ad|%s",
            "--date=format:'%Y-%m-%d'",
        ])
        .output()?;

    let raw_branch1_output = get_branch_commits(&repo_path, branch1).unwrap();
    let raw_branch2_output = get_branch_commits(&repo_path, branch2).unwrap();

    let raw_branch1_commits = parse_git_output(raw_branch1_output);
    let raw_branch2_commits = parse_git_output(raw_branch2_output);
    let branch1_commits: Vec<Commit> = raw_branch1_commits
        .into_iter()
        .filter_map(|msg| parse_commit_message(&msg, exclude.clone()))
        .collect();
    let branch2_commits: Vec<Commit> = raw_branch2_commits
        .into_iter()
        .filter_map(|msg| parse_commit_message(&msg, exclude.clone()))
        .collect();

    println!("branch1_commits: {:?}", branch1_commits);

    Ok(Vec::new())
}

fn get_branch_commits(repo_path: &str, branch: &str) -> Result<Output, std::io::Error> {
    Command::new("git")
        .current_dir(&repo_path)
        .args(&[
            "log",
            &format!("{}", branch),
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
