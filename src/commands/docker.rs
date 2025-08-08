use colored::*;
use dotenv::dotenv;
use std::env;
use std::process::Command;

pub async fn handle(args: super::cli::DockerArgs) {
    dotenv().ok();
    let dockerhub_user = env::var("DOCKERHUB_USER").expect("Set DOCKERHUB_USER in .env");

    match args.action {
        DockerAction::Scan { image } => {
            println!("🔍 Scanning Docker image {}...", image.blue());
            let status = Command::new("trivy")
                .arg("image")
                .arg(&image)
                .status()
                .expect("Failed to run trivy");

            if !status.success() {
                eprintln!("{}", "❌ Scan failed".red());
            }
        }
        DockerAction::Push { image, tag } => {
            let tag = tag.unwrap_or_else(|| "latest".to_string());
            let target_image = format!("{}/{}:{}", dockerhub_user, image, tag);

            println!("🚀 Pushing {} to DockerHub...", target_image.green());

            // Push
            let build_status = Command::new("docker")
                .args(["build", "-t", &target_image, "."])
                .status()
                .expect("Docker build failed");

            if !build_status.success() {
                eprintln!("{}", "❌ Build failed".red());
                return;
            }

            let push_status = Command::new("docker")
                .arg("push")
                .arg(&target_image)
                .status()
                .expect("Docker push failed");

            if push_status.success() {
                println!("✅ Successfully pushed to DockerHub");
            } else {
                eprintln!("{}", "❌ Push failed".red());
            }
        }
    }
}
