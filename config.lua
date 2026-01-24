local aria = require("lua.aria")

-- os.execute("echo 123")

aria.config = {
  packages = {
    terminal = { "bat", "eza" },
    -- desktop = { "hyprland", "pipewire" },
    audio = { "pipewire" },
    -- wm = { "sway", "hyprland" },
    -- apps = { "firefox", "discord", "telegram-desktop" },
  },
}

return aria.config
