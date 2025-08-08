use colored::Colorize;
use std::process::{Command, Output};

use crate::cli::GitArgs;

fn execute_git_command(args: &[&str]) -> Result<Output, std::io::Error> {
    Command::new("git").args(args).output()
}

pub fn handle(args: GitArgs) {
    if args.add {
        match execute_git_command(&["add", "."]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✓ Added all changes".green());
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

    let commit_output = execute_git_command(&["commit", "-m", &args.message]);
    match commit_output {
        Ok(output) if output.status.success() => {
            println!("{}", "✓ Commit created successfully".green());
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            if error_msg.contains("nothing to commit") {
                println!("{} {}", "⚠️".yellow(), "Nothing to commit".yellow());
            } else {
                eprintln!("{} {}", "❌ Git commit failed:".red(), error_msg.red());
            }
            return;
        }
        Err(e) => {
            eprintln!("{} Failed to execute git commit: {}", "❌".red(), e);
            return;
        }
    }

    if args.push {
        match execute_git_command(&["push"]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✓ Pushed successfully".green());
            }
            Ok(output) => {
                eprintln!(
                    "{} {}",
                    "❌ Git push failed:".red(),
                    String::from_utf8_lossy(&output.stderr).red()
                );
                return;
            }
            Err(e) => {
                eprintln!("{} Failed to execute git push: {}", "❌".red(), e);
                return;
            }
        }
    }

    println!("{}", "✅ Git operation completed successfully".green());
}
