use colored::Colorize;
use std::process::{Command, Output};

use crate::cli::GitArgs;

fn execute_git_command(args: &[&str]) -> Result<Output, std::io::Error> {
    Command::new("git").args(args).output()
}

fn is_commit_successful(output: &Output) -> bool {
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.is_empty() || stderr.contains("[") || stderr.contains("]")
}

pub fn handle(args: GitArgs) {
    // Git add
    if args.add {
        match execute_git_command(&["add", "."]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✅ Added all changes".green());
            }
            Ok(_) => {
                eprintln!("{}", "❌ Git add failed".red());
                return;
            }
            Err(e) => {
                eprintln!("{} Failed to execute git add: {}", "❌".red(), e);
                return;
            }
        }
    }

    // Git commit
    let commit_output = execute_git_command(&["commit", "-m", &args.message]);
    match commit_output {
        Ok(output) if is_commit_successful(&output) => {
            let msg = String::from_utf8_lossy(&output.stderr);
            if !msg.is_empty() {
                println!("{} {}", "✅".green(), msg.trim().green());
            } else {
                println!("{}", "✅ Commit created successfully".green());
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            if error_msg.contains("nothing to commit") {
                println!("{} {}", "⚠️".yellow(), "Nothing to commit".yellow());
            } else {
                eprintln!(
                    "{} {}",
                    "❌ Git commit failed:".red(),
                    error_msg.trim().red()
                );
            }
            return;
        }
        Err(e) => {
            eprintln!("{} Failed to execute git commit: {}", "❌".red(), e);
            return;
        }
    }

    // Git push
    if args.push {
        match execute_git_command(&["push"]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✅ Pushed successfully".green());
            }
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);

                if error_msg.contains("Updates were rejected") {
                    println!(
                        "{} {}",
                        "⚠️".yellow(),
                        "Push rejected - remote has new changes".yellow()
                    );
                    println!("{} Try running:", "💡".blue());
                    println!("  git pull --rebase");
                    println!("Then push again with:");
                    println!("  monokkai git -a -m \"your message\" -p");
                } else {
                    eprintln!("{} {}", "❌ Git push failed:".red(), error_msg.trim().red());
                }
                return;
            }
            Err(e) => {
                eprintln!("{} Failed to execute git push: {}", "❌".red(), e);
                return;
            }
        }
    }

    // Git pull / pull --rebase
    if args.pull || args.rebase {
        let pull_type = if args.rebase { "--rebase" } else { "" };
        match execute_git_command(&["pull", pull_type]) {
            Ok(output) if output.status.success() => {
                println!(
                    "{} {}",
                    "✅ Pull successful:".green(),
                    String::from_utf8_lossy(&output.stdout).trim()
                );
            }
            Ok(output) => {
                eprintln!(
                    "{} {}",
                    "❌ Pull failed:".red(),
                    String::from_utf8_lossy(&output.stderr).trim()
                );
                return;
            }
            Err(e) => {
                eprintln!("{} Failed to execute git pull: {}", "❌".red(), e);
                return;
            }
        }
    }

    println!("{}", "🎉 All operations completed successfully".green());
}
