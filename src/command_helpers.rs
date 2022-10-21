use serde::Serialize;
use which::which;

pub fn vexcom_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    which("vexcom")?;
    let output = std::process::Command::new("vexcom").arg(command).output()?;
    Ok(output.status.to_string())
}

pub fn make() -> Result<String, Box<dyn std::error::Error>> {
    // check if arm-none-eabi-g++ exists?
    // check that we're in a pros project?
    let output = std::process::Command::new("make").output()?;
    Ok(output.status.to_string())
}

#[derive(Serialize, Debug)]
struct ErrorMessage<'a> {
    name: &'a str,
    description: String,
}

pub fn display_error(is_json: bool, name: &str, message: String) {
    if is_json {
        eprintln!(
            "{}",
            serde_json::to_string(&ErrorMessage {
                name: name,
                description: message
            })
            .unwrap()
        )
    } else {
        println!("{}", message)
    }
}
