use hidapi::HidDevice;

pub fn set(hid_device: &HidDevice, id: u8) {
    let mut write_bfr = [0u8; 65];

    write_bfr[3] = 0x02;
    write_bfr[4] = 0x01;
    write_bfr[6] = 0x05;
    write_bfr[7] = id;

    hid_device
        .send_feature_report(&write_bfr)
        .unwrap(); // TODO: error handling
}