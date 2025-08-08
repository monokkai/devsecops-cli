# DevSecOps CLI(monokkai) Toolkit 🔒

![Rust CI](https://github.com/monokkai/devsecops-cli/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue)
![Version](https://img.shields.io/badge/version-0.1.0-orange)

A Swiss Army knife for modern DevSecOps workflows, combining security scanning, container management, and automation in one fast Rust-powered CLI.

## 🚀 Features

- **Security First**:
    - Code vulnerability scanning
    - JWT/GitHub token validation
    - Docker image security audits

- **DevOps Automation**:
    - Smart Git workflows
    - Docker build/push pipelines
    - CI/CD ready architecture

- **Enterprise Ready**:
    - Async I/O where it matters
    - Proper error handling
    - Configurable via ENV

## 📦 Installation

### From Source
```bash
cargo install --path .
```

# Commands

```zsh
# Security Scanning
monokkai scan -p ./path/to/scan  # Scan directory for vulnerabilities

# Docker Operations
monokkai docker scan --image alpine:latest  # Scan Docker image
monokkai docker push --image myapp --tag v1.0  # Build and push to DockerHub

# Authentication
monokkai auth jwt --token "your.jwt.token"  # Validate JWT
monokkai auth github --token "ghp_yourtoken"  # Verify GitHub token

# Git Automation
monokkai git -m "commit message" [--push]  # Commit (and optionally push)
monokkai git -a -m "My commit message" --push # Already contains 'git add .' flag
```
