use which::which;

pub fn vexcom_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    which("vexcom")?;
    let output = std::process::Command::new("vexcom").arg(command).output()?;
    Ok(output.status.to_string())
}