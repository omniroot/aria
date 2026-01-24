use anyhow::{Context, Result};
use clap::Parser;
use mlua::{Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

/// Простой парсер конфигурации aria (Lua -> Rust) и вывод её в JSON.
#[derive(Parser, Debug)]
#[command(name = "aria-config-parser")]
#[command(about = "Load Lua config and print typed representation", long_about = None)]
struct Args {
    /// Путь до Lua-конфига (например ~/.config/aria/config.lua)
    #[arg(value_name = "CONFIG")]
    config: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Aria {
    #[serde(default)]
    packages: Option<HashMap<String, Vec<String>>>, // произвольные группы пакетов
    #[serde(default)]
    systemd: Option<Systemd>,
    #[serde(default)]
    symlinks: Option<Vec<SymlinkEntry>>,
    #[serde(default)]
    exec: Option<Vec<ExecEntry>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Systemd {
    #[serde(default)]
    services: Option<Vec<Service>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    name: String,
    action: Option<String>,
    scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SymlinkEntry {
    source: String,
    target: String,
    use_sudo: Option<bool>,
    #[serde(rename = "type")]
    link_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecEntry {
    cmd: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let aria = load_lua_config(&args.config)
        .with_context(|| format!("failed to load config from {:?}", args.config))?;

    // Печатаем как красивый JSON
    let out = serde_json::to_string_pretty(&aria)?;
    println!("{}", out);

    Ok(())
}

/// Попытаться извлечь Aria из Lua-конфига.
///
/// Поддерживаем два варианта:
/// 1) Файл возвращает таблицу (например `return { packages = { ... } }`)
/// 2) Файл присваивает глобу `aria.config` (например `aria.config = { ... }; return aria.config`)
fn load_lua_config(path: &PathBuf) -> Result<Aria> {
    let src = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file {:?}", path))?;

    let lua = Lua::new();

    // Ставим имя chunk'а (не использую ? на set_name, он возвращает Chunk)
    let chunk = lua.load(&src).set_name(path.to_string_lossy().as_ref());

    // Выполняем код и получаем возвращаемое значение. Явно конвертируем mlua::Error в anyhow.
    let val = chunk
        .eval::<Value>()
        .map_err(|e| anyhow::anyhow!("error while evaluating lua config: {}", e))?;

    // Если файл явно вернул значение (не nil) — пробуем десериализовать это в Aria
    if !matches!(val, Value::Nil) {
        match lua.from_value::<Aria>(val) {
            Ok(aria) => return Ok(aria),
            Err(e) => {
                eprintln!(
                    "warning: couldn't deserialize returned value from config: {}",
                    e
                );
                // идём дальше, попробуем global aria
            }
        }
    }

    // Если ничего не вернули — пробуем взять global aria.config
    let globals = lua.globals();
    if let Ok(aria_table_val) = globals.get::<mlua::Value>("aria") {
        if !matches!(aria_table_val, Value::Nil) {
            if let Value::Table(t) = aria_table_val {
                // пробуем aria.config
                if let Ok(cfg_val) = t.get::<mlua::Value>("config") {
                    if !matches!(cfg_val, Value::Nil) {
                        let aria_struct = lua.from_value::<Aria>(cfg_val).map_err(|e| {
                            anyhow::anyhow!("failed to deserialize aria.config: {}", e)
                        })?;
                        return Ok(aria_struct);
                    }
                }

                // если aria.config отсутствует, попробуем десериализовать саму таблицу aria как конфиг
                let aria_struct = lua.from_value::<Aria>(Value::Table(t)).map_err(|e| {
                    anyhow::anyhow!("failed to deserialize global aria table: {}", e)
                })?;
                return Ok(aria_struct);
            }
        }
    }

    anyhow::bail!(
        "no config found: the file didn't return a table and global 'aria' table is missing or empty"
    );
}
