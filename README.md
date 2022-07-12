# swaybar\_info

## About

swaybar\_info is a program to be utilized by swaybar that is used by the [Sway
tiling Wayland compositor](https://swaywm.org).

## Usage

    # build the "release" build of the program
    cargo build --release
    # put the "release" build somewhere to be used by swaybar
    cp ./target/release/swaybar_info ~/.config/sway/

Put the following in your `~/.config/sway/config`:

    bar {
        position top
        # Set --netdev=<device> such that <device> is the network device you
        # want to monitor. You can omit --netdev=<device>, but that will also
        # cause the program to omit network traffic stats.
        status_command $HOME/.config/sway/swaybar_info --netdev=enp7s0

        # One can use the "--regex-cmd=<cmd>,<args...>,<regex>" option like so:
        status_command $HOME/.config/sway/swaybar_info --regex-cmd="acpi,-b,[0-9]+%.*"
        # This example gets battery info into the bar.
    }

## Dependencies

Uses [`serde_json`](https://crates.io/crates/serde_json),
[`serde`](https://crates.io/crates/serde),
[`chrono`](https://crates.io/crates/chrono),
and [`regex`](https://crates.io/crates/regex).
