# Model O Wireless (mow)

## Goal
The goal of this project is to reverse engineer the communication between Glorious Core and the Model O Wireless mouse, so the mouse can be used (feature complete) on all platforms.

Another goal is just (re-)learning Rust, so if any Rust afficionados come across this project, feel free to tell me which parts of the code suck the most.

## About
So far I've been using WireShark and USBPcap for packet sniffing. Also the sloppily packaged Glorious Core software (for Windows only, hence this project) has been great help, with it's easily unarchive'able `.asar` file containing the JavaScript source.

The CLI tool is written in Rust. I've tried to keep it somewhat readable, however my main focus at this point is working on functionality. Currently, the following is working:

### Reports
- [x] Battery percentage
- [x] Battery percentage

### Configuration
- [x] LED brightness
- [x] LED Effects (all of them!)
- [x] Active profile
- [x] Sleep delay
- [ ] Key/macro related stuff
- [ ] DPI (active, stages, colors)
- [ ] Calibration (?)
- [ ] Polling rate
- [ ] Debounce

At some point, after this is done, I might work on creating a daemon that can listen for "commands" from the device. This will be needed to make the most advanced macros work, like launching applications and various other OS specific stuff.

## Installation
It is a Rust (Cargo) project, so just run `cargo build --release` in the root folder, create a symlink somewhere in your `PATH`, and you should be good to go. The CLI has built in `--help`, so use that to understand usage.

### Arch Linux

On Arch linux, you can install it from AUR:

``` bash
paru -S mow-git
```


## Misc
With some minor alteration, my findings probably also apply to Model D as well as reduced size versions of both devices. But unless someone wants to buy me the extra devices, I will not extend support myself, at least for now.

I am working on a separate detailed writeup describing the protocol in it's entirety, will probably commit those docs into this repo sooner or later, but there's way too much "maybe this, maybe that" and otherwise speculative language in there still.