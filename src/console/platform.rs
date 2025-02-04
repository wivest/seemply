use std::process::Command;

pub fn get_command(sequence: &str) -> Command {
    let mut command: Command;
    if cfg!(target_os = "windows") {
        command = Command::new("cmd");
        command.args(["/C", "echo", sequence]);
    } else {
        command = Command::new("sh");
        command.args(["-c", "echo", sequence]);
    }
    return command;
}
