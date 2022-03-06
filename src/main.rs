pub mod args;
pub mod lib;
pub mod config;
pub mod report;

use clap::Parser;
use hidapi::HidApi;

use lib::color;
use lib::none::None;
use args::{ Args, Kind, Report, Config };

fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    
    // Interface with platform specific 'hidapi'
    let hid_api = HidApi::new().unwrap();

    // Try to find a matching device
    let device_info = hid_api
        .device_list()
        .find(|d| {
            // Configuration interface
            d.interface_number() == 2 &&

            // Glorious' vendor id
            d.vendor_id() == 0x258a &&
            [0x2022, 0x2011].contains(&d.product_id())
        })
        .none("No matching device found!");

    // Product id indicates whether wired
    let wired = device_info.product_id() == 0x2011;

    // Connect to the device
    let device = device_info.open_device(&hid_api).unwrap();

    // Act upon command line arguments
    match args.kind {
        // mow report
        Kind::Report(report) => match report {
            // mow report battery
            Report::Battery =>
                report::battery::get(&device),

            // mow report firmware
            Report::Firmware =>
                report::firmware::get(&device, wired),
        },

        // mow config
        Kind::Config(config) => match config {
            // mow config profile <ID>
            Config::Profile { id } =>
                config::profile::set(&device, id),

            // mow config sleep <MINUTES> [SECONDS]
            Config::Sleep { minutes, seconds} =>
                config::sleep::set(&device, minutes, seconds),

            // mow config led-brightness <WIRED> [WIRELESS]
            Config::LEDBrightness { wired, wireless } =>
                config::led_brightness::set(&device, wired, wireless),

            // mow config led-effect <EFFECT> ...
            Config::LEDEffect { profile, effect }  =>
                config::led_effect::set(&device, profile, effect),

            _ => println!("(not implemented)"),
        },

        // mow hex <HEX>...
        Kind::Hex { colors } => color::print(colors),
    }
}
