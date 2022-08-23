# swaybar\_info

[![swaybar\_info badge](https://img.shields.io/crates/v/swaybar_info)](https://crates.io/crates/swaybar_info)

[![swaybar\_info preview image](https://github.com/Stephen-Seo/swaybar_info/raw/master/pictures/swaybar_screenshot_00.png)](https://github.com/Stephen-Seo/swaybar_info/raw/master/pictures/swaybar_screenshot_00.png)

## About

swaybar\_info is a program to be utilized by swaybar that is used by the [Sway
tiling Wayland compositor](https://swaywm.org).

## Changes in What Version

[See the Changelog.md for details.](https://github.com/Stephen-Seo/swaybar_info/blob/master/Changelog.md)

## Help Text

    Usage:
      -h | --help                                      Prints help
      --netdev=<device_name>                           Check network traffic on specified device
      --interval-sec=<seconds>                         Output at intervals of <seconds> (default 5)
      --acpi-builtin                                   Use "acpi -b" built-in fetching (battery info, with color)
      --regex-cmd=<cmd>[SPLIT]<args...>[SPLIT]<regex>  Use an output of a command as a metric
      --time-format=<date format string>               Set the format string for the date

## Usage

    # build the "release" build of the program
    cargo build --release
    # put the "release" build somewhere to be used by swaybar
    cp ./target/release/swaybar_info ~/.config/sway/

    # Alternatively, get it from crates.io/
    cargo install swaybar_info
    # The `swaybar_info` binary should be placed in $HOME/.cargo/bin/

Put the following in your `~/.config/sway/config` (assuming the binary is at
`$HOME/.config/sway/swaybar_info`):

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


        # The following uses 24 hour time
        #status_command $HOME/.config/sway/swaybar_info --time-format="%Y-%m-%d %R:%S"
    }

## Advanced Usage of `--regex-cmd`

If the regex provided to `swaybar_info` has two captures, the first capture will
be used as the text to be displayed, and the second capture will be expected to
be the color string (such as FFFFFF for white, or 44FF44 for a lighter green).

For example, if the script invoked with `--regex-cmd` has output like the
following:

    MPD Title | MPD Album | playingCOLORSPLIT44FF44

That sometimes becomes:

    MPD is not running

Then this text can be parsed with a regex like:

    status_command $HOME/.config/sway/swaybar_info \
    '--regex-cmd=$HOME/scripts/mpc/mpcCommand.sh[SPLIT]simple[SPLIT]^\(.\*?\)\(?:\(?:COLORSPLIT\)\([A-F0-9]{6}\)\)?$'


Note that some characters like `*` or `(` had to be escaped because they are
being passed verbatim to a shell.

## Dependencies

Uses [`serde_json`](https://crates.io/crates/serde_json),
[`serde`](https://crates.io/crates/serde),
[`chrono`](https://crates.io/crates/chrono),
and [`regex`](https://crates.io/crates/regex).
