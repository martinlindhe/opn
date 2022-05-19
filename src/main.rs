use std::process::{Command, Stdio};

fn main() {
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

    let mut cmd: Vec<String> = Vec::new();
    cmd.extend(prefix);
    cmd.extend(args);
 
    let _ = Command::new("cmd")
        .args(cmd)
        .stdout(Stdio::piped())
        .stdout(Stdio::piped())
        .output() 
        .expect("failed to execute process");
}
