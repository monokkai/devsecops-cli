use crate::cli::{GitArgs, GitSubcommand};
use colored::*;
use dialoguer::{
    Confirm, Input, Select,
    theme::{ColorfulTheme, Theme},
};
use std::process::{Command, Output};

const COMMIT_TYPES: &[(&str, &str, Color, &str)] = &[
    ("feat", "A new feature", Color::BrightGreen, "✨"),
    ("fix", "A bug fix", Color::BrightRed, "🐛"),
    (
        "docs",
        "Documentation only changes",
        Color::BrightBlue,
        "📚",
    ),
    (
        "style",
        "Changes that do not affect meaning",
        Color::BrightMagenta,
        "🎨",
    ),
    (
        "refactor",
        "A code change that neither fixes a bug nor adds a feature",
        Color::BrightCyan,
        "♻️",
    ),
    (
        "perf",
        "A code change that improves performance",
        Color::BrightYellow,
        "⚡️",
    ),
    (
        "test",
        "Adding missing or correcting tests",
        Color::BrightWhite,
        "✅",
    ),
    (
        "chore",
        "Changes to build process or tools",
        Color::White,
        "🔧",
    ),
];

struct GitHelper;

impl GitHelper {
    fn execute_git_command(args: &[&str]) -> Result<Output, std::io::Error> {
        Command::new("git").args(args).output()
    }

    fn is_commit_successful(output: &Output) -> bool {
        let stderr = String::from_utf8_lossy(&output.stderr);
        stderr.is_empty() || stderr.contains('[') || stderr.contains(']')
    }

    fn get_changed_files() -> Result<(Vec<String>, Vec<String>, Vec<String>), std::io::Error> {
        let output = Command::new("git")
            .args(&["status", " - - porcelain"])
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
            println!("\n{} {}: ", "↻".yellow(), "Modified files".yellow());
            for file in modified {
                println!("  {}", file.yellow());
            }
        }

        if !added.is_empty() {
            println!("\n{} {}: ", " + ".green(), "Added files".green());
            for file in added {
                println!("  {}", file.green());
            }
        }

        if !deleted.is_empty() {
            println!("\n{} {}: ", " - ".red(), "Deleted files".red());
            for file in deleted {
                println!("  {}", file.red());
            }
        }

        let total = modified.len() + added.len() + deleted.len();
        println!("\n{} {}: {}", "ℹ".blue(), "Total changes".blue(), total);
    }

    fn add_changes() -> Result<(), String> {
        match Self::execute_git_command(&["add", "."]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✅ Added all changes".green());
                Ok(())
            }
            Ok(_) => Err("❌ Git add failed".red().to_string()),
            Err(e) => Err(format!("{} Failed to execute git add: {}", "❌".red(), e)),
        }
    }

    fn create_commit(message: &str) -> Result<(), String> {
        let commit_output = Self::execute_git_command(&["commit", " - m", message]);

        match commit_output {
            Ok(output) if Self::is_commit_successful(&output) => {
                let msg = String::from_utf8_lossy(&output.stderr);
                if !msg.is_empty() {
                    println!("{} {}", "✅".green(), msg.trim().green());
                } else {
                    println!("{}", "✅ Commit created successfully".green());
                }
                Ok(())
            }
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                if error_msg.contains("nothing to commit") {
                    Err("⚠️ Nothing to commit".yellow().to_string())
                } else {
                    Err(format!(
                        "{} {}",
                        "❌ Git commit failed:".red(),
                        error_msg.trim().red()
                    ))
                }
            }
            Err(e) => Err(format!(
                "{} Failed to execute git commit: {}",
                "❌".red(),
                e
            )),
        }
    }

    fn push_changes() -> Result<(), String> {
        match Self::execute_git_command(&["push"]) {
            Ok(output) if output.status.success() => {
                println!("{}", "✅ Pushed successfully".green());
                Ok(())
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
                    println!("  git pull - - rebase");
                    println!("Then push again with:");
                    println!("  monokkai git -a -m \"your message\" -p");
                    Err("Push rejected".to_string())
                } else {
                    Err(format!(
                        "{} {}",
                        "❌ Git push failed:".red(),
                        error_msg.trim().red()
                    ))
                }
            }
            Err(e) => Err(format!("{} Failed to execute git push: {}", "❌".red(), e)),
        }
    }

    fn pull_changes(rebase: bool) -> Result<(), String> {
        let pull_type = if rebase { "--rebase" } else { "" };
        match Self::execute_git_command(&["pull", pull_type]) {
            Ok(output) if output.status.success() => {
                println!(
                    "{} {}",
                    "✅ Pull successful:".green(),
                    String::from_utf8_lossy(&output.stdout).trim()
                );
                Ok(())
            }
            Ok(output) => Err(format!(
                "{} {}",
                "❌ Pull failed:".red(),
                String::from_utf8_lossy(&output.stderr).trim()
            )),
            Err(e) => Err(format!("{} Failed to execute git pull: {}", "❌".red(), e)),
        }
    }
}

struct CommitHelper;

impl CommitHelper {
    fn select_commit_type(theme: &dyn Theme) -> Result<(&'static str, Color), std::io::Error> {
        let items = COMMIT_TYPES
            .iter()
            .map(|(t, d, c, _emoji)| format!("{}: {}", t.color(*c), d))
            .collect::<Vec<_>>();

        let selection = Select::with_theme(theme)
            .with_prompt("Select the type of change that you're committing:")
            .items(&items)
            .default(0)
            .interact()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        Ok((COMMIT_TYPES[selection].0, COMMIT_TYPES[selection].2))
    }

    fn get_commit_scope(theme: &dyn Theme) -> Result<String, std::io::Error> {
        Input::with_theme(theme)
            .with_prompt("What is the scope of this change (e.g. component or file name):")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn get_short_description(theme: &dyn Theme) -> Result<String, std::io::Error> {
        Input::with_theme(theme)
            .with_prompt(
                "Write a short, imperative tense description of the change (max 75 chars):",
            )
            .validate_with(|input: &String| {
                if input.len() > 75 {
                    Err("Description must be 75 characters or less".to_string())
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn get_long_description(theme: &dyn Theme) -> Result<String, std::io::Error> {
        Input::with_theme(theme)
            .with_prompt("Provide a longer description of the change:")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn has_breaking_changes(theme: &dyn Theme) -> Result<bool, std::io::Error> {
        Confirm::with_theme(theme)
            .with_prompt("Are there any breaking changes?")
            .default(false)
            .interact()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn affects_open_issues(theme: &dyn Theme) -> Result<bool, std::io::Error> {
        Confirm::with_theme(theme)
            .with_prompt("Does this change affect any open issues?")
            .default(false)
            .interact()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn build_commit_message(
        commit_type: &str,
        type_color: Color,
        scope: &str,
        short_desc: &str,
        long_desc: &str,
        breaking: bool,
    ) -> String {
        let type_part = commit_type.color(type_color).to_string();

        let mut message = if scope.is_empty() {
            format!("{}: {}", type_part, short_desc)
        } else {
            format!("{}({}): {}", type_part, scope, short_desc)
        };

        if !long_desc.is_empty() {
            message.push_str(&format!("\n\n{}", long_desc));
        }

        if breaking {
            message.push_str(&format!("\n\n{}", "BREAKING CHANGE:".bright_red()));
        }

        message
    }

    fn interactive_commit() -> Result<String, std::io::Error> {
        let theme = ColorfulTheme::default();

        let (commit_type, type_color) = Self::select_commit_type(&theme)?;
        let scope = Self::get_commit_scope(&theme)?;
        let short_desc = Self::get_short_description(&theme)?;
        let long_desc = Self::get_long_description(&theme)?;
        let breaking = Self::has_breaking_changes(&theme)?;
        let _affects_issues = Self::affects_open_issues(&theme)?;

        Ok(Self::build_commit_message(
            commit_type,
            type_color,
            &scope,
            &short_desc,
            &long_desc,
            breaking,
        ))
    }
}

pub fn handle(args: GitArgs) {
    let commit_message = match args.subcommand {
        Some(GitSubcommand::Cz {
            add,
            push,
            pull,
            rebase,
        }) => match CommitHelper::interactive_commit() {
            Ok(msg) => {
                println!("\n{} Commit message:\n{}", "✏️".cyan(), msg);
                msg
            }
            Err(e) => {
                eprintln!("{} Failed to get commit message: {}", "❌".red(), e);
                return;
            }
        },
        None => args.message.unwrap_or_else(|| {
            eprintln!("{} Commit message is required", "❌".red());
            std::process::exit(1);
        }),
    };

    let (modified, added, deleted) = match GitHelper::get_changed_files() {
        Ok(changes) => changes,
        Err(e) => {
            eprintln!("{} Failed to get changed files: {}", "❌".red(), e);
            return;
        }
    };

    if args.add || matches!(args.subcommand, Some(GitSubcommand::Cz { add: true, .. })) {
        if let Err(e) = GitHelper::add_changes() {
            eprintln!("{}", e);
            return;
        }
    }

    GitHelper::print_changes(&modified, &added, &deleted);

    if let Err(e) = GitHelper::create_commit(&commit_message) {
        eprintln!("{}", e);
        return;
    }

    if args.push || matches!(args.subcommand, Some(GitSubcommand::Cz { push: true, .. })) {
        if let Err(e) = GitHelper::push_changes() {
            eprintln!("{}", e);
            return;
        }
    }

    if args.pull
        || args.rebase
        || matches!(args.subcommand, Some(GitSubcommand::Cz { pull: true, .. }))
    {
        if let Err(e) = GitHelper::pull_changes(args.rebase) {
            eprintln!("{}", e);
            return;
        }
    }

    println!("{}", "🎉 All operations completed successfully".green());
}
