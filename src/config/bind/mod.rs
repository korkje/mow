pub mod key;
pub mod mouse;

use colored::Colorize;
use hidapi::HidDevice;
use crate::args::{ Button, Binding };
use std::{ thread, time::Duration };

const PROFILE_DEFAULT: u8 = 1;

pub fn set(device: &HidDevice, profile: Option<u8>, button: Button, binding: Binding) {
    let mut bfr = [0u8; 65];
    let profile_id = profile.unwrap_or(PROFILE_DEFAULT);

    bfr[3] = 0x02;
    bfr[4] = 0x09;
    bfr[5] = 0x03;
    bfr[7] = profile_id;
    bfr[8] = id_from_btn(button);
    
    match binding {
        Binding::Key { kind } =>
            key::set(&mut bfr[10..], kind),

        Binding::Mouse(kind) =>
            mouse::set(&mut bfr[10..], kind),

        _ => println!("(not implemented)"),
    }

    device.send_feature_report(&mut bfr).unwrap();
    set_and_check(device, &mut bfr, 0, false);
}

pub fn set_and_check(device: &HidDevice, _bfr: &mut [u8], depth: u8, waiting: bool) {
    if depth < 3 {
        if waiting {
            thread::sleep(Duration::from_millis(100));
            set_and_check(device, _bfr, depth + 1, true);
        }
        else {
            thread::sleep(Duration::from_millis(100));
            let mut bfr = [0u8; 55];
            device.get_feature_report(&mut bfr).unwrap();
            thread::sleep(Duration::from_millis(40));

            match bfr[0] {
                0xA2 => {
                    device.send_feature_report(_bfr).unwrap();
                    set_and_check(device, _bfr, depth + 1, false)
                },
                0xA0 => set_and_check(device, _bfr, depth + 1, false),
                0xA4 => set_and_check(device, _bfr, depth + 1, true),
                _ => return
            }
        }
    }
    else {
        println!("{}: {}", "Error".bold().red(), "Failed setting key binding!");
    }
}

fn id_from_btn(button: Button) -> u8 {
    match button {
        Button::Left => 1,
        Button::Scroll => 3,
        Button::Right => 2,
        Button::Forward => 5,
        Button::Back => 4,
        Button::DPI => 20,
        Button::ScrollUp => 16,
        Button::ScrollDown => 17,
    }
}