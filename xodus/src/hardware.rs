// Hardware probing utilities

use crate::models::devicecredential::Component;


pub fn probe_provision_components() -> Vec<Component> {
    let mut components = Vec::with_capacity(16);

    components.push(Component::new(4131, mac_address));
    components.push(Component::new(4112, disks));
    components.push(Component::new(4113, disks_serial));
    components.push(Component::new(4098, board_manufacturer));
    components.push(Component::new(4097, board_version));
    components.push(Component::new(4099, pc_model));
    components.push(Component::new(4100, smbios_board_version));
    components.push(Component::error(4101));
    components.push(Component::new(4102, smbios_uuid));
    components.push(Component::new(8196, clepv2));
    components.push(Component::new(8197, clepv4));
    components.push(Component::new(8195, unknown48));
    components.push(Component::new(4144, unknown32));
    components.push(Component::new(4145, chassis_type));
    components.push(Component::error(4160));
    components.push(Component::error(4161));


    components
}