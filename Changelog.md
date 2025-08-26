# Changelog

## Upcoming Changes

## 0.1.19

`cargo update` (updating dependencies).

## 0.1.18

Update dependencies (`cargo update`). Update `chrono` dependency.

## 0.1.17

Update dependencies (`cargo update`). Update `regex` dependency.

## 0.1.16

Update dependencies (`cargo update`).

## 0.1.15

Update dependencies (`cargo update` and bump `regex` version).

## 0.1.14

Bumped dependency `regex` to version `1.9`.

## 0.1.13

Fix to workaround when `acpi` output contains a `0%` line with "unavailable".
When such a line is encountered, it is ignored.

## 0.1.12

Some refactoring of the code related to colorizing the netgraph.

## 0.1.11

Use pango markup to colorize the netgraph, making it look cleaner.

## 0.1.10

Colorize the netgraph based on if download or upload is greater.  
Download is red, upload is green, and same amount is yellow.

## 0.1.9

Impl. changing the size of the net-graph (default 10).

## 0.1.8

Impl. showing the maximum value in a dynamic netgraph.

## 0.1.7

When swaybar\_info starts, it no longer displays the traffic amount leading up
to the start of the program (it now starts at 0).

Increase netdev traffic minimum text width to 11 (was 10).

Updated README.md and help text based on new dynamic-netgraph-display feature.

Impl. dynamic netgraph display (the netgraph will scale the graph based on the
maximum traffic in an interval dynamically).

## 0.1.6

Minor refactoring of how the netgraph string is handled.

Refactoring of handling of Option types wrapping primitive values.

Round values when determining netgraph results instead of truncating to an
integer.

## 0.1.5

Implemented `--netdev_width=<width>` which sets the minimum width of the netdev
byte/KiB/MiB text displays.

Implemented `--netgraph_max_bytes=<bytes>` which displays a graph in text using
Unicode "Block Elements" symbols. The `<bytes>` argument determines the maximum
amount of bytes that will determine which block-character is printed on the
interval. The graph is always 10 characters wide, and the right side is the
most-recent side. Note that this always checks against the maximum of either
download or upload rates. For example, if `<bytes>` is set to 1024, and 128
bytes were downloaded and 512 bytes were uploaded in an interval, the "Lower
Half Block" Unicode symbol will be emitted (exactly half).

SwaybarObject was changed to use an `Option<String>` instead of an `Option<u16>`
for `min_width`.

## 0.1.4

Implemented advanced usage of `--regex-cmd=...` such that output text and output
text color can be specified with regex captures. The first capture is the output
text and the second capture is its color.

The README.md explains how to use this feature.

## 0.1.3

Fix failing to fetch netdev info when /proc/net/dev device starts with whitespace.

## 0.1.2

Fix README.md (erroneously used `--date-format` instead of `--time-format`).

## 0.1.1

Implement setting the time format string with the `--time-format=<time format string>` arg.

## 0.1.0

Implementation of info output for use by swaybar.

By default, displays the date, load-avg, and memory usage.

Network stats can be shown with the `--netdev=<device>` arg.

Arbitrary output from a command can be shown with the `--regex-cmd=<cmd>[SPLIT]<args...>[SPLIT]<regex>` arg.

The update interval in seconds can be set with the `--interval-sec=<seconds>` arg.

Battery stats display can be enabled with the `--acpi-builtin` arg.
