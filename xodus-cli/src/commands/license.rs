use xodus::{
    hardware,
    licensing::utils::generate_string,
    models::devicecredential::{Authentication, ClientInfo, DeviceAddRequest, DeviceInfo},
};

pub async fn run(client: &reqwest::Client) {
    let provision = DeviceAddRequest {
        client_info: ClientInfo::default(),
        authentication: Authentication::new(
            format!("02{}", generate_string(14)),
            generate_string(20),
        ),
        device_info: Some(DeviceInfo {
            id: "DeviceInfo".to_string(),
            components: hardware::probe_provision_components(),
        }),
    };

    xodus::api::live::login_device_credential(client, provision).await.expect("Failed to get device creds");

}
