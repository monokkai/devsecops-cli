use colored::Colorize;
use dotenvy::dotenv;
use std::env;
use std::process::Command;

use crate::cli::{DockerAction, DockerArgs};

pub fn handle(args: DockerArgs) {
    dotenv().ok();
    let dockerhub_user = env::var("DOCKERHUB_USER").expect("DOCKERHUB_USER not set in .env");

    match args.action {
        DockerAction::Scan { image } => {
            println!("{} {}", "🔍 Scanning image:".blue(), image.green());
            let status = Command::new("trivy")
                .args(["image", &image])
                .status()
                .expect("Failed to execute trivy");

            if !status.success() {
                eprintln!("{}", "❌ Scan failed".red());
            }
        }
        DockerAction::Push { image, tag } => {
            let tag = tag.unwrap_or_else(|| "latest".to_string());
            let target = format!("{}/{}:{}", dockerhub_user, image, tag);

            println!("{} {}", "🚀 Pushing:".blue(), target.green());

            let build_status = Command::new("docker")
                .args(["build", "-t", &target, "."])
                .status()
                .expect("Docker build failed");

            if build_status.success() {
                Command::new("docker")
                    .args(["push", &target])
                    .status()
                    .expect("Docker push failed");
                println!("{}", "✅ Push successful".green());
            } else {
                eprintln!("{}", "❌ Build failed".red());
            }
        }
    }
}
