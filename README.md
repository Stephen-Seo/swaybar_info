# swaybar\_info

[![swaybar\_info badge](https://img.shields.io/crates/v/swaybar_info)](https://crates.io/crates/swaybar_info)

[![swaybar\_info preview image](https://github.com/Stephen-Seo/swaybar_info/raw/master/pictures/swaybar_screenshot_00.png)](https://github.com/Stephen-Seo/swaybar_info/raw/master/pictures/swaybar_screenshot_00.png)

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

        # A "built-in" for "acpi -b" is available, and can be activated with the
        # --acpi-builtin flag:

        #status_command $HOME/.config/sway/swaybar_info --acpi-builtin

        # One can use the "--regex-cmd=<cmd>[SPLIT]<args...>[SPLIT]<regex>" option like so:

        #status_command $HOME/.config/sway/swaybar_info --regex-cmd="acpi[SPLIT]-b[SPLIT][0-9]+%.*"

        # This example gets battery info into the bar.
        # Multiple args should be separated with "[SPLIT]".
        # Note that the <args...> portion is optional.
    }

## Dependencies

Uses [`serde_json`](https://crates.io/crates/serde_json),
[`serde`](https://crates.io/crates/serde),
[`chrono`](https://crates.io/crates/chrono),
and [`regex`](https://crates.io/crates/regex).
