use std::process::{Command, Stdio};

use std::fs;

fn main() -> std::io::Result<()> {
    let mut cmd_args: Vec<String> = Vec::new();

    #[cfg(target_os = "windows")]
    let cmd = "cmd";

    #[cfg(target_os = "windows")]
    cmd_args.extend(vec!["/C".to_string(), "start".to_string()]);

    #[cfg(target_os = "linux")]
    let cmd = "xdg-open";

    #[cfg(target_os = "macos")]
    let cmd = "open";

    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        args.push(String::from("."));
    }

    let p = args.join(" ");

    if p.contains("://") {
        cmd_args.push(p);
    } else {
        let abs1 = fs::canonicalize(p)?;

        let absolute = abs1.into_os_string().into_string().unwrap();
        cmd_args.push(absolute);
    }

    let _ = Command::new(cmd)
        .args(cmd_args)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    Ok(())
}
