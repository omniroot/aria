// src/config.rs
// Здесь лежат все структуры, в которые мы хотим десериализовать
// конфиг из Lua. Они простые и понятные — с derive Serialize/Deserialize
// чтобы можно было легко печатать в JSON или работать дальше.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AriaConfig {
    // Произвольные группы пакетов: ключ — имя группы, значение — список пакетов.
    // Опционально — пустой конфиг валиден.
    #[serde(default)]
    pub packages: Option<HashMap<String, Vec<String>>>,

    #[serde(default)]
    pub systemd: Option<Systemd>,

    #[serde(default)]
    pub symlinks: Option<Vec<SymlinkEntry>>,

    #[serde(default)]
    pub exec: Option<Vec<ExecEntry>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Systemd {
    #[serde(default)]
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub action: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymlinkEntry {
    pub source: String,
    pub target: String,
    pub use_sudo: Option<bool>,
    #[serde(rename = "type")]
    pub link_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecEntry {
    pub cmd: String,
}
