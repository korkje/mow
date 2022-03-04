use hidapi::HidDevice;
use std::{ thread, time::Duration };

pub fn get(hid_device: &HidDevice, wired: bool) {
    let mut write_bfr = [0u8; 65];

    if wired {
        write_bfr[3];
    }
    write_bfr[4] = 0x03;
    write_bfr[6] = 0x81;

    hid_device
        .send_feature_report(&write_bfr)
        .unwrap(); // TODO: error handling

    thread::sleep(Duration::from_millis(50));

    let mut read_bfr = [0u8; 65];

    hid_device
        .get_feature_report(&mut read_bfr)
        .unwrap(); // TODO: error handling

    println!(
        "{}.{}.{}.{}",
        read_bfr[7],
        read_bfr[8],
        read_bfr[9],
        read_bfr[10],
    );
}