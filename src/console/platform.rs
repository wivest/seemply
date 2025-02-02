use std::process::Command;

pub fn get_command() -> Command {
    let mut command: Command;
    if cfg!(target_os = "windows") {
        command = Command::new("cmd");
        command.args(["/C", "echo"]);
    } else {
        command = Command::new("sh");
        command.args(["-c", "echo"]);
    }
    return command;
}
