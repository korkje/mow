use hidapi::HidDevice;
use std::{ thread, time::{ Duration } };

pub fn get(device: &HidDevice) {
    let mut bfr_w = [0u8; 65];

    bfr_w[3] = 0x02;
    bfr_w[4] = 0x02;
    bfr_w[6] = 0x83;

    device.send_feature_report(&bfr_w).unwrap();

    thread::sleep(Duration::from_millis(50));

    let mut read_bfr = [0u8; 65];

    device.get_feature_report(&mut read_bfr).unwrap();

    let mut battery_percentage = read_bfr[8];

    if battery_percentage == 0 {
        battery_percentage = 1;
    }

    let mut status = [0xA1, 0xA4, 0xA2, 0xA0, 0xA3]
        .iter().position(|&s| { s == read_bfr[1] }).unwrap();

    if read_bfr[6] != 0x83 {
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