use crate::core::run::{ExecMode, ExecOptions, run};
use std::process::{Command, Stdio};

pub fn install(packages: &[String]) {
    let mut to_install = Vec::new();

    for pkg in packages {
        if is_installed(pkg) {
            println!("[SUCCESS] Package {} Skipped", pkg);
        } else {
            to_install.push(pkg.as_str());
        }
    }

    if to_install.is_empty() {
        return;
    }

    let status = run(
        &["pacman", "-S", "--noconfirm"],
        ExecOptions {
            root: false,
            mode: ExecMode::Quiet,
        },
    );

    // let status = Command::new("sudo")
    //     .args()
    //     .args(&to_install)
    //     .stdout(Stdio::null())
    //     .stderr(Stdio::null())
    //     .status()
    //     .expect("failed to run pacman");

    if status {
        for pkg in to_install {
            println!("[SUCCESS] Package {} Installed", pkg);
        }
    }
}

pub fn remove(packages: &[String]) {
    let mut to_remove = Vec::new();

    for pkg in packages {
        if is_installed(pkg) {
            to_remove.push(pkg.as_str());
        } else {
            println!("[SUCCESS] Package {} Skipped", pkg);
        }
    }

    if to_remove.is_empty() {
        return;
    }
    let status = run(
        &["pacman", "-Rns"],
        ExecOptions {
            root: false,
            mode: ExecMode::Quiet,
        },
    );
    // let status = Command::new("sudo")
    //     .args(["pacman", "-Rns", "--noconfirm"])
    //     .args(&to_remove)
    //     .stdout(Stdio::null())
    //     .stderr(Stdio::null())
    //     .status()
    //     .expect("failed to run pacman");

    if status {
        for pkg in to_remove {
            println!("[SUCCESS] Package {} Removed", pkg);
        }
    }
}

pub fn update() {
    let status = Command::new("sudo")
        .args(["pacman", "-Syu", "--noconfirm"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("failed to run pacman");

    if status.success() {
        println!("[SUCCESS] Package system Updated");
    }
}

fn is_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(["-Qi", pkg])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
