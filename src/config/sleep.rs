use hidapi::HidDevice;

pub fn set(hid_device: &HidDevice, m: u8, s: u8) {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x02;
    bfr[6] = 0x07;

    let s_total = (m as u16) * 60 + (s as u16);

    if m > 0 || s > 0 {
        let [first, second] = s_total.to_be_bytes();
        bfr[7] = first;
        bfr[8] = second;
    }
    else {
        bfr[7] = 0xFF;
        bfr[8] = 0xFF;
    }


    hid_device
        .send_feature_report(&bfr)
        .unwrap(); // TODO: error handling
}