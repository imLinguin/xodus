use crate::models::devicecredential::{DeviceAddRequest, DeviceAddResponse};

pub async fn login_device_credential(
    client: &reqwest::Client,
    data: DeviceAddRequest,
) -> reqwest::Result<()> {
    let data = quick_xml::se::to_string(&data).unwrap();

    let response = client
        .post(format!(
            "https://login.live.com/ppsecure/deviceaddcredential.srf"
        ))
        .header("User-Agent", "MSAWindows/55 (OS 10.0.26100.0.0 ge_release; IDK 10.0.26100.5074 ge_release; Cfg 16.000.29325.00; Test 0)")
        .header("Content-Type", "application/soap+xml")
        .header("Host", "login.live.com")
        .body(data)
        .send()
        .await?;

    let text = response.text().await?;
    println!("{text:#?}");

    let resp: DeviceAddResponse = quick_xml::de::from_str(&text).expect("Failed to de xml");

    println!("{resp:#?}");

    Ok(())
}

