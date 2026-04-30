use xodus::{
    licensing::utils::generate_string,
    models::devicecredential::{Authentication, ClientInfo, DeviceAddRequest, DeviceInfo},
};

pub async fn run(client: &reqwest::Client) {
    let provision = DeviceAddRequest {
        client_info: ClientInfo::default(),
        authentication: Authentication::new(generate_string(16), generate_string(20)),
        device_info: DeviceInfo { components: vec![] },
    };

    
    println!("{provision:#?}");
}
