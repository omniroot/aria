-- lua/aria.lua
-- Минимальная библиотека с только типами (EmmyLua аннотации).
-- Никакой логики/функций — только описание типов для LSP.

---@alias PackageGroups table<string, string[]>
-- ^ произвольные группы пакетов: ключ -> список пакетов

---@class Service
---@field name string            -- имя сервиса
---@field action string?         -- действие, например "enable_and_start"
---@field scope string?          -- "user" | "system" и т.д.

---@class Systemd
---@field services Service[]?    -- массив сервисов (опционально)

---@class SymlinkEntry
---@field source string
---@field target string
---@field use_sudo boolean?
---@field type string?           -- например "children" / "file" / "dir" — произвольная строка

---@class ExecEntry
---@field cmd string

---@class AriaConfig
---@field packages PackageGroups?     -- таблица с произвольными группами пакетов
---@field systemd Systemd?            -- systemd-конфигурация (опционально)
---@field symlinks SymlinkEntry[]?    -- опционально
---@field exec ExecEntry[]?           -- опционально
---@field [string] any?               -- разрешаем доп. произвольные поля (на будущее)

---@class AriaModule
---@field config AriaConfig?          -- если пользователь хочет положить сюда конфиг, LSP подскажет поля
---@field default_config AriaConfig?  -- (пустая заглушка, можно установить при init)

---@type AriaModule
local M = {}

-- Не кладём сюда реальные дефолты — чтобы не жестко привязывать имена групп.
-- Оставляем пустые поля: конфиг может быть пустым и это валидно.
M.config = nil
M.default_config = {}

--- Setup module config. Merges defaults with user config and normalizes shapes.
--- Returns the normalized effective config.
---@param cfg AriaConfig?
---@return AriaConfig
function M.setup(cfg)
  cfg = cfg or {}
  -- merge user cfg into default; default values are mostly nil (so user wins)
  -- local merged = deep_extend(M.default, cfg)
  -- local norm = M.normalize(merged)
  M._config = cfg
  return M._config
end

return M
