use std::ops::Not;

use crate::config::{Config, Packages};
use crate::modules;

pub fn install_packages(packages: &Option<Packages>) {
    if let Some(packages) = &packages {
        if let Some(pacman_packages) = &packages.pacman {
            modules::pacman::install(pacman_packages);
        }
        if let Some(aur_packages) = &packages.aur {
            modules::aur::install(aur_packages);
        }
    }
}

// pub fn apply(cfg: &Config) {
//     // apply = install + (позже другие ресурсы)
//     install_packages(cfg);
// }
