use colored::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn handle(args: &crate::cli::LogArgs) {
    let log_format = format!(
        "%C(auto)%h%Creset - %C(bold yellow)%an%Creset - %C(green)%s%Creset - %C(cyan)%ad%Creset"
    );

    let mut git_log = Command::new("git");

    git_log
        .arg("log")
        .arg("--pretty=format:".to_owned() + &log_format)
        .arg("--date=format:%Y-%m-%d %H:%M")
        .arg("--color=always");

    if args.compact {
        git_log.arg("--oneline");
    }
    if args.graph {
        git_log.arg("--graph");
    }
    if let Some(limit) = args.limit {
        git_log.arg(format!("-{}", limit));
    }

    let child = git_log
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git log");

    let reader = BufReader::new(child.stdout.expect("Failed to capture stdout"));

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.splitn(4, " - ").collect();
            if parts.len() == 4 {
                println!(
                    "{:<10} {:<20} {:<50} {}",
                    parts[0].red().bold(),
                    parts[1].yellow(),
                    parts[2].green(),
                    parts[3].cyan()
                );
            } else {
                println!("{}", line);
            }
        }
    }
}
