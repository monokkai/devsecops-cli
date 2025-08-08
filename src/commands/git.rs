use colored::Colorize;
use std::process::Command;

use crate::cli::GitArgs;

pub fn handle(args: GitArgs) {
    if args.add {
        let add_status = Command::new("git")
            .args(["add", "."])
            .status()
            .expect("Failed to execute git add");

        if !add_status.success() {
            eprintln!("{}", "❌ Git add failed".red());
            return;
        }
        println!("{}", "✓ Added all changes".green());
    }

    let commit_status = Command::new("git")
        .args(["commit", "-m", &args.message])
        .status()
        .expect("Failed to execute git commit");

    if !commit_status.success() {
        eprintln!("{}", "❌ Git commit failed".red());
        return;
    }

    if args.push {
        let push_status = Command::new("git")
            .arg("push")
            .status()
            .expect("Failed to execute git push");

        if !push_status.success() {
            eprintln!("{}", "❌ Git push failed".red());
            return;
        }
    }

    println!("{}", "✅ Git operation completed successfully".green());
}
