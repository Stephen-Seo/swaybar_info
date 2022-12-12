# swaybar\_info

[![swaybar\_info crates.io version badge](https://img.shields.io/crates/v/swaybar_info)](https://crates.io/crates/swaybar_info)
[![swaybar\_info license badge](https://img.shields.io/github/license/Stephen-Seo/swaybar_info)](https://choosealicense.com/licenses/mit/)

[![swaybar\_info preview image](https://github.com/Stephen-Seo/swaybar_info/raw/master/pictures/swaybar_screenshot_00.png)](https://github.com/Stephen-Seo/swaybar_info/raw/master/pictures/swaybar_screenshot_00.png)

## About

swaybar\_info is a program to be utilized by swaybar that is used by the [Sway
tiling Wayland compositor](https://swaywm.org).

## Changes in What Version

[See the Changelog.md for details.](https://github.com/Stephen-Seo/swaybar_info/blob/master/Changelog.md)

## Help Text

    Usage:
      -h | --help                                       Prints help
      --netdev=<device_name>                            Check network traffic on specified device
      --netdev_width=<width>                            Sets the min-width of the netdev output (default 11)
      --netgraph_max_bytes=<bytes>                      Enable "graph" output when polling network traffic
                                                          (Set to "dynamic" instead of a byte count for dynamic sizing)
      --netgraph_dyn_display                            Enable showing the current maximum value in the graph
      --interval-sec=<seconds>                          Output at intervals of <seconds> (default 5)
      --acpi-builtin                                    Use "acpi -b" built-in fetching (battery info, with color)
      --regex-cmd=<cmd>[SPLIT]<args...>[SPLIT]<regex>   Use an output of a command as a metric
      --time-format=<date format string>                Set the format string for the date

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
    '--regex-cmd=$HOME/scripts/mpc/mpcCommand.sh[SPLIT]simple[SPLIT]^\(.\*?\)\(?:COLORSPLIT\([A-F0-9]{6}\)\)?$'

Note that some characters like `*` or `(` had to be escaped because they are
being passed verbatim to a shell.

If only one capture is used in the regex string, then that capture will be used
for the output text, and the color will be left unspecified (usually defaulting
to white).

For a reference of what kind of regex is supported,
[see this page](https://docs.rs/regex/1.6.0/regex/index.html#syntax).

## Net graph

The `--netgraph_max_bytes=<bytes>` arg enables a 10-character-wide text graph
showing a history of network traffic. The algorithm checks the larger of bytes
sent or received in an interval and compares it to `<bytes>`. If it is greater,
then the graph character will be a "full block" character. If it is less, then
the graph character will be something in between out of 9 possible characters (a
space, and [8 unicode block
characters](https://en.wikipedia.org/wiki/Block_Elements)). Thus, this outputs
a history graph of network traffic. A sane value for `<bytes>` can be 1048576,
which is 1 MiB.

Specify "dynamic" instead of a bytecount (such as
`--netgraph_max_bytes=dynamic`) to have the graph dynamically resize based on
the maximum amount of bytes transferred in an interval.

When dynamic netgraph is used, swaybar\_info can display the maximum value in
the netgraph. Use the `--netgraph_dyn_display` option to enable this. (This
only works when dynamic netgraph is enabled with
`--netgraph_max_bytes=dynamic`.)

## Dependencies

Uses [`serde_json`](https://crates.io/crates/serde_json),
[`serde`](https://crates.io/crates/serde),
[`chrono`](https://crates.io/crates/chrono),
and [`regex`](https://crates.io/crates/regex).
