use crate::args::MouseFn;

pub fn set(bfr: &mut [u8], kind: MouseFn) {
    let mouse_fn = id_from_fn(kind);

    bfr[0] = 0x01;
    bfr[1] = 0x01;
    bfr[2] = mouse_fn;

    if mouse_fn == 12 {
        bfr[0] = 12;
        bfr[2] = 1;
    }
    else if mouse_fn == 24 {
        bfr[0] = 8;
        bfr[2] = 4;
    }
    else if mouse_fn == 25 {
        bfr[0] = 8;
        bfr[2] = 3;
    }
}

fn id_from_fn(button: MouseFn) -> u8 {
    match button {
        MouseFn::Left => 1,
        MouseFn::Scroll => 3,
        MouseFn::Right => 2,
        MouseFn::Forward => 5,
        MouseFn::Back => 4,
        MouseFn::DPI => 20,
        MouseFn::ScrollUp => 16,
        MouseFn::ScrollDown => 17,
        MouseFn::ProfileCycleUp => 24,
        MouseFn::ProfileCycleDown => 25,
        MouseFn::BatteryStatus => 12,
    }
}