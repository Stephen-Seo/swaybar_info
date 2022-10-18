# Changelog

## Upcoming Changes

Minor refactoring of how the netgraph string is handled.

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
