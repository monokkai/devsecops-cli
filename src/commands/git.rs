use colored::Colorize;
use std::process::Command;

use crate::cli::GitArgs;

pub fn handle(args: GitArgs) {
    Command::new("git")
        .args(["commit", "-m", &args.message])
        .status()
        .expect("Failed to execute git commit");

    if args.push {
        Command::new("git")
            .arg("push")
            .status()
            .expect("Failed to execute git push");
    }

    println!("{}", "✅ Git operation completed".green());
}
