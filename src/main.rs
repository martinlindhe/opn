use std::process::{Command, Stdio};

use std::fs;

fn main() -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    let prefix = vec!["cmd".to_string(), "/C".to_string(), "start".to_string()];

    #[cfg(target_os = "linux")]
    let prefix = vec!["xdg-open".to_string()];

    #[cfg(target_os = "macos")]
    let prefix = vec!["open".to_string()];

    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        args.push(String::from("."));
    }

    let p = args.join(" ");

    let abs1 = fs::canonicalize(p)?;

    let absolute = abs1.into_os_string().into_string().unwrap();

    let mut cmd: Vec<String> = Vec::new();
    cmd.extend(prefix);
    cmd.push(absolute);

    let _ = Command::new("cmd")
        .args(cmd)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    Ok(())
}
