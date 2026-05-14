// Hardware probing utilities

use crate::{
    clep,
    models::devicecredential::{Component},
};
use std::process::{Command, Stdio};
use base64::prelude::*;

pub fn probe_provision_components() -> Vec<Component> {
    let mut components = Vec::with_capacity(16);
    let cmd = Command::new("pkexec").args(["cat", "/sys/firmware/dmi/entries/1-0/raw"]).stdout(Stdio::piped()).spawn().expect("Failed to get hardware entries");
    let output = cmd.wait_with_output().expect("Failed to wait");
    let smbios = output.stdout;

    let drive_serial = BASE64_STANDARD.decode("AA==").unwrap();
    let mut smbios_buf = [0; 256];
    let mut drive_buf = [0; 64];
    drive_buf
        .iter_mut()
        .zip(drive_serial.iter())
        .for_each(|(place, data)| *place = *data);
    smbios_buf
        .iter_mut()
        .zip(smbios.iter())
        .for_each(|(place, data)| *place = *data);
    let (clepv2, clepv4) = clep::challenge::get_license_challange(smbios_buf, drive_buf);

    components.push(Component::new(4113, "AA==".to_string()));
    components.push(Component::error(4101));
    components.push(Component::new(8196, BASE64_STANDARD.encode(clepv2)));
    components.push(Component::new(8197, BASE64_STANDARD.encode(clepv4)));
    components.push(Component::new(4145, "AQAAAA==".to_string()));
    components.push(Component::error(4160));
    components.push(Component::error(4161));

    // Common values sent with the request
    // "4128"
    // "4130"
    // "4112"
    // "4113"
    // "4098"
    // "4099"
    // "4100"
    // "4101"
    // "4102"
    // "4097"
    // "8195"
    // "8196"
    // "8197"
    // "4144"
    // "4145"
    // "4160"
    // "4161"
   

    components
}
