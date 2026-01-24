// src/main.rs
// Простая обвязка: читаем путь из аргументов и печатаем JSON результата.
mod config;
mod core;

use clap::Parser;
use serde_json;
use std::path::PathBuf;

use crate::config::AriaConfig;

#[derive(Parser, Debug)]
#[command(name = "aria")]
struct Args {
    /// Путь до Lua-конфига
    #[arg(value_name = "CONFIG")]
    config: PathBuf,
}

fn main() {
    let args = Args::parse();
    match core::parser::load_config_from_lua(&args.config) {
        Ok(config) => {
            if let Some(packages) = config.packages {
                for (group, pkgs) in packages {
                    println!("group: {}", group);

                    for pkg in pkgs {
                        println!("  - {}", pkg);
                    }
                }
            } else {
                println!("no packages defined");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // println!("{:?}", lua.packages);

    // .expect("Lua parser not returned value")

    // match core::parser::load_config_from_lua(&args.config) {
    //     Ok(aria) => {
    //         // красивый вывод в JSON — удобно для дебага
    //         let pretty = serde_json::to_string_pretty(&aria)
    //             .unwrap_or_else(|e| format!("failed to serialize result to JSON: {}", e));
    //         println!("{}", pretty);
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // }
}
