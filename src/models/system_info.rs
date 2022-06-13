use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub external_ip_address: String,
    pub internal_ip_address: String,
    pub os_type: String,
    pub machine_user: String,
    pub machine_name: String,
    pub process_name: String,
    pub pid: i32,
    pub architecture: String,
    pub implant_id: String,
}

impl SystemInfo {
    pub fn new(
        external_ip_address: String,
        internal_ip_address: String,
        os_type: String,
        machine_user: String,
        machine_name: String,
        process_name: String,
        pid: i32,
        architecture: String,
        implant_id: String,
    ) -> Self {
        SystemInfo {
            external_ip_address,
            internal_ip_address,
            os_type,
            machine_user,
            machine_name,
            process_name,
            pid,
            architecture,
            implant_id,
        }
    }
}
