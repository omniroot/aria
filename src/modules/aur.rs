use std::process::Command;

pub fn install(packages: &[String]) {
    for pkg in packages {
        install_package(pkg);
    }
}

fn install_package(pkg: &str) {
    let status = Command::new("yay")
        .args(["-S", "--noconfirm", "--needed", pkg])
        .status()
        .expect("failed to execute pacman");

    if status.success() {
        println!("[SUCCESS]: Package {} installed!", pkg);
    } else {
        eprintln!("[ERROR]: Failed to install package {}", pkg);
    }
}
