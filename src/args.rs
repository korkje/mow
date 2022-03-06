use clap::{ self, Parser, Subcommand };
use crate::lib::color::{ self, Color};
use crate::lib::range::in_range;

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
pub struct Args {
    #[clap(subcommand)]
    pub kind: Kind,
}

#[derive(Subcommand)]
pub enum Kind {
    /// Retrieve information about the device
    #[clap(subcommand)]
    Report(Report),

    /// Change the device's various settings
    #[clap(subcommand)]
    Config(Config),

    #[clap(hide = true)]
    Hex {
        #[clap(
            required = true,
            min_values = 1, max_values = 3,
            parse(try_from_str = color::parse_hex)
        )]
        colors: Vec<Color>
    },
}

#[derive(Subcommand)]
pub enum Report {
    /// Battery percentage (if available)
    Battery,

    /// Device firmware version
    Firmware,
}

#[derive(Subcommand)]
pub enum Config {
    /// Active profile by id
    Profile {
        #[clap(possible_values(["1", "2", "3"]))]
        id: u8
    },

    /// LED Effect
    LEDEffect {
        // Profile id (1-3)
        #[clap(
            short, long,
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
    DPIColors,

    /// Lift-off distance in mm
    LiftOff {
        #[clap(possible_values(["1", "2"]))]
        mm: u8,
    },

    /// Polling rate in ms
    PollingRate {
        #[clap(possible_values(["1", "2", "4", "8"]))]
        ms: u8,
    },

    /// Debounce in ms (0-16)
    Debounce {
        // Profile id (1-3)
        #[clap(
            short, long,
            help = "[default: 1]",
            possible_values(["1", "2", "3"])
        )]
        profile: Option<u8>,

        #[clap(validator = in_range(&(0..=16)))]
        ms: u8,
    },

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
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option <u8>,
    },

    /// Cycle through all colors
    Cycle {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option<u8>,
    },

    /// Pulse on/off through given colors
    Pulse {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option<u8>,

        /// From 2 to 6 colors in hex format
        #[clap(
            required = true,
            min_values = 2, max_values = 6,
            parse(try_from_str = color::parse_hex)
        )]
        colors: Vec<Color>
    },

    /// Solid color
    Solid {
        /// Color in hex format
        #[clap(parse(try_from_str = color::parse_hex))]
        color: Color
    },

    /// Pulse on/off one color
    PulseOne {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option<u8>,

        #[clap(parse(try_from_str = color::parse_hex))]
        color: Color,
    },

    /// Glorious, but colors don't "move"
    Tail {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option<u8>,
    },

    /// Strobe-like effect
    Rave {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option<u8>,

        /// 1 or 2 colors in hex format
        #[clap(
            required = true,
            min_values = 1, max_values = 2,
            parse(try_from_str = color::parse_hex)
        )]
        colors: Vec<Color>
    },

    /// Glorious, but more circus
    Wave {
        /// Effect rate, 0-100
        #[clap(
            short, long,
            validator = in_range(&(0..=100))
        )]
        rate: Option<u8>,
    },

    /// No effect, LED off
    Off, 
}