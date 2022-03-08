use hidapi::HidDevice;
use std::{ thread, time::{ Duration } };

pub fn get(device: &HidDevice, wired: bool) {
    let mut bfr_w = [0u8; 65];

    bfr_w[3] = 0x02;
    bfr_w[4] = 0x02;
    bfr_w[6] = 0x83;

    device.send_feature_report(&bfr_w).unwrap();

    thread::sleep(Duration::from_millis(50));

    let mut bfr_r = [0u8; 65];

    device.get_feature_report(&mut bfr_r).unwrap();

    let mut battery_percentage = bfr_r[8];

    if battery_percentage == 0 {
        battery_percentage = 1;
    }

    let mut status = [0xA1, 0xA4, 0xA2, 0xA0, 0xA3]
        .iter().position(|&s| { s == bfr_r[1] }).unwrap();

    if bfr_r[6] != 0x83 {
        status = 2;
    }

    match (status, wired) {
        (0, false) => println!("{}%", battery_percentage),
        (0, true) => println!("(charging) {}%", battery_percentage),
        (1, _) => println!("zzz..."),
        (_, _) => {
            println!("(unknown status)");
            println!(
                "01 06 08\n{:0>2X} {:0>2X} {:0>2X}",
                bfr_r[1], bfr_r[6], bfr_r[8],
            );
        },
    }
}