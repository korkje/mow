pub mod report;
pub mod config;
pub mod hex;
pub mod none;

use clap::{ self, Parser, Subcommand };
use hidapi::HidApi;
use none::None;
use hex::Hex;

#[derive(Parser)]
#[clap(
    // From Cargo.toml
    author, version, about,

    // Don't need a subcommand when we have flags
    disable_help_subcommand = true,

    // Same reported version for subcommands
    propagate_version = true,

    // Prevent automatic uppercase short version
    mut_arg("version", |x| { x.short('v') }),
)]
struct Args {
    #[clap(subcommand)]
    kind: Kind,
}

#[derive(Subcommand)]
enum Kind {
    /// Retrieve information about the device
    #[clap(subcommand)]
    Report(Report),

    /// Change the device's various settings
    #[clap(subcommand)]
    Config(Config),

    #[clap(hide = true)]
    Hex {
        #[clap(
            min_values = 1,
            max_values = 3,
            parse(try_from_str = hex::parse)
        )]
        colors: Vec<Hex>
    },
}

#[derive(Subcommand)]
enum Report {
    /// Battery percentage (if available)
    Battery,

    /// Device firmware version
    Firmware,
}

#[derive(Subcommand)]
enum Config {
    /// Active profile by id
    Profile {
        #[clap(possible_values(["1", "2", "3"]))]
        id: u8
    },

    /// LED Effect
    LEDEffect {
        // Profile id (1-3)
        #[clap(
            short,
            long,
            help = "[default: 1]",
            possible_values(["1", "2", "3"])
        )]
        profile: Option<u8>,

        #[clap(subcommand)]
        effect: Effect
    },

    /// LED brightness value[s] (0-255)
    LEDBrightness {
        wired: u8,

        #[clap(help = "[default: <WIRED>]")]
        wireless: Option<u8>,
    },

    /// Sleep delay in minutes [and seconds]
    Sleep {
        minutes: u8,

        #[clap(help = "[default: 0]")]
        seconds: Option<u8>,
    },

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

#[derive(Subcommand)]
pub enum Effect {
    /// Name says it all
    Glorious {
        /// Effect rate, 0-100
        #[clap(short, long)]
        rate: Option <u8>,
    },

    /// Cycle through all colors
    Cycle {
        /// Effect rate, 0-100
        #[clap(short, long)]
        rate: Option<u8>,
    },

    /// Pulse on/off through given colors
    Pulse {
        /// Effect rate, 0-100
        #[clap(short, long)]
        rate: Option<u8>,

        /// From 2 to 6 colors in hex format
        #[clap(
            min_values = 1,
            max_values = 6,
            parse(try_from_str = hex::parse)
        )]
        colors: Vec<Hex>
    },

    /// Solid color
    Solid {
        /// Color in hex format
        #[clap(parse(try_from_str = hex::parse))]
        color: Hex
    },

    /// Glorious, but colors don't "move"
    Tail {
        /// Effect rate, 0-100
        #[clap(short, long)]
        rate: Option<u8>,
    },

    /// Strobe-like effect
    Rave {
        /// Effect rate, 0-100
        #[clap(short, long)]
        rate: Option<u8>,

        /// 1 or 2 colors in hex format
        #[clap(
            min_values = 1,
            max_values = 2,
            parse(try_from_str = hex::parse)
        )]
        colors: Vec<Hex>
    },

    /// Glorious, but more circus
    Wave {
        /// Effect rate, 0-100
        #[clap(short, long)]
        rate: Option<u8>,
    },
    
    /// No effect, LED off
    Off, 
}

fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    
    // Interface with platform specific 'hidapi'
    let hid_api = HidApi::new()
        .unwrap();

    // Try to find a matching device
    let device_info = hid_api
        .device_list()
        .find(|d| {
            // Configuration interface
            d.interface_number() == 2 &&

            // Glorious' vendor id
            d.vendor_id() == 0x258a && (
                // Product id (wireless)
                d.product_id() == 0x2022 ||

                // Product id (wired)
                d.product_id() == 0x2011
            )
        })
        .none("No matching device found!");

    // Product id indicates whether wired
    let wired = device_info.product_id() == 0x2011;

    // Connect to the device
    let hid_device = hid_api
        .open_path(device_info.path())
        .unwrap(); // TODO: error handling

    // Act upon command line arguments
    match args.kind {
        // mow report
        Kind::Report(report) => match report {
            // mow report battery
            Report::Battery =>
                report::battery::get(&hid_device),

            // mow report firmware
            Report::Firmware =>
                report::firmware::get(&hid_device, wired),
        },

        // mow config
        Kind::Config(config) => match config {
            // mow config profile <ID>
            Config::Profile { id } =>
                config::profile::set(&hid_device, id),

            // mow config sleep <MINUTES> [SECONDS]
            Config::Sleep { minutes, seconds} =>
                config::sleep::set(
                    &hid_device,
                    minutes,
                    seconds
                ),

            // mow config led-brightness <WIRED> [WIRELESS]
            Config::LEDBrightness { wired, wireless } =>
                config::led_brightness::set(
                    &hid_device,
                    wired,
                    wireless
                ),

            // mow config led-brightness <EFFECT> ...
            Config::LEDEffect { profile, effect }  =>
                config::led_effect::set(&hid_device, profile, effect),

            _ => println!("(not implemented)"),
        },

        // mow hex <HEX>
        Kind::Hex { colors } => hex::print(colors),
    }
}
