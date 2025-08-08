use std::process::Command;

pub fn handle(args: cli::GitArgs) {
    // Git commit
    Command::new("git")
        .args(["commit", "-m", &args.message])
        .status()
        .expect("Failed to run git");

    if args.push {
        // Git push
        Command::new("git")
            .arg("push")
            .status()
            .expect("Ошибка git push");
    }

    println!("✅ Success: commit + push (--push)");
}
