use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceAddRequest {
    pub client_info: ClientInfo,
    pub authentication: Authentication,
    pub device_info: DeviceInfo,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ClientInfo {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: String,
    pub binary_version: u32,
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            name: "IDCRL".to_owned(),
            version: "1.0".to_owned(),
            binary_version: 55,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Authentication {
    pub membername: String,
    pub password: String,
}

impl Authentication {
    pub fn new(membername: String, password: String) -> Self {
        Self {
            membername,
            password,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceInfo {
    pub components: Vec<Component>,
}
#[derive(Serialize, Debug)]
pub struct Component {
    #[serde(rename = "@name")]
    pub name: u32,
    #[serde(rename = "$value")]
    pub value: Option<String>,
    #[serde(rename = "@error")]
    pub error: Option<String>,
}

impl Component {
    pub fn error(id: u32) -> Self {
        Self {
            name: id,
            value: None,
            error: Some("-2147024894".to_owned()),
        }
    }

    pub fn new(id: u32, value: String) -> Self {
        Self {
            name: id,
            value: Some(value),
            error: None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceAddResponse {
    pub success: bool,
    pub puid: String,
    pub device_tpm_key_state: Option<u32>,
    pub license: License,
    pub key_holder_license: KeyHolderLicense,
    #[serde(rename = "HWDeviceID")]
    pub hw_device_id: String,
    pub global_device_id: String,
    pub license_key_sequence: String,
    pub license_signature_key_version: u32,
    pub server_info: ServerInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct License {
    #[serde(rename = "SPLicenseBlock")]
    pub splicense_block: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct KeyHolderLicense {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ServerInfo {
    #[serde(rename = "@ServerTime")]
    pub server_time: String,
    #[serde(rename = "$value")]
    pub id: String,
}
