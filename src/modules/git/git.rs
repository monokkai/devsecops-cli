use crate::cli::GitArgs;
use colored::Colorize;
use std::process::{Command, Output};

fn execute_git_command(args: &[&str]) -> Result<Output, std::io::Error> {
    Command::new("git").args(args).output()
}

fn is_commit_successful(output: &Output) -> bool {
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.is_empty() || stderr.contains("[") || stderr.contains("]")
}

fn get_changed_files() -> Result<(Vec<String>, Vec<String>, Vec<String>), std::io::Error> {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()?;

    let status = String::from_utf8_lossy(&output.stdout);
    let mut modified = Vec::new();
    let mut added = Vec::new();
    let mut deleted = Vec::new();

    for line in status.lines() {
        if line.len() >= 3 {
            let status = &line[..2];
            let file = line[3..].trim().to_string();

            match status {
                s if s.contains('M') => modified.push(file),
                s if s.contains('A') => added.push(file),
                s if s.contains('D') => deleted.push(file),
                _ => {}
            }
        }
    }

    Ok((modified, added, deleted))
}

fn print_changes(modified: &[String], added: &[String], deleted: &[String]) {
    if !modified.is_empty() {
        println!("\n{} Modified files:", "↻".yellow());
        for file in modified {
            println!("  {}", file.yellow());
        }
    }

    if !added.is_empty() {
        println!("\n{} Added files:", "+".green());
        for file in added {
            println!("  {}", file.green());
        }
    }

    if !deleted.is_empty() {
        println!("\n{} Deleted files:", "-".red());
        for file in deleted {
            println!("  {}", file.red());
        }
    }

    let total = modified.len() + added.len() + deleted.len();
    println!("\n{} Total changes: {}", "ℹ".blue(), total);
}

pub fn handle(args: GitArgs) {
    // Получаем список изменений ДО выполнения операций
    let changes_result = get_changed_files();
    let (modified, added, deleted) = match changes_result {
        Ok(changes) => changes,
        Err(e) => {
            eprintln!("{} Failed to get changed files: {}", "❌".red(), e);
            return;
        }
    };

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

    // Выводим изменения перед коммитом
    print_changes(&modified, &added, &deleted);

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

    if args.push {
        match execute_git_command(&["push"]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✅ Pushed successfully".green());
                println!("\n{} Commit message: {}", "✏️".cyan(), args.message.cyan());
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
