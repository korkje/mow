# Model O Wireless (mow)

## Goal
The goal of this project is to reverse engineer the communication between Glorious Core and the Model O Wireless mouse, so the mouse can be used (feature complete) on all platforms.

Another goal is just (re-)learning Rust, so if any Rust afficionados come across this project, feel free to tell me which parts of the code suck the most.

## About
So far I've been using WireShark and USBPcap for packet sniffing. Also the sloppily packaged Glorious Core software (for Windows only, hence this project) has been great help, with it's easily unarchive'able `.asar` file containing the JavaScript source.

The CLI tool is written in Rust. I've tried to keep it somewhat readable, however my main focus at this point is working on functionality. Currently, the following is working:

### Reports
- [x] Battery percentage
- [x] Firmware version

### Configuration
- [x] LED
    - [x] Brightness
    - [x] Effects
        - [x] Glorious
        - [x] Seamless breathing
        - [x] Breathing
        - [x] Single color
        - [x] Breathing single color
        - [x] Tail
        - [x] Rave
        - [x] Wave
        - [x] Off
- [x] Active profile
- [x] Sleep delay
- [x] Lift-off distance
- [x] Polling rate
- [x] Debounce
- [x] DPI
    - [x] Active
    - [x] Stages
    - [x] Colors
- [x] Scroll inversion (NEW)
- [ ] Key binding
    - [x] Single key
        - [x] Scan code
        - [x] Key code
        - [x] Code
    - [ ] Keyboard function
    - [x] Mouse function
    - [ ] DPI modifier
    - [ ] Macro
    - [ ] Multimedia
    - [ ] Shortcuts
    - [ ] None

At some point, after this is done, I might work on creating a daemon that can listen for "commands" from the device. This will be needed to make the most advanced key bindings work, e.g. launching applications.

## Installation
It is a Rust (Cargo) project, so just run `cargo build --release` in the root folder, create a symlink somewhere in your `PATH`, and you should be good to go. The CLI has built in `--help`, so use that to understand usage.

### Arch
On Arch linux, you can install this from [AUR](https://aur.archlinux.org) with `paru -S mow-git`.

## Misc
With some minor alteration, my findings probably also apply to Model D as well as reduced size versions of both devices. But unless someone wants to buy me the extra devices, I will not extend support myself, at least for now.

I am working on a separate detailed writeup describing the protocol in it's entirety, will probably commit those docs into this repo sooner or later, but there's way too much "maybe this, maybe that" and otherwise speculative language in there still.

## Safety
Should you be so unlucky as to somehow brick your device (as I have done myself repeatedly while working on this project), there is the option of factory resetting by pressing down both left and right mouse buttons and the scroll wheel, and hold for five seconds while the device flashes green.

However, I do not have insight into the inner workings of this device, and can therefore NOT guarantee all faults will be recoverable. Use this at your own risk!