use std::process::Command;

#[cfg(target_os = "windows")]
pub fn get_command(sequence: &str) -> Command {
    let mut command = Command::new("cmd");
    command.args([
        "/c", "echo", sequence, "|", "set", "/p", "size=", "&", "echo", "%size%",
    ]);
    command
}

#[cfg(not(target_os = "windows"))]
pub fn get_command(sequence: &str) -> Command {
    // TODO: not implemented!
    let mut command = Command::new("sh");
    command.args(["-c", "echo", sequence]);
    command
}
