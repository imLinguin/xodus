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
        .header("Content-Type", "application/xml")
        .body(data)
        .send()
        .await?;

    let text = response.text().await?;
    println!("{text:#?}");

    let resp: DeviceAddResponse = quick_xml::de::from_str(&text).expect("Failed to de xml");
    
    println!("{resp:#?}");

    Ok(())
}
