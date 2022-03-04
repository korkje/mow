use hidapi::HidDevice;
use std::{ thread, time::{ Duration } };

pub fn get(hid_device: &HidDevice) {
    let mut bfr_w = [0u8; 65];

    bfr_w[3] = 0x02;
    bfr_w[4] = 0x02;
    bfr_w[6] = 0x83;

    hid_device
        .send_feature_report(&bfr_w)
        .unwrap(); // TODO: error handling

    thread::sleep(Duration::from_millis(50));

    let mut bfr_r = [0u8; 65];

    hid_device
        .get_feature_report(&mut bfr_r)
        .unwrap(); // TODO: error handling

    let mut battery_percentage = bfr_r[8];

    if battery_percentage == 0 {
        battery_percentage = 1;
    }

    let mut status = [0xA1, 0xA4, 0xA2, 0xA0, 0xA3]
        .iter()
        .position(|&s| { s == bfr_r[1] })
        .unwrap(); // TODO: error handling

    if bfr_r[6] != 0x83 {
        status = 2;
    }

    let is_sleeping = status == 1;

    if is_sleeping {
        println!("zzz...")
    }
    else {
        println!("{}%", battery_percentage);
    }
}