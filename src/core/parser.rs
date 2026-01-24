// src/core/parser.rs
//
// Упрощённая и понятная логика: читаем Lua-файл, выполняем его в mlua,
// пытаемся достать таблицу-конфиг тремя способами (порядок — важен):
// 1) если файл `return { ... }` — берем возвращённую таблицу;
// 2) если файл модуль делает `aria.config = { ... }` — берем aria.config;
// 3) если файл экспортирует модуль `aria = { ... }` — пробуем саму таблицу aria.
//
// Везде ошибки mlua мы конвертируем в anyhow::Error с простым описанием.
// Поясняющие комментарии внутри.

use mlua::{Error, Lua, LuaSerdeExt, Value};
use std::{fs, path::Path};

use crate::config::AriaConfig;

pub fn load_config_from_lua<P: AsRef<Path>>(path: P) -> Result<AriaConfig, Error> {
    // 1. читаем файл
    let src = fs::read_to_string(&path).expect("Failed to read lua config");
    // .map_err(|e| anyhow!("failed to read config file {:?}: {}", path.as_ref(), e))?;

    let lua = Lua::new();
    let value: Value = lua.load(&src).eval()?;
    let json: AriaConfig = lua.from_value(value)?;

    Ok(json)
}
// if let Value::Table(_) = &result {
//     return lua
//         .from_value(result)
//         .expect("failed to deserialize returned table");
// }

// println!("{:?}", result);

// // 3. загружаем chunk и выполняем его
// //    set_name возвращает Chunk (не Result), поэтому не используем ? на нём
// let chunk = lua
//     .load(&src)
//     .set_name(path.as_ref().to_string_lossy().as_ref());

// // eval возвращает mlua::Result<Value>; конвертируем ошибку в anyhow, чтобы
// // не тащить mlua::Error наружу.
// let returned = chunk
//     .eval::<Value>()
//     .map_err(|e| anyhow!("error while evaluating lua config: {}", e))?;

// // 4. Если файл явно вернул таблицу — пытаемся десериализовать её в Aria
// if let Value::Table(_) = &returned {
//     return lua
//         .from_value(returned)
//         .map_err(|e| anyhow!("failed to deserialize returned table: {}", e));
// }

// // 5. Иначе пробуем global `aria` (модуль в package.path или aria.config)
// let globals = lua.globals();
// match globals.get::<Value>("aria") {
//     Ok(Value::Table(t)) => {
//         // 5.a если есть aria.config — пробуем его
//         if let Ok(cfg_val) = t.get::<Value>("config") {
//             if !matches!(cfg_val, Value::Nil) {
//                 return lua
//                     .from_value(cfg_val)
//                     .map_err(|e| anyhow!("failed to deserialize aria.config: {}", e));
//             }
//         }
//         // 5.b иначе пробуем десериализовать саму таблицу aria
//         return lua
//             .from_value(Value::Table(t))
//             .map_err(|e| anyhow!("failed to deserialize global aria table: {}", e));
//     }
//     // aria отсутствует или не таблица — продолжаем дальше
//     _ => {}
// }

// // 6. если ничего не найдено — возвращаем понятную ошибку
// Err(anyhow!(
//     "no config found: file didn't return a table and global 'aria' is missing"
// ))
