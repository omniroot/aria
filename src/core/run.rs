use std::process::{Command, Stdio};

pub enum ExecMode {
    Quiet,   // stdout/stderr -> null
    Inherit, // inherit stdio
}

pub struct ExecOptions {
    pub root: bool,
    pub mode: ExecMode,
}

impl Default for ExecOptions {
    fn default() -> Self {
        Self {
            root: false,
            mode: ExecMode::Quiet,
        }
    }
}

pub fn run(cmd: &[&str], opts: ExecOptions) -> bool {
    let mut command = if opts.root {
        let mut c = Command::new("sudo");
        c.arg("-E").args(cmd);
        c
    } else {
        let mut c = Command::new(cmd[0]);
        c.args(&cmd[1..]);
        c
    };

    match opts.mode {
        ExecMode::Quiet => {
            command.stdout(Stdio::null()).stderr(Stdio::null());
        }
        ExecMode::Inherit => {}
    }

    command.status().map(|s| s.success()).unwrap_or(false)
}
