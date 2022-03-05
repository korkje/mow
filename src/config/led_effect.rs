use hidapi::HidDevice;
use crate::Effect;

const PROFILE: u8 = 1;
const RATE: u8 = 40;

pub fn set(hid_device: &HidDevice, profile: Option<u8>, effect: Effect) {
    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(PROFILE);

    bfr[3] = 0x02;
    bfr[5] = 0x02;
    bfr[7] = profile_id;
    bfr[8] = 0xFF;
    bfr[11] = (105 - RATE) / 5;


    match effect {
        Effect::Glorious { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x01;
            bfr[11] = (105 - rate.unwrap_or(RATE)) / 5;
        },

        Effect::Cycle { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x02;
            bfr[11] = (105 - rate.unwrap_or(RATE)) / 5;
            bfr[12] = 0xFF;
        },

        Effect::Pulse { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x03;
            bfr[11] = (105 - rate.unwrap_or(RATE)) / 5;
            
            for i in 0..6 {
                if i >= colors.len() {
                    bfr[12 + 3 * i + 0] = 0x00;
                    bfr[12 + 3 * i + 1] = 0x00;
                    bfr[12 + 3 * i + 2] = 0x00;
                }
                else {
                    bfr[12 + 3 * i + 0] = colors[i].red;
                    bfr[12 + 3 * i + 1] = colors[i].green;
                    bfr[12 + 3 * i + 2] = colors[i].blue;
                }
            }
        },

        Effect::Solid { color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x04;

            bfr[12 + 0] = color.red;
            bfr[12 + 1] = color.green;
            bfr[12 + 2] = color.blue;
        },

        Effect::Tail { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x06;
            bfr[11] = (105 - rate.unwrap_or(RATE)) / 5;
        },

        Effect::Rave { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x07;
            bfr[11] = (105 - rate.unwrap_or(RATE)) * 2;
            
            for i in 0..2 {
                if i >= colors.len() {
                    bfr[12 + 3 * i + 0] = 0x00;
                    bfr[12 + 3 * i + 1] = 0x00;
                    bfr[12 + 3 * i + 2] = 0x00;
                }
                else {
                    bfr[12 + 3 * i + 0] = colors[i].red;
                    bfr[12 + 3 * i + 1] = colors[i].green;
                    bfr[12 + 3 * i + 2] = colors[i].blue;
                }
            }
        },

        Effect::Wave { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x08;
            bfr[11] = (105 - rate.unwrap_or(RATE)) * 2;
        },

        Effect::Off => {
            bfr[4] = 0x05;
            bfr[9] = 0x00;
        },
    }

    hid_device
        .send_feature_report(&bfr)
        .unwrap();
}