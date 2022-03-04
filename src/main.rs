pub mod report;
pub mod config;

use clap::{ self, Parser, Subcommand };
use hidapi::HidApi;

#[derive(Parser)]
#[clap(
    name = "Model O Wireless",
    author = "Ola Næss Kaldestad <mail@okal.no>",
    version = "0.1.0",
    disable_help_subcommand = true,
)]
struct Args {
    #[clap(subcommand)]
    kind: Kind,
}

#[derive(Subcommand)]
enum Kind {
    #[clap(subcommand)]
    Report(Report),
    #[clap(subcommand)]
    Config(Config),
}

#[derive(Subcommand)]
enum Report {
    Battery,
    Firmware,
}

#[derive(Subcommand)]
enum Config {
    Profile {
        #[clap(possible_values(["1", "2", "3"]))]
        id: u8
    },
    /// (not implemented)
    LEDEffect,
    /// (not implemented)
    LEDBrightness,
    /// (not implemented)
    Sleep,
    /// (not implemented)
    DPIStages,
    /// (not implemented)
    DPIStage,
    /// (not implemented)
    DPIColor,
    /// (not implemented)
    Calibration,
    /// (not implemented)
    PollingRate,
    /// (not implemented)
    Debounce,
    /// (not implemented)
    Macro,
    /// (not implemented)
    Key,
}

fn main() {
    let args = Args::parse();
    
    let hid_api = HidApi::new()
        .unwrap(); // TODO: error handling

    let device_info = hid_api
        .device_list()
        .find(|d| {
            d.interface_number() == 2 &&
            d.vendor_id() == 0x258a && (
                d.product_id() == 0x2022 ||
                d.product_id() == 0x2011
            )
        })
        .unwrap(); // TODO: error handling

    let wired = device_info.product_id() == 0x2011;

    let hid_device = hid_api
        .open_path(device_info.path())
        .unwrap(); // TODO: error handling

    match args.kind {
        Kind::Report(report) => match report {
            Report::Battery => report::battery::get(&hid_device),
            Report::Firmware => report::firmware::get(&hid_device, wired),
        },
        Kind::Config(config) => match config {
            Config::Profile { id } => config::profile::set(&hid_device, id),
            _ => println!("(not implemented)"),
        },
    }
}
