use colored::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn handle(args: &crate::cli::LogArgs) {
    let mut log_command = Command::new("git");

    log_command
        .arg("log")
        .arg("--pretty=format:%C(yellow)%h%Creset - %C(bold blue)%an%Creset : %s")
        .arg("--date=short");

    if args.compact {
        log_command.arg("--oneline");
    }

    if args.graph {
        log_command.arg("--graph");
    }

    if let Some(limit) = args.limit {
        log_command.arg(format!("-{}", limit));
    }

    let child = log_command
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git log");

    let reader = BufReader::new(child.stdout.unwrap());

    println!(
        "{}",
        "┌──────────────────────────────────────────────────────┐".bright_black()
    );
    println!(
        "{}",
        "│                   COMMIT HISTORY                      │"
            .bright_blue()
            .bold()
    );
    println!(
        "{}",
        "└──────────────────────────────────────────────────────┘".bright_black()
    );

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("* ") || line.starts_with("| ") {
                // Graph
                println!("{}", line.bright_black());
            } else if let Some((hash, rest)) = line.split_once(" - ") {
                // Default view
                println!("{:<10} {}", hash, rest);
            } else {
                println!("{}", line);
            }
        }
    }
}
