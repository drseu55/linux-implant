use gethostname::gethostname;
use local_ip_address::list_afinet_netifas;
use reqwest;
use std::ffi::OsString;
use std::{env, process};
use users::get_current_username;

use crate::errors::ImplantError;
use crate::models::system_info::SystemInfo;

pub fn gather_system_info() {
    println!("External ip address: {:?}", get_external_ip_address());
    println!("Internal ip address: {:?}", get_internal_ip_address());
    println!("OS type: {}", env::consts::OS);
    println!("Current username: {:?}", get_current_user());
    println!("Machine name: {:?}", gethostname());
    println!("Current process name: {:?}", get_current_executable_name());
    println!("Current PID: {}", process::id());
    println!("Architecture: {}", env::consts::ARCH);
}

fn get_external_ip_address() -> Result<String, ImplantError> {
    let external_ip_address = reqwest::blocking::get("http://api.ipify.org/")?.text()?;
    Ok(external_ip_address)
}

fn get_internal_ip_address() -> Result<String, local_ip_address::Error> {
    let network_interfaces = list_afinet_netifas()?;

    for (name, ip) in network_interfaces {
        if name != "lo:" {
            return Ok(ip.to_string());
        }
    }

    Ok("Not found".to_owned())
}

fn get_current_user() -> OsString {
    let current_username = get_current_username();

    if let Some(username) = current_username {
        username
    } else {
        OsString::new()
    }
}

fn get_current_executable_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}
