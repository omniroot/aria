use clap::{Parser, Subcommand};

use std::fs;

mod aria;
mod config;
mod core;
mod learn;
mod modules;

use config::Config;

use crate::learn::learn;

#[derive(Parser)]
#[command(name = "aria")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install packages
    Install {
        /// Path to config file
        config: String,
    },

    /// Apply full configuration
    Apply {
        /// Path to config file
        config: String,
    },
}

fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("failed to read config file");

    serde_yaml::from_str(&content).expect("failed to parse yaml")
}

fn main() {
    // let cli = Cli::parse();
    learn();
    // match cli.command {
    //     Commands::Install { config } => {
    //         let cfg: Config = load_config(&config);
    //         aria::install_packages(&cfg.packages);
    //     }
    //     Commands::Apply { config } => {
    //         // let cfg = load_config(&config);
    //         // aria::apply(&cfg);
    //     }
    // }
}
